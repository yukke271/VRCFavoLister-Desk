// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod structs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::auth::login,
            commands::auth::logout,
            commands::auth::check_cookie,
            commands::favorite::load_favorite_worlds,
            commands::favorite::read_favorite,
        ])
        .manage(structs::app_state::AppState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}