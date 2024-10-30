const DB_DEFAULT_URI: &'static str = "mongodb://localhost:27017";
pub const DB_NAME: &'static str = "bookstore";
pub const BOOK_COLL: &'static str = "books";
pub const USER_COLL: &'static str = "users";

pub struct DBMap<'a, T> {
    pub key: &'a str,
    pub value: T,   
}

pub async fn init_mongodb() -> Result<mongodb::Client, mongodb::error::Error> {
    let db_uri: String = std::env::var("MONGO_URI").unwrap_or_else(|_| DB_DEFAULT_URI.to_owned());
    let client: mongodb::Client = mongodb::Client::with_uri_str(db_uri).await?;
    Ok(client)
}