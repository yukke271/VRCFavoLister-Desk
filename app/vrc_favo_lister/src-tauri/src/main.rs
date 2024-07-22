// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod structs;
mod database;

use tauri::{Manager, async_runtime::block_on};
use database::sqlitedb::init_db_pool;

fn main() {
    
    // DBの初期化
    let db_pool = block_on(init_db_pool()).expect("error while initializing database pool");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::auth::login,
            commands::favorite::load_favorite,
        ])
        .setup(|app| {
            app.manage(db_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
*/
