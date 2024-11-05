use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

impl Default for Book {
    fn default() -> Self {
        Self {
            id: None,
            title: String::new(),
        }
    }
}

#[derive(Serialize)]
pub struct SingleBookResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
}

impl SingleBookResponse {
    pub fn new(id: String, title: String) -> Self {
        Self { id, title }
    }
}

impl From<Book> for SingleBookResponse {
    fn from(value: Book) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            title: value.title,
        }
    }
}

impl Default for SingleBookResponse {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
        }
    }
}
