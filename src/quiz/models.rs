use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::GameCategory;

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    id: i32,
    name: String,
    description: Option<String>,
    category: GameCategory,
    iterations: i32,
    times_played: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuizRequest {
    name: String,
    description: Option<String>,
    category: Option<GameCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    id: i32,
    quiz_id: i32,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizSession {
    id: Uuid,
    name: String,
    description: Option<String>,
    category: Option<GameCategory>,
    iterations: u8,
    current_iteration: u8,
    questions: Vec<String>,
}

// Only used for a new game, when stored to the db
impl Into<Quiz> for QuizSession {
    fn into(self) -> Quiz {
        Quiz {
            id: 0,
            name: self.name,
            description: self.description,
            category: self.category.unwrap_or(GameCategory::Casual),
            iterations: self.iterations.into(),
            times_played: 0,
        }
    }
}

impl QuizSession {
    pub fn new(req: CreateQuizRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: req.name,
            description: req.description,
            category: req.category,
            iterations: 0,
            current_iteration: 0,
            questions: Vec::new(),
        }
    }

    pub fn inc_iteration(&mut self) {
        self.current_iteration = self.current_iteration + 1;
    }

    pub fn add_question(&mut self, title: String) {
        self.questions.push(title);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.questions.shuffle(&mut rng);
    }
}
