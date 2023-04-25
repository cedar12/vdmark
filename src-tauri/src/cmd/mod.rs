use std::collections::HashMap;
use reqwest::blocking::Client;
use tauri::{Window};

use crate::{lang::Lang, db};


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
  db::set_conf("lang".into(),lang.clone());
  //let menu_handle=window.menu_handle();
  //crate::menu::update_menu(lang, menu_handle);
}

#[tauri::command]
pub fn get_lang(lang:String)->Lang{
  crate::lang::get_lang(lang)
}

#[tauri::command]
pub fn pin(window:Window,pin:bool){
  window.set_always_on_top(pin).unwrap();
}

#[derive(serde::Serialize,serde::Deserialize)]
pub struct PicGoResp{
  success:bool,
  result:Vec<String>,
}

#[tauri::command]
pub fn update_picgo(path:&str)->PicGoResp{
  let client = Client::new();
  let mut json=HashMap::new();
  json.insert("list", vec![path]);
  let server=db::get_conf_default("picgo".into(), "127.0.0.1:36677".into());
  let response = client.post(format!("http://{}/upload",server))
      .json(&json)
      .send();
  match response{
    Ok(response)=>{
      match response.json(){
        Err(e)=>{
          PicGoResp{
            success:false,result:vec![format!("{:?}",e)]
          }
        },
        Ok(json)=>json
      }
    },
    Err(e)=>{
      PicGoResp{
        success:false,result:vec![format!("{:?}",e)]
      }
    }
  }
}


#[tauri::command]
pub fn set_picgo_server(server:String){
  db::set_conf("picgo".into(),server);
}