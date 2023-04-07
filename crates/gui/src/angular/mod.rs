use utils::GenericError;
use ux::prelude::*;
use ux::Window;
use std::default;

#[derive(Default, Application)]
struct Application{
    window: Window,
}

impl Application {

    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .show()
            .set_window_size(512, 512)
            .set_resizable(true)
            .set_title("Hello window")
            .connect_destroy(move |_win| {
                Application::quit()
            });
        
        app.window.set_background_color(Some(color::TEAL_9));
        
        app
    }
}

pub fn start_ui() -> Result<(),GenericError> {

    Application::run();

    Ok(())
}

fn build_ui(app: &Application) {

}


