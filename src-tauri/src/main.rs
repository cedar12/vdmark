// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;

use tauri::Manager;
use window_shadows::set_shadow;



fn main() {
    tauri::Builder::default()
        .setup(|app|{
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmd::read_file,cmd::save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
