use mongodb::{Client, Database, options::ClientOptions};

pub async fn init_db(uri: &str, db_name: &str) -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("ActixAuth".into());
    let client = Client::with_options(client_options)?;
    Ok(client.database(db_name))
}
