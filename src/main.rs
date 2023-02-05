mod gui;
mod data;
mod constantes;
mod resources;
mod utils;

fn main() {

    match data::start_db() {

        Ok(msg) => println!("{}",msg),
        Err(err) => println!("Error starting Db: {:?}",err),
    }

    gui::start_ui();

}

