mod config;
mod engine;
mod input;
mod ui;

use config::Config;
use crossbeam_channel::unbounded;
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let config = Config::load_or_default("config.toml");

    let shared_state = Arc::new(RwLock::new(engine::EngineState::Idle));
    let shared_params = Arc::new(RwLock::new(engine::ScanParams::default()));

    let (cmd_sender, cmd_receiver) = unbounded();
    let cmd_sender_clone = cmd_sender.clone();

    let engine_state = Arc::clone(&shared_state);
    let engine_params = Arc::clone(&shared_params);

    // Spawn the engine thread
    thread::spawn(move || {
        engine::run_engine(cmd_receiver, engine_state, engine_params);
    });

    // Spawn the input listener thread
    let input_config = config.clone();
    thread::spawn(move || {
        input::run_input_listener(input_config, cmd_sender_clone);
    });

    // Run the UI on the main thread
    ui::run_ui(config, shared_state, shared_params);
}
