use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::bson::doc;
use std::{str::FromStr, sync::Arc};

use crate::{
    error::ApiError,
    persistence,
    types::book::{Book, SingleBookResponse},
    AppState,
};

pub async fn get_book_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<SingleBookResponse>), (StatusCode, String)> {
    let res = handle(state.mongo_client.clone(), id).await;
    match res {
        Ok((status, book_resp)) => Ok((status, Json(book_resp))),
        Err(err) => Err((err.0, err.1.to_string())),
    }
}

async fn handle(
    c: Arc<mongodb::Client>,
    id: String,
) -> Result<(StatusCode, SingleBookResponse), (StatusCode, ApiError)> {
    let book: Result<Book, ApiError> = get_book_by_id(c, &id).await;

    match book {
        Ok(b) => Ok((StatusCode::OK, SingleBookResponse::from(b))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

async fn get_book_by_id<'a>(c: Arc<mongodb::Client>, id: &str) -> Result<Book, ApiError> {
    let oid = match mongodb::bson::oid::ObjectId::from_str(id) {
        Ok(oid) => oid,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    let book_coll: mongodb::Collection<Book> = c
        .clone()
        .database(persistence::DB_NAME)
        .collection(persistence::BOOK_COLL);
    let res_book = book_coll.find_one(doc! {"_id": oid}).await;

    let maybe_book = match res_book {
        Ok(b) => b,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    match maybe_book {
        Some(b) => Ok(b),
        None => Err(ApiError::NotFoundError),
    }
}
