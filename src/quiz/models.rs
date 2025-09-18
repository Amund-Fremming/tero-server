use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::models::GameCategory;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Quiz {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: GameCategory,
    pub iterations: i32,
    pub times_played: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuizRequest {
    name: String,
    description: Option<String>,
    category: Option<GameCategory>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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
    category: GameCategory,
    iterations: u8,
    current_iteration: u8,
    questions: Vec<String>,
}

// Only used for a new game, when stored to the db
impl Into<Quiz> for QuizSession {
    fn into(self) -> Quiz {
        Quiz {
            id: Uuid::new_v4(),
            name: self.name,
            description: self.description,
            category: self.category,
            iterations: self.iterations.into(),
            times_played: 0,
        }
    }
}

impl QuizSession {
    pub fn from_request(req: CreateQuizRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: req.name,
            description: req.description,
            category: req.category.unwrap_or(GameCategory::Casual),
            iterations: 0,
            current_iteration: 0,
            questions: Vec::new(),
        }
    }

    pub fn from_db(quiz: Quiz, mut questions: Vec<Question>) -> Self {
        Self {
            id: quiz.id,
            name: quiz.name,
            description: quiz.description,
            category: quiz.category,
            iterations: u8::try_from(quiz.iterations).ok().unwrap(),
            current_iteration: 0,
            questions: questions.iter_mut().map(|q| q.title.clone()).collect(),
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
