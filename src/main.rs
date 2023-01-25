mod gui;
mod data;
mod constantes;
mod resources;

use gtk::prelude::*;
use gtk::{Application};

fn main() {

    // Create a new application
    let app = Application::builder().application_id(constantes::APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|_| resources::load_resources());
    app.connect_activate(gui::build_ui);

    // Run the application
    app.run();
}

