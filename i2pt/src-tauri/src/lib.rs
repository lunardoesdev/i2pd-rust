// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use i2pd;

#[tauri::command]
fn starti2pd(args: &str) {
    i2pd::init(args);
    i2pd::start();
}

#[tauri::command]
fn stopi2pd() {
    i2pd::stop();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, starti2pd])
        // .invoke_handler(tauri::generate_handler![starti2pd])
        // .invoke_handler(tauri::generate_handler![stopi2pd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
