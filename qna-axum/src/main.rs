mod entities;
mod handlers;
mod middlewares;
mod models;

use handlers::questions;
use log::info;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

use tower::ServiceBuilder;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse},
    LatencyUnit,
};
use tracing::Level;

use axum::body::Body;
use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::{Html, Response},
    routing::{delete, get, post, put},
    Router,
};
use dotenvy::dotenv;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnBodyChunk, OnBodyChunk};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    db: Arc<sea_orm::DatabaseConnection>,
}

#[tokio::main]
async fn main() {
    dotenvy::from_path("./qna-axum/.env").expect("Failed to load .env file");
    init_logging();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = match sea_orm::Database::connect(&database_url).await {
        Ok(db_conn) => {
            info!("Successfully connected to the database");
            db_conn
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/", get(root))
        .route("/questions", get(questions::index))
        .route("/questions", post(questions::create))
        .route("/questions/{id}", get(questions::show))
        .route("/questions/{id}", put(questions::update))
        .route("/questions/{id}", delete(questions::delete))
        .layer(middlewares::cors::cors())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Server is running..."
}

fn init_logging() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
            "{}=debug,tower_http=trace,axum::rejection=trace",
            env!("CARGO_PKG_NAME")
        )
        .into()
    });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
