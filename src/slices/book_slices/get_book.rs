use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::bson::doc;
use serde::Serialize;
use std::sync::Arc;

use crate::{
    error::ApiError,
    persistence::{self, DBMap},
    types::book::Book,
    AppState,
};

#[derive(Serialize)]
pub struct GetBookResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub err: Option<String>,
}

impl From<Book> for GetBookResponse {
    fn from(value: Book) -> Self {
        Self {
            id: value.id,
            title: value.title,
            err: None,
        }
    }
}

pub async fn get_book_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<GetBookResponse>) {
    let (status, book_resp) = handle(state.mongo_client.clone(), id).await;
    (status, Json(book_resp))
}

async fn handle(c: Arc<mongodb::Client>, id: String) -> (StatusCode, GetBookResponse) {
    let book: Result<Book, ApiError> = get_book_by_id(
        c,
        DBMap {
            key: "_id",
            value: id,
        },
    )
    .await;

    match book {
        Ok(b) => (StatusCode::OK, GetBookResponse::from(b)),
        Err(err) => panic!("{:?}", err.to_string()),
    }
}

async fn get_book_by_id<'a>(
    c: Arc<mongodb::Client>,
    filter: DBMap<'a, String>,
) -> Result<Book, ApiError> {
    let book_coll: mongodb::Collection<Book> = c
        .clone()
        .database(persistence::DB_NAME)
        .collection(persistence::BOOK_COLL);
    let res_book = book_coll.find_one(doc! {filter.key: filter.value}).await;

    let maybe_book = match res_book {
        Ok(b) => b,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    match maybe_book {
        Some(b) => Ok(b),
        None => Err(ApiError::NotFoundError),
    }
}
