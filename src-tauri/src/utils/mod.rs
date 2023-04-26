use std::{path::PathBuf, fs};

use tauri::{api::path::{self, resolve_path, BaseDirectory}, Config, Env};


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
