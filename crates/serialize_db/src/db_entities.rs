use serde::{Serialize, Deserialize,de::DeserializeOwned};
use std::fs::OpenOptions;
use std::result::Result;
use std::io::{Read, Write};
use utils::GenericError;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DbEntity<T> {
  pub list:  Vec<T>,
}

 impl<P> DbEntity<P> {

    pub fn load_entity<T: DeserializeOwned>(db_entity_path: &String) -> Result<T,GenericError> {

      let mut file = OpenOptions::new().read(true).write(true).create(true).open(db_entity_path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        print!("buf {}",buf.len());
        
        if buf.len() == 0 {
         
         Err(GenericError::new("Empty file"))

        }else {

          let entity = serde_json::from_slice(&buf[..])?;
          Ok(entity)
        }
        
    }
    
    pub fn save_entity<T: Serialize>(db_entity_path: &String,entity: DbEntity<T>) -> Result<(),GenericError> {
            
        let mut f = OpenOptions::new().write(true).create(true).open(db_entity_path)?;
        let buf = serde_json::to_vec(&entity)?;
        f.write_all(&buf[..])?;
        Ok(())
    }

 }