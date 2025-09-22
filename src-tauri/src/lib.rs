use std::sync::Arc;
use parking_lot::Mutex;
use tauri::Manager;

mod ws;
use ws::chat_runtime::ChatRuntime;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(
            |app| {
                app.manage(Arc::new(Mutex::new(ChatRuntime::default())));
                Ok(())
            }
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
