// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

extern crate html5ever;

mod cmd;
mod db;
mod lang;
mod menu;
mod utils;
mod parser;

mod tests;

use tauri::Manager;
use window_shadows::set_shadow;

pub mod shadow {
    include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
}

fn main() {
    if let Err(e) = db::init() {
        println!("{:?}", e);
    }

    let lang = db::get_conf_default("lang".into(), "zh".into());
    println!("init lang {}", lang);

    let builder = tauri::Builder::default();
    builder
        .menu(menu::create_menu(lang.clone()))
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            if cfg!(target_os = "windows") {
                set_shadow(&window, true).expect("Unsupported platform!");
            } else {
                window.set_decorations(true).unwrap();
            }
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                window.open_devtools();
            }
            window.emit("lang", lang).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd::read_file,
            cmd::save_file,
            cmd::update_menu_lang,
            cmd::get_lang,
            cmd::pin,
            cmd::update_picgo,
            cmd::set_picgo_server,
            cmd::build_info,
            cmd::save_image,
            cmd::html2md,
            cmd::md2html,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
