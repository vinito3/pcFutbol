mod gui;
mod data;
mod constantes;
mod resources;

use std::num::ParseIntError;

fn main() -> Result<(),ParseIntError> {

    data::start_db();

    gui::start_ui();

    Ok(())
}

