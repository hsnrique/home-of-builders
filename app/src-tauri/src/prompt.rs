use serde::{Deserialize, Serialize};

use crate::catalog::{Area, AreaKind};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    En,
    Pt,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Pt => "pt",
        }
    }

    fn instruction(self) -> &'static str {
        match self {
            Lang::En => "Write the entire agent in English.",
            Lang::Pt => "Write the description, headings and the entire body in Brazilian Portuguese (pt-BR), natural and idiomatic for Brazilian marketers. Keep the frontmatter keys (name, description, tools, model) and tool names in English. Keep `name` in kebab-case ASCII without accents. Keep metric acronyms and platform names as Brazilian marketers use them (CPA, ROAS, CTR, Meta Ads).",
        }
    }
}

pub const SYSTEM_PROMPT: &str = "You design elite Claude Code subagents for a marketing team. \
You receive the library's agent standard, a base agent that sets the seniority and quality bar, \
a specialization and a target language, and you write ONE new agent file. \
Output only the raw Markdown file, starting with the `---` frontmatter line: \
no preamble, no code fences, no commentary.";

pub struct Brief<'a> {
    pub standard: &'a str,
    pub base_agent: &'a str,
    pub area: Option<&'a Area>,
    pub needs: &'a str,
    pub lang: Lang,
}

pub fn build(brief: &Brief) -> String {
    format!(
        "<standard>\n{}\n</standard>\n\n<base_agent>\n{}\n</base_agent>\n\n\
         <specialization>\n{}\n</specialization>\n\n<language>\n{}\n</language>\n\n{TASK}",
        brief.standard.trim(),
        brief.base_agent.trim(),
        specialization(brief.area, brief.needs),
        brief.lang.instruction(),
    )
}

fn specialization(area: Option<&Area>, needs: &str) -> String {
    let area_line = match area {
        Some(a) => format!("Area ({}): {} — {}", kind_label(a.kind), a.label.en, a.hint),
        None => "Area: none selected — specialize from the specific needs alone.".to_string(),
    };
    let needs = match needs.trim() {
        "" => "none given — specialize from the area alone.",
        text => text,
    };
    format!("{area_line}\n\nSpecific needs from the user:\n{needs}")
}

fn kind_label(kind: AreaKind) -> &'static str {
    match kind {
        AreaKind::Niche => "niche",
        AreaKind::Channel => "channel",
    }
}

const TASK: &str = "Task:
1. Build a NEW agent that keeps the base agent's seniority, rigor and section structure, rebuilt from the ground up for this specialization: intake questions, playbooks, benchmarks, compliance and deliverables must be specific to it. Do not just rename the base agent.
2. Choose a precise kebab-case ASCII `name` (max 64 characters) that describes the specialization, e.g. `clinic-meta-ads-traffic-manager`.
3. Write the `description` so Claude knows exactly when to delegate to this agent, with two quoted trigger examples in the target language.
4. Keep `tools` least-privilege, starting from the base agent's tools. Keep `model: inherit`.
5. Follow the standard's 10 sections in order, 120-250 lines, grounded in the canonical books of the field where they apply.
Output only the file.";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Label;

    fn clinic() -> Area {
        Area {
            id: "healthcare".into(),
            kind: AreaKind::Niche,
            label: Label { en: "Clinics".into(), pt: "Clinics (pt)".into() },
            hint: "Council rules apply.".into(),
        }
    }

    #[test]
    fn prompt_carries_every_input() {
        let area = clinic();
        let prompt = build(&Brief {
            standard: "STD",
            base_agent: "BASE",
            area: Some(&area),
            needs: "Botox leads in Curitiba",
            lang: Lang::Pt,
        });
        for expected in ["STD", "BASE", "Area (niche): Clinics — Council rules apply.", "Botox leads in Curitiba", "pt-BR", "Task:"] {
            assert!(prompt.contains(expected), "missing {expected}");
        }
    }

    #[test]
    fn prompt_explains_missing_area_or_needs() {
        let no_area = build(&Brief { standard: "", base_agent: "", area: None, needs: "x", lang: Lang::En });
        assert!(no_area.contains("Area: none selected"));
        let area = clinic();
        let no_needs = build(&Brief { standard: "", base_agent: "", area: Some(&area), needs: "  ", lang: Lang::En });
        assert!(no_needs.contains("none given"));
        assert!(no_needs.contains("entire agent in English"));
    }
}
