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
        // Try to receive a command, without blocking if we're currently running
        let cmd_res = if state == EngineState::Running {
            cmd_receiver.try_recv()
        } else {
            // Block until we get a command if we are idle or paused
            cmd_receiver.recv().map_err(|_| crossbeam_channel::TryRecvError::Disconnected)
        };

        match cmd_res {
            Ok(EngineCommand::Start) => state = EngineState::Running,
            Ok(EngineCommand::Pause) => state = EngineState::Paused,
            Ok(EngineCommand::Stop) => state = EngineState::Idle,
            Err(crossbeam_channel::TryRecvError::Empty) => {} // No command, continue
            Err(crossbeam_channel::TryRecvError::Disconnected) => break, // Channel closed, exit
        }

        if state == EngineState::Running {
            let _ = enigo.button(enigo::Button::Left, enigo::Direction::Click);
            std::thread::sleep(std::time::Duration::from_millis(100)); // Default interval
        }
    }
}
