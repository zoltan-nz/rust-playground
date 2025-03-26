use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct QuestionId(pub String);

impl Default for QuestionId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Question {
    #[serde(default)]
    pub id: QuestionId,
    pub title: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}
