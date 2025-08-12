use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::GameCategory;

#[derive(Debug, Serialize, Deserialize)]
pub struct Spinner {
    id: Uuid,
    host_id: i32,
    name: String,
    description: Option<String>,
    category: GameCategory,
    iterations: i32,
    times_played: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpinnerRequest {
    host_id: i32,
    name: String,
    description: Option<String>,
    category: Option<GameCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    id: Uuid,
    spinner_id: i32,
    participants: u8,
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
    pub fn new() -> Self {
        Self {}
    }
}
