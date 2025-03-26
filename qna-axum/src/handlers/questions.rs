use crate::models::question::Question;
use crate::models::question::QuestionId;
use crate::services;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn index(State(store): State<services::store::Store>) -> Json<Vec<Question>> {
    let questions = store.questions.read().await.values().cloned().collect();

    Json(questions)
}

pub async fn create(
    State(store): State<services::store::Store>,
    Json(question): Json<Question>,
) -> Json<Question> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question.clone());

    Json(question)
}

pub async fn show(
    State(store): State<services::store::Store>,
    Path(question_id): Path<String>,
) -> impl IntoResponse {
    let questions = store.questions.read().await;
    let question_id = QuestionId(question_id);

    match questions.get(&question_id) {
        Some(question) => Json(question.clone()).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
