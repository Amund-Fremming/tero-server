use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{
    quiz::{Question, Quiz},
    spinner::{Round, Spinner, SpinnerPlayer},
};

pub static ACTIVE_QUIZ_CACHE: Lazy<ActiveQuizGames> = Lazy::new(|| ActiveQuizGames::new());
pub static ACTIVE_SPINNER_CACHE: Lazy<ActiveQuizGames> = Lazy::new(|| ActiveQuizGames::new());

/*
 * THOUGHTS
 * - Make this generic service, update by passing in closures of the game and how its updated?
 * - Grandulated control with many functions
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizSession {
    game: Quiz,
    questions: Vec<String>, // For sparing storage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveQuizGames {
    games: RwLock<HashMap<String, QuizSession>>,
}

impl ActiveQuizGames {
    pub fn new() -> Self {
        Self {
            games: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_question(&self, game_name: &str, title: &str) {
        let mut map = self.games.write().unwrap();
        let game = map.get(game_name).unwrap();
        //  update some
        // game.update()
    }

    pub fn read_question(&self, game_name: &str) -> String {
        let map = self.games.read().unwrap();
        let game = map.get(game_name).unwrap();
        // get a question..

        String::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinnerSession {
    game: Spinner,
    rounds: Vec<Round>,
    players: Vec<SpinnerPlayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSpinnerGames {
    games: RwLock<HashMap<u32, Spinner>>,
}

impl ActiveSpinnerGames {
    pub fn new() -> Self {
        Self {
            games: RwLock::new(HashMap::new()),
        }
    }
}
