use crate::models::{answer::Answer, question::{Question, QuestionId}};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::answer::AnswerId;

#[derive(Debug, Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::read_mock_from_json())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn read_mock_from_json() -> HashMap<QuestionId, Question> {
        let file = include_str!("../../questions.json");
        serde_json::from_str(file).expect("Cannot read questions.json")
    }
}