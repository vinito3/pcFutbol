use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate,Entry,Box};

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "../../../resources/gui/pcfutbol.ui")]
pub struct Window {
    #[template_child]
    pub main_button: TemplateChild<Button>,
    #[template_child]
    pub name_input: TemplateChild<Entry>,
    #[template_child]
    pub main_box: TemplateChild<Box>,
}
// ANCHOR_END: object

// ANCHOR: subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainAppWindow";
    type Type = super::window::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: subclass

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        let name = self.name_input.to_owned();
        // Connect to "clicked" signal of `button`
        self.main_button.connect_clicked(move |button| {
                        
            button.set_label(&name.text());            
               
        });
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}