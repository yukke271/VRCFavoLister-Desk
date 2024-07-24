// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod structs;

use structs::app_state::AppState;

fn main() {
    
    // let app_state = AppState::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::auth::login,
            commands::favorite::load_favorite,
            commands::favorite::read_favorite,
        ])
        // .setup(|app| {
        //     app.manage(app_state);
        //     Ok(())
        // })
        .manage(AppState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}