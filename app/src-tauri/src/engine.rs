use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::codex_agent;

/// The coding agents Home of Builders installs into.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    Claude,
    Codex,
}

impl Engine {
    /// Where the engine looks for a user-level agent with this name.
    pub fn agent_file(self, home: &Path, name: &str) -> PathBuf {
        match self {
            Engine::Claude => home
                .join(".claude")
                .join("agents")
                .join(format!("{name}.md")),
            Engine::Codex => home
                .join(".codex")
                .join("agents")
                .join(format!("{name}.toml")),
        }
    }

    /// Library agents are Claude Code Markdown; Codex gets the same agent as TOML.
    pub fn render(self, markdown: &str) -> Option<String> {
        match self {
            Engine::Claude => Some(markdown.to_string()),
            Engine::Codex => codex_agent::from_markdown(markdown),
        }
    }
}
