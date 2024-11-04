pub mod error;
mod middlewares;
pub mod persistence;
mod slices;
mod types;

use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use persistence::DB_NAME;
use tokio::net::TcpListener;

#[derive(PartialEq)]
enum Environment {
    Prod,
    Dev
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        match value {
            "PROD" => Self::Prod,
            "DEV" => Self::Dev,
            _ => Self::Dev,
        }
    }
}

#[tokio::main]
async fn main() {
    let mongo_client: mongodb::Client = persistence::init_mongodb().await.unwrap();

    let env: Environment = match std::env::var("ENV") {
        Ok(val) => Environment::from(val.as_str()),
        Err(_) => Environment::Dev
    };

    if env == Environment::Dev {
        println!("[DEV] db sync");
        _ = mongo_client.database(DB_NAME).drop();
    }

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app_state: AppState = AppState::new(mongo_client);
    axum::serve(listener, app(app_state)).await.unwrap()
}

#[derive(Clone)]
pub struct AppState {
    pub mongo_client: Arc<mongodb::Client>,
}

impl AppState {
    pub fn new(mongo_client: mongodb::Client) -> Self {
        Self {
            mongo_client: Arc::new(mongo_client),
        }
    }
}

fn app(app_state: AppState) -> Router {
    let api_router = Router::new()
        .route("/ping", get(|| async { "ping" }))
        .nest(
            "/books",
            Router::new()
            .route("/:id", get(slices::book_slices::get_book::get_book_handler))
            .route("/", post(slices::book_slices::post_book::insert_book_handler)),
        )
        .with_state(app_state);

    api_router
}
