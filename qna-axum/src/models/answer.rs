use serde::{Deserialize, Serialize};
use crate::models::question::QuestionId;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct AnswerId(String);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}