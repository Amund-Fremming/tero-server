use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::models::GameCategory;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Spin {
    pub id: Uuid,
    pub host_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category: GameCategory,
    pub iterations: i32,
    pub times_played: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Round {
    id: Uuid,
    spinner_id: i32,
    participants: i32,
    read_before: bool,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinSession {
    // metadata
    // players
    // rounds
}

impl SpinSession {
    pub fn from_game_and_rounds(spinner: Spin, rounds: Vec<Round>) -> Self {
        todo!();
        Self {}
    }
}
