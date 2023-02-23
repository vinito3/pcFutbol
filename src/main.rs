use utils::GenericError;

fn main() -> Result<(),GenericError> {
    
    db::serialization_db()?;
    //db::start_db().await?;

    //gui::start_ui();

    Ok(())
}

