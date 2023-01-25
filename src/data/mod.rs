mod db_entities;
mod entities;

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn start_db() -> Result<()>{

    let db = super::data::db_entities::Db::new(); 
    db.open();
    db.create_default_tables();
        
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    db.conn.unwrap().execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = db.conn.unwrap().prepare("SELECT id, name, data FROM person");
    match stmt {

        Ok(stmt) -> let person_iter = stmt.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?;
        
    }
    

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}