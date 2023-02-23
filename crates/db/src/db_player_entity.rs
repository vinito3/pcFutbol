use utils::GenericError;
use crate::db_entities::DbEntity;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug,Default,PartialEq,Serialize,Deserialize)]
pub struct Player {

    pub id:i8,
    pub name: String,
    pub football_club: String,
    pub age: i32,
    pub media: i32,
}

 impl Player {

    pub fn insert_player(mut player: Player) -> Result<(),GenericError> {

       let db_entity_path = env::var("PLAYER_DB_PATH")?;
       //let mut players: DbEntity<Player> = DbEntity::<Player>::load_entity(&db_entity_path)?;
       let mut players: DbEntity<Player>;
       //player.id = players.list.len() as i8 + 1 as i8 ;
       players.list.extend([player].into_iter());
       DbEntity::<Player>::save_entity(&db_entity_path, players)?; 

       Ok(())
    }

    pub fn select_player_by_id(player_id: i8) -> Result<Player,GenericError> {

        let db_entity_path = env::var("PLAYER_DB_PATH")?;
        let players: DbEntity<Player> = DbEntity::<Player>::load_entity(&db_entity_path)?;
        let player: Option<Player> = players.list.into_iter().find(|player| player.id == player_id);
        Ok(player.unwrap())
    }
 }

