mod db_entities;
mod db_player_entity;

use utils::GenericError;
use db_player_entity::Player;


pub fn serialization_db() -> Result<(),GenericError> {

    let player = Player { id:0, name: "pedro".to_string(),football_club:"racing".to_string(),age:18,media:79};
    Player::insert_player(player)?;
    let player = Player::select_player_by_id(0).unwrap();
    println!("Player: {:#?}",player);
    Ok(())
}
