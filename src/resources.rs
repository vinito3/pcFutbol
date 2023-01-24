use gtk::{gio,gdk,CssProvider, StyleContext};
use gdk::Display;

pub fn load_resources(){

    
    
    // Register and include resources
    gio::resources_register_include!("pcfutbol.gresource")
        .expect("Failed to register resources.");

}

pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("../resources/css/style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}