#[derive(Debug)]
pub enum ApiError {
    NotFoundError,
    InternalServerError,
    DupEntityError,
    TextTooLong(String),
    ParamNullError(String),
}

impl ToString for ApiError {
    fn to_string(&self) -> String {
        match self {
            ApiError::NotFoundError => "NotFound".to_owned(),
            ApiError::InternalServerError => "InternalServerError".to_owned(),
            ApiError::DupEntityError => "DuplicateEntity".to_owned(),
            ApiError::TextTooLong(_) => "TextTooLong".to_owned(),
            ApiError::ParamNullError(_) => "ParamNull".to_owned(),
        }
    }
}

// impl IntoResponse for ApiError {
//     fn into_response(self) -> axum::response::Response {
//         let status: StatusCode = match self {
//             ApiError::NotFoundError => StatusCode::NOT_FOUND,
//             ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
//             ApiError::DupEntityError => StatusCode::BAD_REQUEST,
//         };
//         let error: ErrorResponse = self.into();
//         (status, error).into_response()
//     }
// }
