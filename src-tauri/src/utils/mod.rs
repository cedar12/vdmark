use std::{path::PathBuf, fs};

use tauri::{api::path::{self, resolve_path, BaseDirectory}, Config, Env};

use crate::shadow;

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
    println!("{}", shadow::PKG_VERSION);         // current package version. e.g. '1.3.15-beta2'
    println!("{}", shadow::BRANCH);              // the branch, e.g. 'master'
    println!("{}", shadow::SHORT_COMMIT);        // short commit hash, e.g. '8405e28e'
    println!("{}", shadow::COMMIT_HASH);         // full commit hash, e.g. '8405e28e64080a09525a6cf1b07c22fcaf71a5c5'
    println!("{}", shadow::COMMIT_DATE);         // commit date, e.g. '2021-08-04 12:34:03 +00:00'
    println!("{}", shadow::COMMIT_AUTHOR);       // commit author, e.g. 'baoyachi'
    println!("{}", shadow::COMMIT_EMAIL);        // commit email, e.g. 'example@gmail.com'

    println!("{}", shadow::BUILD_OS);            // the OS that built the binary, e.g. 'macos-x86_64'
    println!("{}", shadow::RUST_VERSION);        // rustc version e.g. 'rustc 1.45.0 (5c1f21c3b 2020-07-13)'
    println!("{}", shadow::RUST_CHANNEL);        // rust toolchain e.g. 'stable-x86_64-apple-darwin (default)'
    println!("{}", shadow::CARGO_VERSION);       // cargo version e.g. 'cargo 1.45.0 (744bd1fbb 2020-06-15)'
    println!("{}", shadow::CARGO_TREE);          // e.g. the output of '$ cargo tree'

    println!("{}", shadow::PROJECT_NAME);        // your project name, e.g. 'shadow-rs'
    // Time respects SOURCE_DATE_EPOCH environment variable - see below
    println!("{}", shadow::BUILD_TIME);          // time when start build occurred, e.g. '2020-08-16 14:50:25'
    println!("{}", shadow::BUILD_RUST_CHANNEL); 
}