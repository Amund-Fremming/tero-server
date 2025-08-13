use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::GameCategory;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Spinner {
    pub id: Uuid,
    pub host_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category: GameCategory,
    pub iterations: i32,
    pub times_played: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpinnerRequest {
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
pub struct SpinnerPlayer {
    spinner_id: i32,
    user_id: i32,
    times_choosen: u8,
}

impl SpinnerPlayer {
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
pub struct SpinnerSession {
    // metadata
    // players
    // rounds
}

impl SpinnerSession {
    pub fn from_request(req: CreateSpinnerRequest) -> Self {
        todo!();
        Self {}
    }

    pub fn from_db(spinner: Spinner, rounds: Vec<Round>) -> Self {
        todo!();
        Self {}
    }
}
