use std::sync::Arc;
use parking_lot::Mutex;
use tauri::Manager;

mod ws;
use ws::{chat_runtime::ChatRuntime, ws_handler::{WsGlobalRouterBuilder, WsRouterBuilder}};

mod commands;
use commands::member::handlers::FindMemberHandler;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(
            |app| {
                app.manage(Arc::new(Mutex::new(ChatRuntime::default())));
                app.manage(Arc::new(
                    WsGlobalRouterBuilder::new()
                        .add_router(
                            "member",
                            WsRouterBuilder::new()
                                .add_handler("find", FindMemberHandler)
                                .result()
                        )
                        .result()
                ));
                Ok(())
            }
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
