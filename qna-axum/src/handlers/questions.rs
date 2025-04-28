use crate::entities::answer::Model as AnswerModel;
use crate::entities::prelude::Question;
use crate::entities::question::{Model as QuestionModel, QuestionResponse};
use crate::middlewares::pagination::Pagination;
use crate::models::errors::Error;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, PaginatorTrait, Set};
use serde::Deserialize;
use crate::entities::question;

#[derive(Deserialize)]
pub struct QuestionPayload {
    title: String,
    content: String,
    tags: Vec<String>,
}

pub async fn index(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let (page, per_page) = pagination.get_values();
    let paginator = Question::find().paginate(state.db.as_ref(), per_page as u64);

    match paginator.fetch_page((page - 1) as u64).await {
        Ok(questions) => {
            let response: Vec<QuestionResponse> = questions
                .into_iter()
                .map(QuestionResponse::from)
                .collect();
            Json(response).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch questions. Database error: {e}"),
        )
            .into_response(),
    }
}

// POST /questions
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<QuestionPayload>,
) -> impl IntoResponse {
    let now = chrono::Utc::now();

    let new_question = question::ActiveModel {
        id: NotSet, // Assuming the ID is auto-incremented
        title: Set(payload.title),
        content: Set(payload.content),
        tags: Set(QuestionModel::set_tags(payload.tags)),
        created_at: Set(now.clone()),
        updated_at: Set(now.clone()),
    };
    match new_question.insert(state.db.as_ref()).await {
        Ok(question) => {
            let response = QuestionResponse::from(question);
            (StatusCode::CREATED, Json(response)).into_response()
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create question. Database error: {e}"),
        )
            .into_response(),
    }
}

// GET /questions/:id
pub async fn show(State(store): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match Question::find_by_id(id).one(store.db.as_ref()).await {
        Ok(Some(question)) => {
            let response = QuestionResponse::from(question);
            Json(response).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, Error::QuestionNotFound.to_string()).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch question. Database error: {e}"),
        )
            .into_response(),
    }
}


// PUT /questions/:id
pub async fn update(
    State(store): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<QuestionPayload>,
) -> impl IntoResponse {
    let now = chrono::Utc::now();

    let question_to_update = match Question::find_by_id(id).one(store.db.as_ref()).await {
        Ok(Some(question)) => question,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Error::QuestionNotFound.to_string()).into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch question. Database error: {e}"),
            )
                .into_response()
        }
    };

    let mut active_question: question::ActiveModel = question_to_update.into();

    active_question.title = Set(payload.title);
    active_question.content = Set(payload.content);
    active_question.tags = Set(QuestionModel::set_tags(payload.tags));
    active_question.updated_at = Set(now);

    match active_question.update(store.db.as_ref()).await {
        Ok(updated_question) => {
            let response = QuestionResponse::from(updated_question);
            Json(response).into_response()
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update question. Database error: {e}"),
        )
            .into_response(),
    }
}

// DELETE /questions/:id
pub async fn delete(State(store): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match Question::delete_by_id(id).exec(store.db.as_ref()).await {
        Ok(result) => {
            if result.rows_affected == 0 {
                (StatusCode::NOT_FOUND, Error::QuestionNotFound.to_string()).into_response()
            } else {
                (StatusCode::NO_CONTENT).into_response()
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete question. Database error: {e}"),
        )
            .into_response(),
    }
}
