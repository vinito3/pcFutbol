pub mod window;
pub mod imp;
use gtk::prelude::GtkWindowExt;
use window::Window;
use gtk::Application;
use gtk::prelude::*;

pub fn start_ui(){

        // Create a new application
        let app = Application::builder().application_id(super::constantes::APP_ID).build();

        // Connect to "activate" signal of `app`
        app.connect_startup(|_| super::resources::load_resources());
        app.connect_activate(super::gui::build_ui);
    
        // Run the application
        app.run();
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}