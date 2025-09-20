
pub mod crypto;
pub mod auth;


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() {
    chatree_client_lib::run()
}
