use std::fmt;

use super::question::QuestionId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}

impl fmt::Display for AnswerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "id: {}, content: {}", self.id, self.content)
    }
}
