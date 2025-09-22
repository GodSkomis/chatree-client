
pub mod crypto;
pub mod auth;
pub mod commands;
pub mod message;
pub mod ws;


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() {
    chatree_client_lib::run()
}
