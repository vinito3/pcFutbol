mod db_entities;
mod db_player_entity;

use rusqlite::{Result,Error};

pub fn start_db() -> Result<String,Error> {

    let mut db = super::data::db_entities::Db::new(); 
    let db_player = super::data::db_player_entity::Player::new("pedro".to_string(),"Racing FC".to_string(),18,70);
    print!("Player: {:#?}",db_player);
    db.open();
    db.create_default_tables();
    db.close();
    Ok("Db Started successfully.".to_string())
}