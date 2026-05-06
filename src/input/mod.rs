use crate::config::Config;
use crate::engine::EngineCommand;
use rdev::{Event, EventType, Key, listen};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

// Helper function to map rdev::Key to strings defined in Config
fn key_to_string(key: &Key) -> String {
    format!("{:?}", key)
}

pub fn run_input_listener(config: Config, cmd_sender: crossbeam_channel::Sender<EngineCommand>) {
    let pressed_keys: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    let start_keys: HashSet<String> = config.start_hotkey.into_iter().collect();
    let pause_keys: HashSet<String> = config.pause_hotkey.into_iter().collect();
    let stop_keys: HashSet<String> = config.stop_hotkey.into_iter().collect();

    let callback = move |event: Event| {
        if let Ok(mut keys) = pressed_keys.lock() {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let key_str = key_to_string(&key);
                    // Only process logic if it's a new key press to avoid spamming from OS repeat
                    let is_new = keys.insert(key_str);

                    if is_new {
                        // Check for exact matches instead of superset to prevent overlapping combination issues
                        if !stop_keys.is_empty() && *keys == stop_keys {
                            let _ = cmd_sender.send(EngineCommand::Stop);
                        } else if !pause_keys.is_empty() && *keys == pause_keys {
                            let _ = cmd_sender.send(EngineCommand::Pause);
                        } else if !start_keys.is_empty() && *keys == start_keys {
                            let _ = cmd_sender.send(EngineCommand::Start);
                        }
                    }
                }
                EventType::KeyRelease(key) => {
                    let key_str = key_to_string(&key);
                    keys.remove(&key_str);
                }
                _ => {}
            }
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
