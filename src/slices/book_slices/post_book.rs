use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{error::ApiError, persistence::{BOOK_COLL, DB_NAME}, types::book::Book, AppState};

const MAX_TITLE_LEN: usize = 256;

#[derive(Deserialize)]
pub struct InsertBookRequest {
    pub title: String,
}

impl InsertBookRequest {
    pub fn validate(&self) -> Option<ApiError> {
        if self.title.len() > MAX_TITLE_LEN {
            return Some(ApiError::TextTooLong("title".to_owned()));
        }
        None
    }
}

pub async fn insert_book_handler(
    State(state): State<AppState>,
    Json(req): Json<InsertBookRequest>,
) -> Result<(StatusCode, Json<Book>), String> {
    let res = handle(state.mongo_client, req).await;
    match res {
        Ok((status, book)) => Ok((status, Json(book))),
        Err(err) => Err(err.to_string()),
    }
}

async fn handle(c: Arc<mongodb::Client>, req: InsertBookRequest) -> Result<(StatusCode, Book), ApiError> {
    if let Some(err) = req.validate() {
        return Err(err);
    }

    let book: Book = Book::new(req.title);
    let res = insert_book(c, book).await;

    match res {
        Ok(b) => Ok((StatusCode::OK, b)),
        Err(err) => Err(err),
    }
}

async fn insert_book(
    c: Arc<mongodb::Client>,
    b: Book,
) -> Result<Book, ApiError>{
    let book_coll: mongodb::Collection<&Book> = c
    .clone()
    .database(DB_NAME)
    .collection(BOOK_COLL);

    let result = book_coll.insert_one(&b).await;

    match result {
        Ok(res) => {
            let res_id = res.inserted_id.to_string();
            let mut book: Book = Book::new(b.title);
            book.id = res_id;
            Ok(book)
        },
        Err(_) => Err(ApiError::InternalServerError)
    }
}