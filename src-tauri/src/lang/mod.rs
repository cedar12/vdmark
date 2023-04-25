use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::from_str;


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

const LANG_DIR:&str="lang";

pub fn get_lang(lang:String)->Lang{
    let dir=Path::new(LANG_DIR);
    let dir=dir.join(format!("{}.json",lang));
    let s=fs::read_to_string(dir).unwrap();
    let lang:Lang=from_str(s.as_str()).unwrap();
    lang
}