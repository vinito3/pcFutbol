use utils::GenericError;
use tauri_next_ui as ui;
use sqllite_db as db;

#[tokio::main]
async fn main() -> Result<(),GenericError> {
    
    //ui::start_gtk_ui()?;
    let db = match db::start_db().await {
           Ok(db) => db,
           Err(err) => panic!("{}", err),
    };

    Ok(())
}

