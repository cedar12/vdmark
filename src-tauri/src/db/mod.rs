use rusqlite::{Connection, Result};
use std::sync::{Arc,Mutex};

use crate::utils;

lazy_static! {
  static ref CONN:Arc<Mutex<Option<Connection>>>=Arc::new(Mutex::new(None));
}

const PATH:&str="data.db";

pub fn init()->anyhow::Result<()>{
  let dir=utils::get_path();
  let conn=Connection::open(dir.join(PATH))?;
  conn.execute(
      "create table if not exists app_conf (
          key    varchar(255) PRIMARY KEY,
          value  TEXT NULL
      )",
      (), // empty list of parameters.
  )?;
  conn.execute(
    "create table if not exists open_path (
        path    varchar(255) PRIMARY KEY,
        open_time  varchar(255) NULL
    )",
    (), // empty list of parameters.
)?;
let _=CONN.lock().unwrap().insert(conn);
  Ok(())
}


fn query_conf(conn:&Connection,key:String)->anyhow::Result<String>{
  let mut stmt = conn.prepare("SELECT `value` FROM app_conf where `key`=?")?;
  let value=stmt.query_row([key], |row|{
    let value:String=row.get(0)?;
    Ok(value)
  })?;
  Ok(value)
}

fn update_conf(conn:&Connection,key:String,value:String)->anyhow::Result<usize>{

  let mut rows_count = conn.execute(
    "UPDATE app_conf SET `value` = ? WHERE `key` = ?",
    [value.clone(),key.clone()],
  )?;
  if rows_count==0{
    rows_count = conn.execute(
      "INSERT INTO app_conf (key, value) VALUES (?, ?)",
      [ key,value],
    )?;
  }
  Ok(rows_count)
}

pub fn get_conf_default(key:String,default:String)->String{
  let conn=CONN.lock().unwrap();
  if let Some(conn)=conn.as_ref() {
    match query_conf(conn,key){
      Ok(value)=>{
        if value==""{
          return default
        }
        value
      },
      Err(e)=>{
        println!("{:?}",e);
        default
      }
    }
  }else {
    default
  }
}

pub fn set_conf(key:String,value:String){
  let conn=CONN.lock().unwrap();
  if let Some(conn)=conn.as_ref() {
    match update_conf(conn,key,value){
      Ok(n)=>{},
      Err(e)=>{
        println!("{:?}",e);
      }
    }
  }
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}