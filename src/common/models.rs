use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GameCategory {
    Warmup,
    Casual,
    Spicy,
    Dangerous,
    Ladies,
    Boys,
}
