mod middlewares;
mod models;
mod services;

use crate::models::question::Question;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let store = services::store::Store::new();

    let app = Router::new()
        .route("/", get(root))
        .route("/questions", get(get_questions))
        .layer(middlewares::cors::cors())
        .with_state(store.clone());

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Server is running..."
}

async fn get_questions(State(store): State<services::store::Store>) -> Json<Vec<Question>> {
    let questions = store.questions.read().await
        .values()
        .cloned()
        .collect();

    Json(questions)
}
