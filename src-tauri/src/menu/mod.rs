
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, window::MenuHandle, Manager};
use crate::lang::get_lang;

// #[cfg(not(target_os="windows"))]
pub fn create_menu(lang:String)->Menu{

    let locale=get_lang(lang);
    let file_new = CustomMenuItem::new("file.new".to_string(), locale.file.new);
    
    let file_open = CustomMenuItem::new("file.open".to_string(), locale.file.open);
    let file_save = CustomMenuItem::new("file.save".to_string(), locale.file.save);
    let file_saveas = CustomMenuItem::new("file.saveas".to_string(), locale.file.saveas);
    let file_config = CustomMenuItem::new("file.config".to_string(), locale.file.config);
    let file = Submenu::new(locale.file.name, Menu::new().add_item(file_new).add_item(file_open).add_item(file_save).add_item(file_saveas).add_item(file_config));
    
    let view_wysiwgy = CustomMenuItem::new("view.wysiwyg".to_string(), locale.view.wysiwyg);
    let view_ir = CustomMenuItem::new("view.ir".to_string(), locale.view.ir);
    let view_sv = CustomMenuItem::new("view.sv".to_string(), locale.view.sv);
    let view_top = CustomMenuItem::new("view.top".to_string(), locale.view.top);
    let view = Submenu::new(locale.view.name, Menu::new().add_item(view_wysiwgy).add_item(view_ir).add_item(view_sv).add_item(view_top));

    let view_about = CustomMenuItem::new("help.about".to_string(), locale.help.about);
    let help = Submenu::new(locale.help.name, Menu::new().add_item(view_about));
    
    let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_native_item(MenuItem::Cut)
    .add_native_item(MenuItem::Paste)
    .add_native_item(MenuItem::Redo)
    .add_native_item(MenuItem::Undo)
    .add_native_item(MenuItem::SelectAll)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(file)
    .add_submenu(view)
    .add_submenu(help);
    menu
}

pub fn update_menu(lang:String,menu_handle:MenuHandle){
    let locale=get_lang(lang);

    std::thread::spawn(move ||{
        menu_handle.get_item("file.new").set_title(locale.file.new).unwrap();
        menu_handle.get_item("file.open").set_title(locale.file.open).unwrap();
        menu_handle.get_item("file.save").set_title(locale.file.save).unwrap();
        menu_handle.get_item("file.saveas").set_title(locale.file.saveas).unwrap();
        menu_handle.get_item("file.config").set_title(locale.file.config).unwrap();
        menu_handle.get_item("view.wysiwyg").set_title(locale.view.wysiwyg).unwrap();
        menu_handle.get_item("view.ir").set_title(locale.view.ir).unwrap();
        menu_handle.get_item("view.sv").set_title(locale.view.sv).unwrap();
    });
}


// #[cfg(target_os="windows")]
// pub fn create_menu(_lang:String)->Menu{
//     Menu::new()
// }