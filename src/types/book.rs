use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const MAX_TITLE_LEN: usize = 256;

#[derive(Serialize, Deserialize, Default)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub title: String,
}

impl Book {
    pub fn new(title: String) -> Self {
        Self { id: None, title }
    }

    pub fn new_from_db(id: ObjectId, title: String) -> Self {
        Self {
            id: Some(id),
            title,
        }
    }
}

#[derive(Serialize, Default)]
pub struct SingleBookResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
}

impl From<Book> for SingleBookResponse {
    fn from(value: Book) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            title: value.title,
        }
    }
}
