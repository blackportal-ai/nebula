use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
    Command(String),
}
