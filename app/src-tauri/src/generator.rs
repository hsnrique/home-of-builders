use serde::Deserialize;

use crate::catalog::Area;
use crate::engine::Engine;
use crate::prompt::{self, Brief, Lang};
use crate::workspace::Workspace;
use crate::{frontmatter, library};

pub const MAX_NEEDS_CHARS: usize = 4_000;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRequest {
    pub workspace: String,
    pub base_path: String,
    pub area_id: Option<String>,
    pub needs: String,
    pub lang: Lang,
    pub engine: Engine,
}

/// Base agent × area × needs × language → a new agent file, returned for review
/// (nothing is saved until the user accepts it).
pub fn generate(request: &GenerateRequest) -> Result<String, String> {
    let ws = Workspace::open(&request.workspace)?;
    let prompt = build_prompt(&ws, request)?;
    let raw = request.engine.run(prompt::SYSTEM_PROMPT, &prompt)?;
    frontmatter::extract_agent(&raw)
        .map(|agent| frontmatter::normalize(&agent))
        .ok_or_else(|| "error.invalidOutput".into())
}

fn build_prompt(ws: &Workspace, request: &GenerateRequest) -> Result<String, String> {
    if request.needs.chars().count() > MAX_NEEDS_CHARS {
        return Err("error.needsTooLong".into());
    }
    let areas = ws.areas()?;
    let area = selected_area(&areas, request.area_id.as_deref())?;
    if area.is_none() && request.needs.trim().is_empty() {
        return Err("error.specializationRequired".into());
    }
    Ok(prompt::build(&Brief {
        standard: &ws.standard()?,
        base_agent: &library::read(ws, &request.base_path)?,
        area,
        needs: &request.needs,
        lang: request.lang,
    }))
}

fn selected_area<'a>(areas: &'a [Area], id: Option<&str>) -> Result<Option<&'a Area>, String> {
    match id {
        None | Some("") => Ok(None),
        Some(id) => areas
            .iter()
            .find(|area| area.id == id)
            .map(Some)
            .ok_or_else(|| "error.areaNotFound".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::repo_root;

    fn request(area_id: Option<&str>, needs: &str) -> GenerateRequest {
        GenerateRequest {
            workspace: repo_root().unwrap().display().to_string(),
            base_path: "agents/core/traffic-manager.md".into(),
            area_id: area_id.map(String::from),
            needs: needs.into(),
            lang: Lang::Pt,
            engine: Engine::Claude,
        }
    }

    fn prompt_for(req: &GenerateRequest) -> Result<String, String> {
        build_prompt(&Workspace::open(&req.workspace).unwrap(), req)
    }

    #[test]
    fn builds_prompt_from_the_real_library() {
        let prompt = prompt_for(&request(Some("healthcare"), "Botox in Curitiba")).unwrap();
        assert!(prompt.contains("# Elite Traffic Manager"));
        assert!(prompt.contains("Clinics & healthcare"));
        assert!(prompt.contains("Botox in Curitiba"));
    }

    #[test]
    fn validates_the_request() {
        assert_eq!(
            prompt_for(&request(None, " ")).unwrap_err(),
            "error.specializationRequired"
        );
        assert_eq!(
            prompt_for(&request(Some("nope"), "")).unwrap_err(),
            "error.areaNotFound"
        );
        let long = "x".repeat(MAX_NEEDS_CHARS + 1);
        assert_eq!(
            prompt_for(&request(None, &long)).unwrap_err(),
            "error.needsTooLong"
        );
        let mut outside = request(Some("seo"), "");
        outside.base_path = "STANDARD.md".into();
        assert_eq!(prompt_for(&outside).unwrap_err(), "error.agentNotFound");
    }

    /// Calls the real CLIs: `cargo test -- --ignored generates_a_real_agent`.
    #[test]
    #[ignore]
    fn generates_a_real_agent_with_claude() {
        let agent = generate(&request(Some("healthcare"), "Dental implants, R$5k/month")).unwrap();
        println!("{agent}");
        assert!(frontmatter::parse(&agent).is_some());
    }

    #[test]
    #[ignore]
    fn generates_a_real_agent_with_codex() {
        let mut req = request(Some("healthcare"), "Dental implants, R$5k/month");
        req.engine = Engine::Codex;
        let agent = generate(&req).unwrap();
        println!("{agent}");
        assert!(frontmatter::parse(&agent).is_some());
    }
}
