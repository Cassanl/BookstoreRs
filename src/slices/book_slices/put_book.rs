use std::{str::FromStr, sync::Arc};

use axum::{extract::State, http::StatusCode, Json};
use mongodb::bson::doc;
use serde::Deserialize;

use crate::{
    error::ApiError,
    persistence::{self, BOOK_COLL, DB_NAME},
    types::{self, book::Book},
    AppState,
};

#[derive(Deserialize)]
pub struct UpdateBookRequest {
    pub id: String,
    pub title: String,
}

impl UpdateBookRequest {
    pub fn validate(&self) -> Option<ApiError> {
        if self.title.len() > types::book::MAX_TITLE_LEN {
            return Some(ApiError::TextTooLong("title".to_owned()));
        }
        None
    }
}

impl From<UpdateBookRequest> for mongodb::bson::Document {
    fn from(value: UpdateBookRequest) -> Self {
        doc! {
            "$set":
            doc! {
                "title": value.title
            }
        }
    }
}

pub async fn put_book_handler(
    State(app_state): State<AppState>,
    Json(req): Json<UpdateBookRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = handle(app_state.mongo_client, req).await;
    match result {
        Ok(status) => Ok(status),
        Err((status, err)) => Err((status, err.to_string())),
    }
}

async fn handle(
    c: Arc<mongodb::Client>,
    req: UpdateBookRequest,
) -> Result<StatusCode, (StatusCode, ApiError)> {
    if let Some(err) = req.validate() {
        return Err((StatusCode::BAD_REQUEST, err));
    }
    let result = update_book(c, req).await;
    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

async fn update_book(c: Arc<mongodb::Client>, b: UpdateBookRequest) -> Result<(), ApiError> {
    let book_coll: mongodb::Collection<Book> =
        persistence::get_mongo_collection(c.clone(), DB_NAME, BOOK_COLL);

    let oid = match mongodb::bson::oid::ObjectId::from_str(&b.id) {
        Ok(oid) => oid,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    let update_result = book_coll
        .update_one(doc! {"_id": oid}, mongodb::bson::Document::from(b))
        .await;

    let result = match update_result {
        Ok(val) => val,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    if result.matched_count == 0 {
        return Err(ApiError::InternalServerError);
    }

    // if result.modified_count == 0 {
    //     return Err(ApiError::InternalServerError);
    // }

    Ok(())
}
