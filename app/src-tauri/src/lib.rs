mod catalog;
mod claude;
mod frontmatter;
mod generator;
mod library;
mod prompt;
mod workspace;

use std::path::PathBuf;

use tauri::Manager;
use tauri::path::BaseDirectory;

use catalog::Area;
use generator::GenerateRequest;
use library::AgentSummary;
use prompt::Lang;
use workspace::Workspace;

// Errors are i18n keys ("error.*") the UI translates, or raw messages it shows as-is.

/// Prepares the library folder (the saved one, or the default when `path` is None) and
/// returns its path.
#[tauri::command]
fn open_library(app: tauri::AppHandle, path: Option<String>) -> Result<String, String> {
    let target = match path {
        Some(path) => PathBuf::from(path),
        None => default_library(&app)?,
    };
    let bundled = app
        .path()
        .resolve("library", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;
    workspace::seed(&bundled, &target)?;
    Ok(target.display().to_string())
}

fn default_library(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    if let Some(repo) = workspace::repo_root() {
        return Ok(repo);
    }
    let documents = app.path().document_dir().map_err(|e| e.to_string())?;
    Ok(documents.join("Home of Builders"))
}

#[tauri::command]
fn claude_status() -> Option<String> {
    claude::locate().map(|path| path.display().to_string())
}

#[tauri::command]
fn list_agents(workspace: String) -> Result<Vec<AgentSummary>, String> {
    Ok(library::scan(&Workspace::open(&workspace)?))
}

#[tauri::command]
fn read_agent(workspace: String, path: String) -> Result<String, String> {
    library::read(&Workspace::open(&workspace)?, &path)
}

#[tauri::command]
fn list_areas(workspace: String) -> Result<Vec<Area>, String> {
    Workspace::open(&workspace)?.areas()
}

#[tauri::command]
async fn generate_agent(request: GenerateRequest) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || generator::generate(&request))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
fn save_agent(workspace: String, lang: Lang, markdown: String) -> Result<AgentSummary, String> {
    library::save(&Workspace::open(&workspace)?, lang, &markdown)
}

#[tauri::command]
fn install_agent(
    app: tauri::AppHandle,
    workspace: String,
    path: String,
    overwrite: bool,
) -> Result<String, String> {
    let home = app.path().home_dir().map_err(|e| e.to_string())?;
    let target = home.join(".claude").join("agents");
    library::install(&Workspace::open(&workspace)?, &path, &target, overwrite)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_library,
            claude_status,
            list_agents,
            read_agent,
            list_areas,
            generate_agent,
            save_agent,
            install_agent
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
