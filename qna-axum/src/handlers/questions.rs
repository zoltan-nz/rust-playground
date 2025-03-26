use crate::models::question::Question;
use crate::services;
use axum::extract::State;
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
