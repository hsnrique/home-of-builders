use super::*;

fn agent(name: &str) -> String {
    format!(
        "---\nname: {name}\ndescription: Does {name} things.\nmodel: inherit\n---\n\n# {name}\n"
    )
}

fn temp_workspace(tag: &str) -> (PathBuf, Workspace) {
    let dir = std::env::temp_dir().join(format!("hob-library-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("agents/core")).unwrap();
    fs::write(dir.join("agents/core/copywriter.md"), agent("copywriter")).unwrap();
    fs::write(dir.join("agents/core/broken.md"), "no frontmatter").unwrap();
    fs::write(dir.join("SECRET.md"), agent("secret")).unwrap();
    let ws = Workspace::open(dir.to_str().unwrap()).unwrap();
    (dir, ws)
}

#[test]
fn scan_lists_valid_agents_with_groups() {
    let (_dir, ws) = temp_workspace("scan");
    let agents = scan(&ws);
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].path, "agents/core/copywriter.md");
    assert_eq!(agents[0].group, "core");
    assert_eq!(agents[0].description, "Does copywriter things.");
}

#[test]
fn find_only_resolves_scanned_paths() {
    let (dir, ws) = temp_workspace("find");
    assert!(read(&ws, "agents/core/copywriter.md").is_ok());
    for outside in ["SECRET.md", "agents/../SECRET.md", "agents/core/broken.md"] {
        assert_eq!(find(&ws, outside).unwrap_err(), "error.agentNotFound");
    }
    let absolute = dir.join("SECRET.md");
    assert!(find(&ws, absolute.to_str().unwrap()).is_err());
}

#[test]
fn save_writes_by_language_and_never_overwrites() {
    let (dir, ws) = temp_workspace("save");
    let first = save(&ws, Lang::Pt, &agent("clinic-ads")).unwrap();
    let second = save(&ws, Lang::Pt, &agent("clinic-ads")).unwrap();
    assert_eq!(first.path, "agents/generated/pt/clinic-ads.md");
    assert_eq!(second.path, "agents/generated/pt/clinic-ads-2.md");
    assert_eq!(second.name, "clinic-ads-2");
    assert_eq!(second.group, "generated/pt");
    assert!(dir.join("agents/generated/pt/clinic-ads.md").is_file());
}

#[test]
fn save_rejects_invalid_or_oversized_agents() {
    let (_dir, ws) = temp_workspace("reject");
    assert_eq!(
        save(&ws, Lang::En, "# no frontmatter").unwrap_err(),
        "error.invalidAgent"
    );
    assert_eq!(
        save(&ws, Lang::En, &agent("../escape")).unwrap_err(),
        "error.invalidAgent"
    );
    let huge = agent("big") + &"x".repeat(MAX_AGENT_BYTES);
    assert_eq!(save(&ws, Lang::En, &huge).unwrap_err(), "error.tooLarge");
}

#[test]
fn install_copies_by_name_and_asks_before_overwriting() {
    let (dir, ws) = temp_workspace("install");
    let target = dir.join("claude-agents");
    let installed = install(&ws, "agents/core/copywriter.md", &target, false).unwrap();
    assert!(installed.ends_with("copywriter.md"));
    assert_eq!(
        install(&ws, "agents/core/copywriter.md", &target, false).unwrap_err(),
        "error.installExists"
    );
    assert!(install(&ws, "agents/core/copywriter.md", &target, true).is_ok());
}

#[test]
fn saved_agents_get_yaml_safe_frontmatter() {
    let (dir, ws) = temp_workspace("yaml");
    let colon = "---\nname: ads\ndescription: Runs ads. Examples: \"go\"\n---\n\n# Ads\n";
    save(&ws, Lang::En, colon).unwrap();
    let saved = fs::read_to_string(dir.join("agents/generated/en/ads.md")).unwrap();
    assert!(saved.contains("description: >-\n  Runs ads. Examples: \"go\"\n"));
}

/// Every agent shipped in this repo must already be in the normalized, YAML-safe form.
#[test]
fn shipped_agents_have_yaml_safe_frontmatter() {
    let root = crate::workspace::repo_root().unwrap();
    let ws = Workspace::open(root.to_str().unwrap()).unwrap();
    let agents = scan(&ws);
    assert!(agents.len() >= 19, "found {} agents", agents.len());
    for agent in agents {
        let markdown = read(&ws, &agent.path).unwrap();
        assert_eq!(
            frontmatter::normalize(&markdown),
            markdown,
            "{} is not normalized",
            agent.path
        );
    }
}
