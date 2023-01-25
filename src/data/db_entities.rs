use rusqlite::Connection;

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

        self.conn.take().unwrap().close().unwrap();
    }
    pub fn get_connection(&mut self) -> Option<Connection> {
        
        return self.conn;
    }

    pub fn create_default_tables(&mut self){

        self.conn.unwrap().execute(
            "CREATE TABLE PLAYER (
                id   INTEGER PRIMARY KEY,
                name TEXT,
                age INTEGER,
                club TEXT
            )",(),).unwrap();
    }
}