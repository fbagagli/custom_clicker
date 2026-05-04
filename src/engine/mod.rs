use enigo::Mouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    Idle,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineCommand {
    Start,
    Pause,
    Stop,
}

pub struct StateMachine {
    pub state: EngineState,
}

impl StateMachine {
    pub fn new() -> Self {
        Self { state: EngineState::Idle }
    }

    pub fn apply(&mut self, cmd: EngineCommand) {
        match (self.state, cmd) {
            (EngineState::Idle, EngineCommand::Start) => self.state = EngineState::Running,
            (EngineState::Running, EngineCommand::Pause) => self.state = EngineState::Paused,
            (EngineState::Running, EngineCommand::Stop) => self.state = EngineState::Idle,
            (EngineState::Paused, EngineCommand::Start) => self.state = EngineState::Running,
            (EngineState::Paused, EngineCommand::Stop) => self.state = EngineState::Idle,
            // Ignore invalid transitions
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let sm = StateMachine::new();
        assert_eq!(sm.state, EngineState::Idle);
    }

    #[test]
    fn test_valid_transitions_from_idle() {
        let mut sm = StateMachine::new();
        sm.apply(EngineCommand::Start);
        assert_eq!(sm.state, EngineState::Running);

        let mut sm = StateMachine::new();
        sm.apply(EngineCommand::Pause);
        assert_eq!(sm.state, EngineState::Idle); // Invalid transition, stays Idle

        let mut sm = StateMachine::new();
        sm.apply(EngineCommand::Stop);
        assert_eq!(sm.state, EngineState::Idle); // Invalid transition, stays Idle
    }

    #[test]
    fn test_valid_transitions_from_running() {
        let mut sm = StateMachine::new();
        sm.state = EngineState::Running;

        sm.apply(EngineCommand::Pause);
        assert_eq!(sm.state, EngineState::Paused);

        sm.state = EngineState::Running;
        sm.apply(EngineCommand::Stop);
        assert_eq!(sm.state, EngineState::Idle);

        sm.state = EngineState::Running;
        sm.apply(EngineCommand::Start);
        assert_eq!(sm.state, EngineState::Running); // Invalid transition, stays Running
    }

    #[test]
    fn test_valid_transitions_from_paused() {
        let mut sm = StateMachine::new();
        sm.state = EngineState::Paused;

        sm.apply(EngineCommand::Start);
        assert_eq!(sm.state, EngineState::Running);

        sm.state = EngineState::Paused;
        sm.apply(EngineCommand::Stop);
        assert_eq!(sm.state, EngineState::Idle);

        sm.state = EngineState::Paused;
        sm.apply(EngineCommand::Pause);
        assert_eq!(sm.state, EngineState::Paused); // Invalid transition, stays Paused
    }
}

pub fn run_engine(cmd_receiver: crossbeam_channel::Receiver<EngineCommand>) {
    let mut state_machine = StateMachine::new();
    let mut enigo = enigo::Enigo::new(&enigo::Settings::default()).unwrap();

    loop {
        // Drain ALL pending messages from the channel before making a decision.
        // This ensures that if the input thread floods the channel with repeat key events,
        // we process them instantly and react only to the latest command.

        let mut last_command = None;

        if state_machine.state == EngineState::Running {
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

        // Apply the latest command safely through the state machine
        if let Some(cmd) = last_command {
            state_machine.apply(cmd);
        }

        if state_machine.state == EngineState::Running {
            let _ = enigo.button(enigo::Button::Left, enigo::Direction::Click);
            std::thread::sleep(std::time::Duration::from_millis(100)); // Default interval
        }
    }
}
