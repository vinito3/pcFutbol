pub mod window;
pub mod imp;
use gtk::prelude::GtkWindowExt;
use window::Window;

use gtk::{Application};

pub fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}