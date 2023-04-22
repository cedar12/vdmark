use tauri::Window;


#[tauri::command]
pub fn read_file(path:String)->Option<String>{
  let ret = std::fs::read_to_string(path);
  match ret{
    Ok(content)=>Some(content),
    Err(e)=>{
      eprintln!("读取文件错误: {:?}",e);
      None
    }
  }
}

#[tauri::command]
pub fn save_file(path:String,content:String)->Option<String>{
  let ret = std::fs::write(path, content);
  match ret{
    Ok(())=>None,
    Err(e)=>{
      eprintln!("写入文件错误: {:?}",e);
      Some(format!("{:?}",e))
    }
  }
}


#[tauri::command]
pub fn update_menu_lang(window:Window,lang:String){
  eprintln!("update_lang {}",lang);
  
  let menu_handle=window.menu_handle();
  crate::menu::update_menu(lang, menu_handle);
}