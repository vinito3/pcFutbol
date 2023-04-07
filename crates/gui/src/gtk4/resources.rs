use gtk::{gio,gdk,CssProvider, StyleContext};
use gdk::Display;
use utils::GenericError;


pub fn load_resources() -> Result<(),GenericError> {

    // Register and include resources
    gio::resources_register_include!("../../../resources/gui/pcfutbol.gresource")?;
    //load css files
    load_css();

    Ok(())
}

pub fn load_css() {

    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data("/resources/gui/css/style.css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}