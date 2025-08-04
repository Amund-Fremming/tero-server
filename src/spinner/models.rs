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
    iterations: u8,
    current_iteration: u8,
}

impl Spinner {
    pub fn new(
        host_id: i32,
        name: &str,
        description: Option<String>,
        category: GameCategory,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            host_id,
            name: name.to_string(),
            description,
            category,
            iterations: 0,
            current_iteration: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    id: Uuid,
    spinner_id: i32,
    participants: u8,
    read_before: bool,
    title: String,
}

impl Round {
    pub fn new(spinner_id: i32, participants: u8, read_before: bool, title: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            spinner_id,
            participants,
            read_before,
            title: title.to_string(),
        }
    }
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
