use std::hash::Hash;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{quiz::models::Quiz, spin::models::Spin};

#[derive(Debug, Serialize, Deserialize, Hash, Clone, sqlx::Type)]
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
    #[serde(rename(deserialize = "default"))]
    Default,
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
            GameCategory::Default => "default",
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

impl From<Spin> for GameBase {
    fn from(value: Spin) -> Self {
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

    pub fn from_spinners(spinners: Vec<Spin>) -> Self {
        Self {
            games: spinners.into_iter().map(|s| s.into()).collect(),
        }
    }
}
