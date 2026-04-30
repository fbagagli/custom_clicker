use crate::engine::EngineCommand;
use rdev::{listen, Event, EventType, Key};

pub fn run_input_listener(cmd_sender: crossbeam_channel::Sender<EngineCommand>) {
    let callback = move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            match key {
                Key::F5 => {
                    let _ = cmd_sender.send(EngineCommand::Start);
                }
                Key::F6 => {
                    let _ = cmd_sender.send(EngineCommand::Pause);
                }
                Key::F7 => {
                    let _ = cmd_sender.send(EngineCommand::Stop);
                }
                _ => {}
            }
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
