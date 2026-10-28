use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BINARY: &str = if cfg!(windows) { "claude.exe" } else { "claude" };

/// GUI apps on macOS start with a minimal PATH, so the usual install folders are
/// checked after PATH.
pub fn locate() -> Option<PathBuf> {
    search_dirs()
        .into_iter()
        .map(|dir| dir.join(BINARY))
        .find(|candidate| candidate.is_file())
}

fn search_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        dirs.extend([home.join(".local/bin"), home.join(".claude/local"), home.join(".npm-global/bin")]);
    }
    dirs.extend([PathBuf::from("/opt/homebrew/bin"), PathBuf::from("/usr/local/bin")]);
    dirs
}

/// Runs one headless Claude Code turn on the user's own subscription.
/// `--safe-mode` keeps the user's CLAUDE.md, hooks and plugins out of the generation and
/// `--tools ""` means the model can only write text back; it never touches the disk.
// Known limit: no timeout; a hung CLI leaves the UI spinning. Add a kill timer if it happens.
pub fn run(binary: &Path, system_prompt: &str, prompt: &str) -> Result<String, String> {
    let mut child = Command::new(binary)
        .args(["-p", "--safe-mode", "--tools", "", "--no-session-persistence"])
        .args(["--output-format", "text", "--system-prompt", system_prompt])
        .current_dir(std::env::temp_dir())
        .env("PATH", child_path(binary))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Claude Code: {e}"))?;
    let mut stdin = child.stdin.take().expect("stdin is piped");
    stdin.write_all(prompt.as_bytes()).map_err(|e| e.to_string())?;
    drop(stdin);
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    Err(failure_message(&output.stdout, &output.stderr))
}

/// npm-installed CLIs need `node` on PATH, which GUI apps do not inherit.
fn child_path(binary: &Path) -> std::ffi::OsString {
    let mut dirs = search_dirs();
    if let Some(parent) = binary.parent() {
        dirs.insert(0, parent.to_path_buf());
    }
    std::env::join_paths(dirs).unwrap_or_default()
}

fn failure_message(stdout: &[u8], stderr: &[u8]) -> String {
    let text = [stderr, stdout]
        .iter()
        .map(|bytes| String::from_utf8_lossy(bytes).trim().to_string())
        .find(|text| !text.is_empty())
        .unwrap_or_else(|| "unknown error".into());
    format!("Claude Code: {}", text.chars().take(500).collect::<String>())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn fake_cli(tag: &str, script: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("fake-claude-{tag}-{}", std::process::id()));
        std::fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    #[test]
    fn pipes_prompt_through_stdin_and_passes_isolation_flags() {
        let cli = fake_cli("echo", r#"echo "$@"; cat"#);
        let output = run(&cli, "SYS", "the prompt").unwrap();
        assert!(output.contains("-p --safe-mode --tools  --no-session-persistence"));
        assert!(output.contains("--system-prompt SYS"));
        assert!(output.ends_with("the prompt"));
    }

    #[test]
    fn surfaces_cli_errors() {
        let cli = fake_cli("fail", "echo 'Please run /login' >&2; exit 1");
        assert_eq!(run(&cli, "", "").unwrap_err(), "Claude Code: Please run /login");
    }
}
