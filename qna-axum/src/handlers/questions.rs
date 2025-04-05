use crate::models::question::Question;
use crate::models::question::QuestionId;
use crate::services;
use crate::models::errors::Error;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use tokio::io::AsyncReadExt;

pub async fn index(State(store): State<services::store::Store>) -> impl IntoResponse {
    let questions: Vec<Question> = store.questions.read().await.values().cloned().collect();

    Json(questions).into_response()
}

pub async fn create(
    State(store): State<services::store::Store>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question.clone());

    Json(question).into_response()
}

pub async fn show(
    State(store): State<services::store::Store>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let questions = store.questions.read().await;
    let question_id = QuestionId(id);

    match questions.get(&question_id) {
        Some(question) => Json(question.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, Error::QuestionNotFound.to_string()).into_response(),
    }
}

pub async fn update(
    State(store): State<services::store::Store>,
    Path(id): Path<String>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    let question_id = QuestionId(id);

    match store.questions.write().await.get_mut(&question_id) {
        Some(q) => {
            *q = question;
            Json(q.clone()).into_response()
        }
        None => (StatusCode::NOT_FOUND, Error::QuestionNotFound.to_string()).into_response(),
    }
}

pub async fn delete(
    State(store): State<services::store::Store>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let question_id = QuestionId(id);

    match store.questions.write().await.remove(&question_id) {
        Some(_) => StatusCode::NO_CONTENT.into_response(),
        None => (StatusCode::NOT_FOUND, Error::QuestionNotFound.to_string()).into_response(),
    }
}