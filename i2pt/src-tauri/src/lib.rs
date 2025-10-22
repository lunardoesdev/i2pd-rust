use tauri::WindowEvent;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use i2pd;

#[tauri::command]
fn i2pd_start() {
    i2pd::start();
}

#[tauri::command]
fn i2pd_stop() {
    i2pd::stop();
}

#[tauri::command]
fn i2pd_terminate() {
    i2pd::terminate();
}

#[tauri::command]
fn i2pd_init(args: &str) {
    i2pd::init(args);
}


use std::fs;
use std::path::Path;
use tauri::Manager;

#[tauri::command]
async fn copy_resource_dir_to_config(
    app_handle: tauri::AppHandle,
    resource_dir_name: String,
) -> Result<(), String> {
    let resource_path = app_handle
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?
        .join(&resource_dir_name);

    let config_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?
        .join(&resource_dir_name);

    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    copy_dir_recursive(&resource_path, &config_dir)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn copy_dir_recursive(from: &Path, to: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from_path = entry.path();
        let to_path = to.join(entry.file_name());

        if file_type.is_dir() {
            fs::create_dir_all(&to_path)?;
            copy_dir_recursive(&from_path, &to_path)?;
        } else {
            fs::copy(&from_path, &to_path)?;
        }
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            i2pd_init,
            i2pd_start,
            i2pd_stop,
            i2pd_terminate,
            copy_resource_dir_to_config
        ])
        .on_window_event(|_window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                i2pd_stop();
                i2pd_terminate();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
