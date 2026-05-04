use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub start_hotkey: Vec<String>,
    pub pause_hotkey: Vec<String>,
    pub stop_hotkey: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            start_hotkey: vec!["F10".to_string()],
            pause_hotkey: vec!["F11".to_string()],
            stop_hotkey: vec!["F12".to_string()],
        }
    }
}

impl Config {
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&contents) {
                return config;
            }
        }

        let config = Config::default();
        config.save(path);
        config
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        if let Ok(contents) = toml::to_string(self) {
            let _ = fs::write(path, contents);
        }
    }
}
