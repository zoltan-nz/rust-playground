use crate::models::question::QuestionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct AnswerId(String);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}
