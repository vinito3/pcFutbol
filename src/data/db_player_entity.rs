#[derive(Debug)]
pub struct Player {
    pub name: String,
    football_club: String,
    age: u32,
    media: u32,
}

impl Player {

    pub fn new(name: String,football_club: String,age: u32,media: u32) -> Self {

        Player {
            
            name,
            football_club,
            age,
            media, 
        }
    }
}