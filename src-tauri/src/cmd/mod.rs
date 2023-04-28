use std::error::Error;
use std::{collections::HashMap, vec};
use reqwest::blocking::Client;
use tauri::{Window, Builder, Wry, EventLoopMessage};

use crate::{utils, parser};
use crate::{lang::Lang, db, shadow};


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
  // let menu_handle=window.menu_handle();
  // crate::menu::update_menu(lang, menu_handle);
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

#[tauri::command]
pub fn build_info()->HashMap<String,String>{
  let mut map=HashMap::new();
  map.insert("PKG_VERSION".into(), format!("{}",shadow::PKG_VERSION));         // current package version. e.g. '1.3.15-beta2'
  map.insert("BRANCH".into(), format!("{}",shadow::BRANCH));              // the branch, e.g. 'master'
  map.insert("SHORT_COMMIT".into(), format!("{}",shadow::SHORT_COMMIT));        // short commit hash, e.g. '8405e28e'
  map.insert("COMMIT_HASH".into(), format!("{}",shadow::COMMIT_HASH));         // full commit hash, e.g. '8405e28e64080a09525a6cf1b07c22fcaf71a5c5'
  map.insert("COMMIT_DATE".into(), format!("{}",shadow::COMMIT_DATE));         // commit date, e.g. '2021-08-04 12:34:03 +00:00'
  map.insert("COMMIT_AUTHOR".into(), format!("{}",shadow::COMMIT_AUTHOR));       // commit author, e.g. 'baoyachi'
  map.insert("COMMIT_EMAIL".into(), format!("{}",shadow::COMMIT_EMAIL));        // commit email, e.g. 'example@gmail.com'

  map.insert("BUILD_OS".into(), format!("{}",shadow::BUILD_OS));            // the OS that built the binary, e.g. 'macos-x86_64'
  map.insert("RUST_VERSION".into(), format!("{}",shadow::RUST_VERSION));        // rustc version e.g. 'rustc 1.45.0 (5c1f21c3b 2020-07-13)'
  map.insert("RUST_CHANNEL".into(), format!("{}",shadow::RUST_CHANNEL));        // rust toolchain e.g. 'stable-x86_64-apple-darwin (default)'
  map.insert("CARGO_VERSION".into(), format!("{}",shadow::CARGO_VERSION));       // cargo version e.g. 'cargo 1.45.0 (744bd1fbb 2020-06-15)'

  map.insert("PROJECT_NAME".into(), format!("{}",shadow::PROJECT_NAME));        // your project name, e.g. 'shadow-rs'
  map.insert("BUILD_TIME".into(), format!("{}",shadow::BUILD_TIME));          // time when start build occurred, e.g. '2020-08-16 14:50:25'
  map.insert("BUILD_RUST_CHANNEL".into(), format!("{}",shadow::BUILD_RUST_CHANNEL)); 
  map
}


#[tauri::command]
pub fn save_image(path:String,base64:String)->Option<String>{
  match utils::write_image(path,base64){
    Ok(())=>{
      None
    },
    Err(e)=>{
      println!("{:?}",e);
      Some("".into())
    }
  }
}


#[tauri::command]
pub fn html2md(html:String)->Option<String>{
  match parser::html::parse_tag(html.as_str()){
    Ok(md)=>Some(md),
    Err(e)=>{
      println!("{:?}",e);
      None
    }
  }
}
#[tauri::command]
pub fn md2html(md:String)->Option<String>{
  match parser::md::parse(md.as_str()){
    Ok(md)=>Some(md),
    Err(e)=>{
      println!("{:?}",e);
      None
    }
  }
}
