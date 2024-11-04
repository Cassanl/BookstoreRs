use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
}

impl Book {
    pub fn new(title: String) -> Self {
        Self { id: String::new(), title }
    }
}