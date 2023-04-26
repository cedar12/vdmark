use std::{ sync::{Arc, Mutex}, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_json::from_str;

lazy_static!{
    pub static ref LOCALE:Arc<Mutex<HashMap<String,Vec<u8>>>>=Arc::new(Mutex::new({
        let mut map=HashMap::new();
        map.insert("zh".into(), include_bytes!("../../lang/zh.json").to_vec());
        map.insert("en".into(), include_bytes!("../../lang/en.json").to_vec());
        map
    }));
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Lang{
    pub file:File,
    pub help:Help,
    pub view:View,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct File{
    pub name:String,
    pub new:String,
    pub open:String,
    pub save:String,
    pub saveas:String,
    pub config:String,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Help{
    pub name:String,
    pub  about:String,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct View{
    pub  name:String,
    pub wysiwyg:String,
    pub  ir:String,
    pub  sv:String,
    pub top:String,
}

// const LANG_DIR:&str="lang";

pub fn get_lang(lang:String)->Lang{
    // let dir=Path::new(LANG_DIR);
    
    // let dir=dir.join(format!("{}.json",lang));
    
    // let s=fs::read_to_string(dir).unwrap();
    let locale=LOCALE.lock().unwrap();
    if let Some(locale)=locale.get(lang.as_str()){
        let s=String::from_utf8(locale.clone());
        match s{
            Ok(s)=>{
                let lang:Lang=from_str(s.as_str()).unwrap();
                return lang;
            },
            Err(e)=>{
                panic!("{:?}",e)
            }
        }
        
    }
    panic!("未获取到语言配置文件")
}

