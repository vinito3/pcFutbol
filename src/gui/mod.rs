pub mod window;

use glib::Object;
use gtk::{gio, glib, Application};

pub fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}