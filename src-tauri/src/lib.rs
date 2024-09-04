// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// -- Sub-Modules
mod model;
mod utils;
mod error;
mod prelude;
mod event;
mod ctx;
mod ipc;

// -- Re-Exports
pub use error::{Error, Result};

// -- Imports
use std::sync::Arc;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let model_manager = ModelStore::new().await?;
    let model_manager = Arc::new(model_manager);

    // for dev only
    seed_store_for_dev(model_manager.clone()).await?;

    tauri::Builder::default()
        .manage(model_manager)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
