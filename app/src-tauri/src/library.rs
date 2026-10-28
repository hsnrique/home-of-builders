use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::frontmatter::{self, is_valid_name};
use crate::prompt::Lang;
use crate::workspace::Workspace;

pub const MAX_AGENT_BYTES: usize = 200_000;

#[derive(Debug, Clone, Serialize)]
pub struct AgentSummary {
    /// Relative to the workspace root, always with `/`. Doubles as the agent's id.
    pub path: String,
    pub name: String,
    pub description: String,
    /// Folder under `agents/`, e.g. `core` or `generated/pt`.
    pub group: String,
}

pub fn scan(ws: &Workspace) -> Vec<AgentSummary> {
    let mut files = Vec::new();
    collect_markdown(&ws.agents_dir(), 0, &mut files);
    let mut agents: Vec<_> = files.iter().filter_map(|f| summarize(ws, f)).collect();
    agents.sort_by(|a, b| (&a.group, &a.name).cmp(&(&b.group, &b.name)));
    agents
}

/// Only paths returned by `scan` resolve, so the UI can never reach outside `agents/`.
pub fn find(ws: &Workspace, path: &str) -> Result<AgentSummary, String> {
    scan(ws)
        .into_iter()
        .find(|agent| agent.path == path)
        .ok_or_else(|| "error.agentNotFound".into())
}

pub fn read(ws: &Workspace, path: &str) -> Result<String, String> {
    let agent = find(ws, path)?;
    fs::read_to_string(ws.root().join(&agent.path)).map_err(|e| e.to_string())
}

pub fn save(ws: &Workspace, lang: Lang, markdown: &str) -> Result<AgentSummary, String> {
    let name = validated_name(markdown)?;
    let dir = ws.generated_dir(lang);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let (path, final_name) = free_slot(&dir, &name);
    let content = frontmatter::rename(markdown.trim_end(), &final_name) + "\n";
    fs::write(&path, content).map_err(|e| e.to_string())?;
    summarize(ws, &path).ok_or_else(|| "error.invalidAgent".into())
}

pub fn install(
    ws: &Workspace,
    path: &str,
    target_dir: &Path,
    overwrite: bool,
) -> Result<String, String> {
    let agent = find(ws, path)?;
    let target = target_dir.join(format!("{}.md", agent.name));
    if target.exists() && !overwrite {
        return Err("error.installExists".into());
    }
    fs::create_dir_all(target_dir).map_err(|e| e.to_string())?;
    fs::copy(ws.root().join(&agent.path), &target).map_err(|e| e.to_string())?;
    Ok(target.display().to_string())
}

fn validated_name(markdown: &str) -> Result<String, String> {
    if markdown.len() > MAX_AGENT_BYTES {
        return Err("error.tooLarge".into());
    }
    frontmatter::parse(markdown)
        .map(|meta| meta.name)
        .filter(|name| is_valid_name(name))
        .ok_or_else(|| "error.invalidAgent".into())
}

/// Never overwrite a saved agent: `name`, then `name-2`, `name-3`…
fn free_slot(dir: &Path, name: &str) -> (PathBuf, String) {
    (1..)
        .map(|n| {
            if n == 1 {
                name.to_string()
            } else {
                format!("{name}-{n}")
            }
        })
        .map(|candidate| (dir.join(format!("{candidate}.md")), candidate))
        .find(|(path, _)| !path.exists())
        .expect("an unbounded range always yields a free slot")
}

fn summarize(ws: &Workspace, file: &Path) -> Option<AgentSummary> {
    let meta = frontmatter::parse(&fs::read_to_string(file).ok()?)?;
    if !is_valid_name(&meta.name) {
        return None;
    }
    let relative = file.strip_prefix(ws.root()).ok()?;
    let group = file.parent()?.strip_prefix(ws.agents_dir()).ok()?;
    Some(AgentSummary {
        path: slash_path(relative),
        name: meta.name,
        description: meta.description,
        group: slash_path(group),
    })
}

// Known limit: depth cap matches the deepest layout we use (agents/generated/<lang>/).
fn collect_markdown(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let (path, Ok(kind)) = (entry.path(), entry.file_type()) else {
            continue;
        };
        if kind.is_dir() && depth < 3 {
            collect_markdown(&path, depth + 1, out);
        } else if kind.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            out.push(path);
        }
    }
}

fn slash_path(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests;
