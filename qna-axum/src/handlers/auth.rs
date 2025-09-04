use axum::{
    extract::State,
    http::{header, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse, Response},
    Json,
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

// Simple demo users; in real app fetch from DB and store password hashes
static USERNAME: &str = "admin";
static PASSWORD: &str = "password";

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

// Render a minimal login page
pub async fn login_page() -> impl IntoResponse {
    Html(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Login</title>
    <style>
      body { font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial; padding: 2rem; }
      form { max-width: 360px; margin: 0 auto; display: grid; gap: 0.75rem; }
      input { padding: .5rem .6rem; font-size: 1rem; }
      button { padding: .6rem .8rem; font-size: 1rem; }
    </style>
  </head>
  <body>
    <h1>Login</h1>
    <form id="login-form">
      <input name="username" placeholder="Username" required />
      <input name="password" type="password" placeholder="Password" required />
      <button type="submit">Sign in</button>
    </form>
    <script>
      const form = document.getElementById('login-form');
      form.addEventListener('submit', async (e) => {
        e.preventDefault();
        const fd = new FormData(form);
        const payload = { username: fd.get('username'), password: fd.get('password') };
        const res = await fetch('/login', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
        if (res.ok) {
          alert('Logged in');
          location.href = '/';
        } else {
          alert('Invalid credentials');
        }
      });
    </script>
  </body>
 </html>"#,
    )
}

// Login API: issues JWT as HttpOnly cookie
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let _db = Arc::clone(&state.db); // reserved for future lookup

    if payload.username != USERNAME || payload.password != PASSWORD {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(8))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: payload.username,
        exp,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string());
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    .unwrap();

    let cookie = format!(
        "token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax",
        token,
        8 * 60 * 60
    );

    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)]
    )
        .into_response()
}

// Simple auth extractor for protected endpoints
use jsonwebtoken::{decode, DecodingKey, Validation};

// Middleware to require authentication on specific routes
pub async fn require_auth(req: axum::http::Request<axum::body::Body>, next: Next) -> Response {
    let headers = req.headers();
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string());

    // Try Bearer token first
    if let Some(value) = headers.get(header::AUTHORIZATION) {
        if let Ok(s) = value.to_str() {
            if let Some(token) = s.strip_prefix("Bearer ") {
                if decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                )
                .is_ok()
                {
                    return next.run(req).await;
                }
            }
        }
    }

    // Fallback to cookie named `token`
    if let Some(cookie_hdr) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_hdr.to_str() {
            if let Some(token) = cookie_str
                .split(';')
                .map(|s| s.trim())
                .find_map(|kv| kv.strip_prefix("token="))
            {
                if decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                )
                .is_ok()
                {
                    return next.run(req).await;
                }
            }
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}


