use crate::lang::get_lang;
use tauri::{window::MenuHandle, CustomMenuItem, Manager, Menu, MenuItem, Submenu};



#[cfg(target_os="macos")]
const ACCELERATOR_FILE_NEW:&str="cmd+n";
#[cfg(not(target_os="macos"))]
const ACCELERATOR_FILE_NEW:&str="ctrl+n";

#[cfg(target_os="macos")]
const ACCELERATOR_FILE_OPEN:&str="cmd+shift+o";
#[cfg(not(target_os="macos"))]
const ACCELERATOR_FILE_OPEN:&str="ctrl+alt+o";

#[cfg(target_os="macos")]
const ACCELERATOR_FILE_SAVE:&str="cmd+s";
#[cfg(not(target_os="macos"))]
const ACCELERATOR_FILE_SAVE:&str="ctrl+s";

#[cfg(target_os="macos")]
const ACCELERATOR_FILE_SAVEAS:&str="cmd+shift+s";
#[cfg(not(target_os="macos"))]
const ACCELERATOR_FILE_SAVEAS:&str="ctrl+alt+s";


#[cfg(target_os="macos")]
const ACCELERATOR_VIEW_WY:&str="cmd+option+7";

#[cfg(target_os="macos")]
const ACCELERATOR_VIEW_IR:&str="cmd+option+8";

#[cfg(target_os="macos")]
const ACCELERATOR_VIEW_SV:&str="cmd+option+9";


// #[cfg(not(target_os="windows"))]
pub fn create_menu(lang: String) -> Menu {
    let locale = get_lang(lang);
    let file_new =
        CustomMenuItem::new("file.new".to_string(), locale.file.new).accelerator(ACCELERATOR_FILE_NEW);

    let file_open = CustomMenuItem::new("file.open".to_string(), locale.file.open).accelerator(ACCELERATOR_FILE_OPEN);
    let file_save = CustomMenuItem::new("file.save".to_string(), locale.file.save).accelerator(ACCELERATOR_FILE_SAVE);
    let file_saveas = CustomMenuItem::new("file.saveas".to_string(), locale.file.saveas).accelerator(ACCELERATOR_FILE_SAVEAS);
    let file_config = CustomMenuItem::new("file.config".to_string(), locale.file.config);
    let file = Submenu::new(
        locale.file.name,
        Menu::new()
            .add_item(file_new)
            .add_item(file_open)
            .add_native_item(MenuItem::Separator)
            .add_item(file_save)
            .add_item(file_saveas)
            .add_native_item(MenuItem::Separator)
            .add_item(file_config),
    );

    let view_wysiwgy = CustomMenuItem::new("view.wysiwyg".to_string(), locale.view.wysiwyg).accelerator(ACCELERATOR_VIEW_WY);
    let view_ir = CustomMenuItem::new("view.ir".to_string(), locale.view.ir).accelerator(ACCELERATOR_VIEW_IR);
    let view_sv = CustomMenuItem::new("view.sv".to_string(), locale.view.sv).accelerator(ACCELERATOR_VIEW_SV);
    let view_top = CustomMenuItem::new("view.top".to_string(), locale.view.top);
    let view = Submenu::new(
        locale.view.name,
        Menu::new()
            .add_item(view_wysiwgy)
            .add_item(view_ir)
            .add_item(view_sv)
            .add_native_item(MenuItem::Separator)
            .add_item(view_top),
    );

    let view_about = CustomMenuItem::new("help.about".to_string(), locale.help.about);
    let help = Submenu::new(locale.help.name, Menu::new().add_item(view_about));

    let edit_item = Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::SelectAll);
    let edit = Submenu::new(locale.edit.name, edit_item);

    let menu = Menu::new()
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(file)
        .add_submenu(edit)
        .add_submenu(view)
        .add_submenu(help);
    menu
}

pub fn update_menu(lang: String, menu_handle: MenuHandle) {
    let locale = get_lang(lang);

    std::thread::spawn(move || {
        menu_handle
            .get_item("file.new")
            .set_title(locale.file.new)
            .unwrap();
        menu_handle
            .get_item("file.open")
            .set_title(locale.file.open)
            .unwrap();
        menu_handle
            .get_item("file.save")
            .set_title(locale.file.save)
            .unwrap();
        menu_handle
            .get_item("file.saveas")
            .set_title(locale.file.saveas)
            .unwrap();
        menu_handle
            .get_item("file.config")
            .set_title(locale.file.config)
            .unwrap();
        menu_handle
            .get_item("view.wysiwyg")
            .set_title(locale.view.wysiwyg)
            .unwrap();
        menu_handle
            .get_item("view.ir")
            .set_title(locale.view.ir)
            .unwrap();
        menu_handle
            .get_item("view.sv")
            .set_title(locale.view.sv)
            .unwrap();
    });
}

// #[cfg(target_os="windows")]
// pub fn create_menu(_lang:String)->Menu{
//     Menu::new()
// }
