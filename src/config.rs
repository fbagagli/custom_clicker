use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub start_hotkey: String,
    pub pause_hotkey: String,
    pub stop_hotkey: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_hotkey: "F5".to_string(),
            pause_hotkey: "F6".to_string(),
            stop_hotkey: "F7".to_string(),
        }
    }
}
