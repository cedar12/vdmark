use std::{path::PathBuf, fs};

use tauri::{api::path::{self, resolve_path, BaseDirectory}, Config, Env};

pub fn get_resolve_path(path:String)->PathBuf{
  let context = tauri::generate_context!("tauri.conf.json");
  let path = resolve_path(
    context.config(),
    context.package_info(),
    &Env::default(),
    path,
    Some(BaseDirectory::AppData))
  .expect("failed to resolve path");
  path

}

pub fn get_path()->PathBuf{
  match path::app_data_dir(&Config::default()){
    Some(p)=>{
      let p=p.join("com.cedar12.vdmark");
      if !p.exists(){
        fs::create_dir_all(p.clone()).unwrap();
      }
      p
    },
    None=>PathBuf::new()
  }
  
}


#[test]
fn test_resouces_path(){
  let p=get_resolve_path("lang".into()).join("zh.json");
  println!("{:?}",p);
  let p=get_path();
  println!("{:?}",p);
  let config_bytes = include_bytes!("..\\..\\lang\\zh.json");
  println!("{}",String::from_utf8_lossy(config_bytes));
  let my_var = std::env::var("LANG_ZH").expect("MY_VAR is not set");
  println!("my_var = {}", my_var);
}