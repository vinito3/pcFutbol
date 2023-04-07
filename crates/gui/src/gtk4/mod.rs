mod window;
mod imp;
mod resources;

use gtk::prelude::GtkWindowExt;
use window::Window;
use gtk::{Application,glib::Error};
use gtk::prelude::*;
use utils::GenericError;

pub fn start_gtk_ui() -> Result<(),GenericError> {

    // Create a new application
    let app = Application::builder().application_id("com.pcfutbol.PcFutbol").build();


    // Connect to "activate" signal of `app`
    app.connect_startup(|_| {
        resources::load_resources();
    });
    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}