use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Hash)]
pub enum GameType {
    Quiz,
    Spinner,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct PagedRequest {
    category: Option<GameCategory>,
    page_num: u32,
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
    id: i32,
    name: String,
    description: Option<String>,
    category: GameCategory,
    iterations: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedResponse {
    games: Vec<GameBase>,
}
