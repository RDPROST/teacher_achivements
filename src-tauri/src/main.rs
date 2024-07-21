#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod update;

use std::env;
use update::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_for_updates, download_file, open_save_dialog, open_link, get_download_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
