// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
mod menu;
mod lang;

use menu::update_menu;
use tauri::Manager;
use window_shadows::set_shadow;



fn main() {
    let builder=tauri::Builder::default();
    builder.menu(menu::create_menu("zh".into()))
    .on_menu_event(|event| {
        let id=event.menu_item_id();
        match  id{
          "quit" => {
            std::process::exit(0);
          }
          "close" => {
            event.window().close().unwrap();
          }
          _ => {
            event.window().emit("menu", id).unwrap();
          }
        }
      })
        .setup(|app|{
            let window = app.get_window("main").unwrap();
            if cfg!(target_os = "windows") {
                window.set_decorations(true).unwrap();
                set_shadow(&window, true).expect("Unsupported platform!");
                
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmd::read_file,cmd::save_file,cmd::update_menu_lang])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
