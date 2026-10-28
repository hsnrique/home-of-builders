use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{cli, codex_agent};

/// The coding agents Home of Builders generates with and installs into.
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

    pub fn locate(self) -> Option<PathBuf> {
        cli::locate(match self {
            Engine::Claude => "claude",
            Engine::Codex => "codex",
        })
    }

    /// One headless turn on the user's own subscription, kept away from their personal
    /// setup and unable to touch the disk: the model can only write text back.
    pub fn run(self, system_prompt: &str, prompt: &str) -> Result<String, String> {
        let binary = self.locate().ok_or(match self {
            Engine::Claude => "error.claudeNotFound",
            Engine::Codex => "error.codexNotFound",
        })?;
        self.run_with(&binary, system_prompt, prompt)
    }

    fn run_with(self, binary: &Path, system_prompt: &str, prompt: &str) -> Result<String, String> {
        match self {
            Engine::Claude => {
                let args = [CLAUDE_ARGS, &["--system-prompt", system_prompt]].concat();
                cli::run(binary, &args, prompt, "Claude Code")
            }
            // Codex has no system prompt flag, so the system prompt leads the stdin prompt.
            Engine::Codex => {
                let full_prompt = format!("{system_prompt}\n\n{prompt}");
                cli::run(binary, CODEX_ARGS, &full_prompt, "Codex")
            }
        }
    }
}

/// `--safe-mode` skips the user's CLAUDE.md, hooks and plugins; `--tools ""` removes every tool.
const CLAUDE_ARGS: &[&str] = &[
    "-p",
    "--safe-mode",
    "--tools",
    "",
    "--no-session-persistence",
    "--output-format",
    "text",
];

/// Skips the user's config.toml (model, MCP servers) and exec rules, and keeps the model in a
/// read-only sandbox. Codex still loads ~/.codex/AGENTS.md; the system prompt tells the
/// model to set those rules aside.
const CODEX_ARGS: &[&str] = &[
    "exec",
    "--ephemeral",
    "--skip-git-repo-check",
    "--ignore-user-config",
    "--ignore-rules",
    "--sandbox",
    "read-only",
    "--color",
    "never",
    "-",
];

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::cli::tests::fake_cli;

    #[test]
    fn claude_runs_isolated_with_the_system_prompt_as_a_flag() {
        let cli = fake_cli("claude-args", r#"echo "$@"; cat"#);
        let output = Engine::Claude.run_with(&cli, "SYS", "the prompt").unwrap();
        assert!(output.contains("-p --safe-mode --tools  --no-session-persistence"));
        assert!(output.contains("--system-prompt SYS\nthe prompt"));
    }

    #[test]
    fn codex_runs_isolated_with_the_system_prompt_first_on_stdin() {
        let cli = fake_cli("codex-args", r#"echo "$@"; cat"#);
        let output = Engine::Codex.run_with(&cli, "SYS", "the prompt").unwrap();
        assert!(output.starts_with("exec --ephemeral --skip-git-repo-check --ignore-user-config"));
        assert!(output.contains("--sandbox read-only"));
        assert!(output.ends_with(" -\nSYS\n\nthe prompt"));
    }
}
