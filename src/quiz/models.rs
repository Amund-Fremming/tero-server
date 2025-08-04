use serde::{Deserialize, Serialize};

use crate::common::GameCategory;

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    id: i32,
    name: String,
    description: Option<String>,
    category: GameCategory,
    iterations: u8,
    current_iteration: u8,
}

impl Quiz {
    pub fn new(name: &str, description: Option<String>, category: GameCategory) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            description: description,
            category,
            iterations: 0,
            current_iteration: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    id: i32,
    quiz_id: i32,
    title: String,
}

impl Question {
    pub fn new(quiz_id: i32, title: &str) -> Self {
        Self {
            id: 0,
            quiz_id,
            title: title.to_string(),
        }
    }
}
