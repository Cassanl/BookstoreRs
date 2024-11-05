use std::sync::Arc;

pub const DB_NAME: &'static str = "bookstore";
pub const BOOK_COLL: &'static str = "books";
pub const USER_COLL: &'static str = "users";

pub async fn init_mongodb(db_uri: &str) -> Result<mongodb::Client, mongodb::error::Error> {
    let client: mongodb::Client = mongodb::Client::with_uri_str(db_uri).await?;
    Ok(client)
}

pub struct MongodbState {
    pub client: Arc<mongodb::Client>,
    pub db: mongodb::Database,
}
