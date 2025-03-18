use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::Method;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(Any)
        .max_age(Duration::from_secs(60) * 10)
}
