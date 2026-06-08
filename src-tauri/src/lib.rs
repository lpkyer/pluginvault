mod arch;
mod db;
mod operations;
mod plugin;
mod scanner;

use db::Database;
use plugin::Plugin;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    db: Database,
}

#[tauri::command]
async fn scan_plugins(state: State<'_, Mutex<AppState>>) -> Result<Vec<Plugin>, String> {
    let plugins = tauri::async_runtime::spawn_blocking(|| scanner::scan_plugins())
        .await
        .map_err(|e| e.to_string())?;

    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state
        .db
        .upsert_plugins(&plugins)
        .map_err(|e| e.to_string())?;
    app_state
        .db
        .remove_stale_plugins(&plugins)
        .map_err(|e| e.to_string())?;
    Ok(plugins)
}

#[tauri::command]
fn get_plugins(state: State<'_, Mutex<AppState>>) -> Result<Vec<Plugin>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.db.get_all_plugins().map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_plugin(id: String, enable: bool, state: State<'_, Mutex<AppState>>) -> Result<bool, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let plugin = app_state
        .db
        .get_plugin(&id)
        .map_err(|e| e.to_string())?
        .ok_or("Plugin not found")?;

    drop(app_state);
    let result = operations::toggle_plugin(&plugin.path, enable)?;
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state
        .db
        .update_enabled(&id, result)
        .map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
fn delete_plugin(id: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    let plugin = app_state
        .db
        .get_plugin(&id)
        .map_err(|e| e.to_string())?
        .ok_or("Plugin not found")?;

    drop(app_state);
    operations::delete_plugin(&plugin.path)?;
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state
        .db
        .delete_plugin(&id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn reveal_in_finder(path: String) -> Result<(), String> {
    std::process::Command::new("open")
        .args(["-R", &path])
        .output()
        .map_err(|e| format!("Failed to reveal in Finder: {}", e))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let db_path = app_dir.join("pluginvault.db");
            let db = Database::new(db_path.to_str().unwrap())
                .expect("failed to initialize database");

            app.manage(Mutex::new(AppState { db }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_plugins,
            get_plugins,
            toggle_plugin,
            delete_plugin,
            reveal_in_finder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
