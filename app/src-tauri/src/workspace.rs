use std::fs;
use std::path::{Path, PathBuf};

use crate::catalog::{parse_areas, Area};
use crate::prompt::Lang;

/// The agents library on disk: a folder with `agents/`, `catalog/` and `STANDARD.md`
/// (this repo, or any clone of it).
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn open(path: &str) -> Result<Self, String> {
        let root = Path::new(path)
            .canonicalize()
            .map_err(|_| "error.workspace".to_string())?;
        if !root.join("agents").is_dir() {
            return Err("error.workspace".into());
        }
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn agents_dir(&self) -> PathBuf {
        self.root.join("agents")
    }

    pub fn generated_dir(&self, lang: Lang) -> PathBuf {
        self.agents_dir().join("generated").join(lang.code())
    }

    pub fn areas(&self) -> Result<Vec<Area>, String> {
        parse_areas(&self.read("catalog/areas.json")?)
    }

    pub fn standard(&self) -> Result<String, String> {
        self.read("STANDARD.md")
    }

    fn read(&self, relative: &str) -> Result<String, String> {
        fs::read_to_string(self.root.join(relative)).map_err(|e| format!("{relative}: {e}"))
    }
}

/// Builds made from a clone use the clone itself as the library; installed builds on other
/// machines get `None` and use a folder in Documents instead.
pub fn repo_root() -> Option<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent()?.parent()?;
    root.join("agents").is_dir().then(|| root.to_path_buf())
}

/// Turns `target` into a library by copying the bundled base agents, catalog and standard
/// into it. Existing files are never overwritten, so edited agents survive app updates and
/// new base agents still arrive. Refuses folders that hold anything else.
// Known limit: a base agent the user deleted comes back on the next launch.
pub fn seed(bundled: &Path, target: &Path) -> Result<(), String> {
    let is_library = target.join("agents").is_dir();
    if !is_library && has_visible_entries(target) {
        return Err("error.workspace".into());
    }
    if bundled.is_dir() {
        copy_missing(bundled, target).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn has_visible_entries(dir: &Path) -> bool {
    fs::read_dir(dir).is_ok_and(|entries| {
        entries
            .flatten()
            .any(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
    })
}

fn copy_missing(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)?.flatten() {
        let destination = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_missing(&entry.path(), &destination)?;
        } else if !destination.exists() {
            fs::copy(entry.path(), &destination)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("hob-seed-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn opens_the_repo_and_reads_its_files() {
        let ws = Workspace::open(repo_root().unwrap().to_str().unwrap()).unwrap();
        assert!(!ws.areas().unwrap().is_empty());
        assert!(ws.standard().unwrap().contains("Agent Standard"));
        assert!(ws.generated_dir(Lang::Pt).ends_with("agents/generated/pt"));
    }

    #[test]
    fn rejects_folders_without_agents() {
        assert_eq!(Workspace::open("/").err().unwrap(), "error.workspace");
        assert_eq!(Workspace::open("/definitely/missing").err().unwrap(), "error.workspace");
    }

    /// A miniature of the bundle `tauri.conf.json` ships under `library/`.
    fn bundle(tag: &str) -> PathBuf {
        let dir = temp_dir(&format!("{tag}-bundle"));
        fs::create_dir_all(dir.join("agents/core")).unwrap();
        fs::create_dir_all(dir.join("catalog")).unwrap();
        fs::write(dir.join("agents/core/traffic-manager.md"), "bundled").unwrap();
        fs::write(dir.join("catalog/areas.json"), "[]").unwrap();
        fs::write(dir.join("STANDARD.md"), "# Agent Standard").unwrap();
        dir
    }

    #[test]
    fn seeds_a_new_library_from_the_bundle() {
        let target = temp_dir("new");
        seed(&bundle("new"), &target).unwrap();
        let ws = Workspace::open(target.to_str().unwrap()).unwrap();
        assert!(ws.standard().is_ok());
        assert!(target.join("agents/core/traffic-manager.md").is_file());
    }

    #[test]
    fn seeding_keeps_edited_files_and_refuses_unrelated_folders() {
        let target = temp_dir("edit");
        fs::create_dir_all(target.join("agents/core")).unwrap();
        fs::write(target.join("agents/core/traffic-manager.md"), "mine").unwrap();
        seed(&bundle("edit"), &target).unwrap();
        assert_eq!(fs::read_to_string(target.join("agents/core/traffic-manager.md")).unwrap(), "mine");

        let unrelated = temp_dir("unrelated");
        fs::create_dir_all(&unrelated).unwrap();
        fs::write(unrelated.join("notes.txt"), "x").unwrap();
        assert_eq!(seed(&bundle("unrelated"), &unrelated).unwrap_err(), "error.workspace");
    }
}
