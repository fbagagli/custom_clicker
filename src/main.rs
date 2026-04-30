mod config;
mod engine;
mod input;
mod ui;

use crossbeam_channel::unbounded;
use std::thread;

fn main() {
    let (cmd_sender, cmd_receiver) = unbounded();

    let cmd_sender_clone = cmd_sender.clone();

    // Spawn the engine thread
    thread::spawn(move || {
        engine::run_engine(cmd_receiver);
    });

    // Spawn the input listener thread
    thread::spawn(move || {
        input::run_input_listener(cmd_sender_clone);
    });

    // Run the UI on the main thread
    ui::run_ui();
}
