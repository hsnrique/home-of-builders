# Security

## Reporting a vulnerability

Please report it privately through [GitHub Security Advisories](https://github.com/hsnrique/home-of-builders/security/advisories/new), not in a public issue.

Only the latest release receives fixes.

## How the app is built to stay safe

- **Local only.** The app itself makes no network requests. Generation runs your own `claude` CLI, which talks to Anthropic exactly as it does when you use it directly.
- **No tools for the model.** Generation uses `claude -p --tools "" --safe-mode`: the model can only return text, and your personal Claude Code configuration is not loaded.
- **No arbitrary file access.** The UI can only read agents that the backend found by scanning the library's `agents/` folder. Saved and installed files are named from a validated kebab-case name.
- **Text, never HTML.** Agent files and model output are rendered as text, under a strict Content-Security-Policy.
- **Pinned dependencies.** npm packages, crates and GitHub Actions are pinned to exact versions or commit SHAs.
