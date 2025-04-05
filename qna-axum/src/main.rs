mod handlers;
mod middlewares;
mod models;
mod services;

use axum::routing::{delete, get, post, put};
use axum::Router;
use handlers::questions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let store = services::store::Store::new();

    let app = Router::new()
        .route("/", get(root))
        .route("/questions", get(questions::index))
        .route("/questions", post(questions::create))
        .route("/questions/{id}", get(questions::show))
        .route("/questions/{id}", put(questions::update))
        .route("/questions/{id}", delete(questions::delete))
        .layer(middlewares::cors::cors())
        .with_state(store.clone());

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Server is running..."
}
