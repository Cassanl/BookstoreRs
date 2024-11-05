use std::sync::Arc;

use axum::http::StatusCode;
use serde::Deserialize;

use crate::{persistence::{self, BOOK_COLL, DB_NAME}, types::book::Book};

#[derive(Deserialize)]
pub struct UpdateBookRequest {

}

pub async fn put_book_handler() -> Result<(StatusCode), (StatusCode)> {
    todo!()
}

async fn handle() {
    todo!()
}

async fn update_book(c: Arc<mongodb::Client>) {
    let book_coll: mongodb::Collection<Book> = persistence::get_mongo_collection(c.clone(), DB_NAME, BOOK_COLL);

}