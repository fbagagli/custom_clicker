use enigo::Mouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    Idle,
    Running,
    Paused,
}

pub enum EngineCommand {
    Start,
    Pause,
    Stop,
}

pub fn run_engine(cmd_receiver: crossbeam_channel::Receiver<EngineCommand>) {
    let mut state = EngineState::Idle;
    let mut enigo = enigo::Enigo::new(&enigo::Settings::default()).unwrap();

    loop {
        // Drain ALL pending messages from the channel before making a decision.
        // This ensures that if the input thread floods the channel with repeat key events,
        // we process them instantly and react only to the latest command.

        let mut last_command = None;

        if state == EngineState::Running {
            // If running, we don't want to block. Try to read all available messages.
            while let Ok(cmd) = cmd_receiver.try_recv() {
                last_command = Some(cmd);
            }
        } else {
            // If idle or paused, we block until we receive AT LEAST one message.
            match cmd_receiver.recv() {
                Ok(cmd) => {
                    last_command = Some(cmd);
                    // Then drain any other pending messages that arrived at the exact same time
                    while let Ok(cmd) = cmd_receiver.try_recv() {
                        last_command = Some(cmd);
                    }
                }
                Err(_) => break, // Disconnected
            }
        }

        // Apply the latest command
        if let Some(cmd) = last_command {
            match cmd {
                EngineCommand::Start => state = EngineState::Running,
                EngineCommand::Pause => state = EngineState::Paused,
                EngineCommand::Stop => state = EngineState::Idle,
            }
        }

        if state == EngineState::Running {
            let _ = enigo.button(enigo::Button::Left, enigo::Direction::Click);
            std::thread::sleep(std::time::Duration::from_millis(100)); // Default interval
        }
    }
}
