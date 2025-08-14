use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::GameCategory;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpinRequest {
    host_id: i32,
    name: String,
    description: Option<String>,
    category: Option<GameCategory>,
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
pub struct SpinPlayer {
    spinner_id: i32,
    user_id: i32,
    times_choosen: u8,
}

impl SpinPlayer {
    pub fn new(spinner_id: i32, user_id: i32) -> Self {
        Self {
            spinner_id,
            user_id,
            times_choosen: 0,
        }
    }

    pub fn inc_times_choosen(&mut self) {
        self.times_choosen = self.times_choosen + 1;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinSession {
    // metadata
    // players
    // rounds
}

impl SpinSession {
    pub fn from_request(req: CreateSpinRequest) -> Self {
        todo!();
        Self {}
    }

    pub fn from_db(spinner: Spin, rounds: Vec<Round>) -> Self {
        todo!();
        Self {}
    }
}
