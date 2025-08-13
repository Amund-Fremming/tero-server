use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    quiz::{Quiz, QuizSession},
    spinner::{Spinner, SpinnerSession},
};

#[derive(Debug, Serialize, Deserialize, Hash, sqlx::Type)]
#[sqlx(type_name = "game_category", rename_all = "lowercase")]
pub enum GameCategory {
    #[serde(rename(deserialize = "warm_up"))]
    Warmup,
    #[serde(rename(deserialize = "casual"))]
    Casual,
    #[serde(rename(deserialize = "spicy"))]
    Spicy,
    #[serde(rename(deserialize = "dangerous"))]
    Dangerous,
    #[serde(rename(deserialize = "ladies"))]
    Ladies,
    #[serde(rename(deserialize = "boys"))]
    Boys,
}

impl GameCategory {
    pub fn as_str(&self) -> &str {
        match self {
            GameCategory::Warmup => "warm_up",
            GameCategory::Casual => "casual",
            GameCategory::Spicy => "spicy",
            GameCategory::Dangerous => "dangerous",
            GameCategory::Ladies => "ladies",
            GameCategory::Boys => "boys",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub enum GameType {
    Quiz,
    Spinner,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct PagedRequest {
    pub category: Option<GameCategory>,
    pub page_num: u32,
}

impl PagedRequest {
    pub fn generate_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GameBase {
    id: Uuid,
    name: String,
    description: Option<String>,
    category: GameCategory,
    iterations: i32,
}

impl From<Quiz> for GameBase {
    fn from(value: Quiz) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            category: value.category,
            iterations: value.iterations,
        }
    }
}

impl From<Spinner> for GameBase {
    fn from(value: Spinner) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            category: value.category,
            iterations: value.iterations,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedResponse {
    games: Vec<GameBase>,
}

impl PagedResponse {
    pub fn from_quizzes(quizzes: Vec<Quiz>) -> Self {
        Self {
            games: quizzes.into_iter().map(|q| q.into()).collect(),
        }
    }

    pub fn from_spinners(spinners: Vec<Spinner>) -> Self {
        Self {
            games: spinners.into_iter().map(|s| s.into()).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GameApiWrapper {
    Quiz(QuizSession),
    Spinner(SpinnerSession),
}
