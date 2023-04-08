use utils::GenericError;
use gtk_ui as ui;

fn main() -> Result<(),GenericError> {
    
    ui::start_gtk_ui()?;

    Ok(())
}

