pub mod config;
pub mod error;
mod middlewares;
pub mod persistence;
mod slices;
mod types;

use axum::{
    routing::{get, post, put},
    Router,
};
use config::{AppConfig, Environment};
use persistence::DB_NAME;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config: AppConfig = AppConfig::default();
    let env_lvl: Environment = Environment::from(config.server_conf.level.as_str());

    let mongo_client: mongodb::Client = persistence::init_mongodb(&config.mongodb_conf.uri)
        .await
        .unwrap();

    if env_lvl == Environment::Dev {
        println!("[DEV] db sync");
        mongo_client.database(DB_NAME).drop().await.unwrap();
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
    Router::new()
        .route("/ping", get(|| async { "ping" }))
        .nest(
            "/books",
            Router::new()
                .route("/:id", get(slices::book_slices::get_book::get_book_handler))
                .route("/", post(slices::book_slices::post_book::post_book_handler))
                .route("/", put(slices::book_slices::put_book::put_book_handler)),
        )
        .with_state(app_state)
}
