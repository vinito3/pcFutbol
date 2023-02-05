use rusqlite::Connection;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Db {
    
    pub db_name: String,
    pub conn: Option<Connection>,
}

impl Db {
    
    pub fn new() -> Self {

        return Db {
            db_name: String::from("pcfutbol.db") ,
            conn: None, 
            };
    }

    pub fn open(&mut self) {
        
        self.conn = Some(Connection::open("pcfutbol.db").unwrap());
    }

    pub fn close(&mut self) {

        let conn = self.conn.take();
        
        match conn.unwrap().close() {

            Ok(()) => (),
            Err(err) => println!("Db conecction close error: {:?}",err),
        }
        
    }
    pub fn get_connection(&mut self) -> Option<&mut Connection> {

        return self.conn.as_mut();
        
        
    }

    pub fn create_default_tables(&mut self) {

        let conn = self.get_connection().unwrap();

        conn.execute(
            "create table if not exists player (
                id integer primary key,
                name text,
                age integer,
                football_club text,
                media integer
            )",(),).unwrap();
        
        conn.execute(
                "create table if not exists football_club (
                    id integer primary key,
                    name text,
                    foundation_date integer,
                    stadium_name text,
                    money integer
                )",(),).unwrap();
    }

    pub fn insert(&mut self,table_name: String,fields: HashMap<String,String>) {

        let conn = self.get_connection().unwrap();
        
        let mut insert_query: String = "INSERT INTO ".to_string() + &table_name + " " + "(";
        insert_query = crate::utils::cocatenate_string_from_hashmap_key(&mut insert_query,&fields).to_string() + ")" + " values (";
        insert_query = crate::utils::cocatenate_string_from_hashmap_value(&mut insert_query,&fields).to_string() + ");";
        
       
        
    }
}