// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod controller;
mod model;
mod service;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::delete_repeat,
            controller::download_file,
            controller::delete_online_repeat
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
