use std::sync::Arc;

pub const DB_NAME: &str = "bookstore";
pub const BOOK_COLL: &str = "books";
pub const USER_COLL: &str = "users";

pub async fn init_mongodb(db_uri: &str) -> Result<mongodb::Client, mongodb::error::Error> {
    let client: mongodb::Client = mongodb::Client::with_uri_str(db_uri).await?;
    Ok(client)
}

pub fn get_mongo_collection<T>(
    c: Arc<mongodb::Client>,
    db_name: &str,
    coll_name: &str,
) -> mongodb::Collection<T>
where
    T: Sync + Send,
{
    c.database(db_name).collection(coll_name)
}

pub struct MongodbState {
    pub client: Arc<mongodb::Client>,
    pub db: mongodb::Database,
}
