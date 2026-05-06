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

#[derive(Debug, Clone, Copy)]
pub struct ScanParams {
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
    pub y_step_pixels: i32,
    pub x_shift_pixels: i32,
    pub total_time_seconds: f64,
}

impl Default for ScanParams {
    fn default() -> Self {
        Self {
            start_x: 100,
            start_y: 100,
            end_x: 200,
            end_y: 200,
            y_step_pixels: 20,
            x_shift_pixels: 20,
            total_time_seconds: 5.0,
        }
    }
}

pub mod zigzag;

pub struct StateMachine {
    pub state: EngineState,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: EngineState::Idle,
        }
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

use std::sync::{Arc, RwLock};

pub fn run_engine(
    cmd_receiver: crossbeam_channel::Receiver<EngineCommand>,
    shared_state: Arc<RwLock<EngineState>>,
    shared_params: Arc<RwLock<ScanParams>>,
) {
    let mut state_machine = StateMachine::new();
    let mut enigo = enigo::Enigo::new(&enigo::Settings::default()).unwrap();

    loop {
        // Handle IDLE state (blocking wait)
        if state_machine.state == EngineState::Idle {
            match cmd_receiver.recv() {
                Ok(cmd) => {
                    let mut last_command = cmd;
                    while let Ok(c) = cmd_receiver.try_recv() {
                        last_command = c;
                    }
                    state_machine.apply(last_command);
                    if let Ok(mut state) = shared_state.write() {
                        *state = state_machine.state;
                    }
                }
                Err(_) => break, // Disconnected
            }
            continue;
        }

        if state_machine.state == EngineState::Running || state_machine.state == EngineState::Paused
        {
            // Read latest params at the start of a cycle
            let current_params = {
                let params_lock = shared_params.read().unwrap();
                *params_lock
            };

            // Precompute points for this cycle.
            let points = zigzag::generate_cycle_points(&current_params);
            if points.is_empty() {
                // Failsafe to prevent divide by zero
                state_machine.state = EngineState::Idle;
                continue;
            }

            let num_points = points.len();
            let dt = if num_points > 1 {
                std::time::Duration::from_secs_f64(
                    current_params.total_time_seconds / (num_points - 1) as f64,
                )
            } else {
                std::time::Duration::from_secs_f64(current_params.total_time_seconds)
            };

            let mut cycle_start_time = std::time::Instant::now();
            let mut i = 0;

            while i < num_points {
                // Drain messages
                let mut last_command = None;
                while let Ok(cmd) = cmd_receiver.try_recv() {
                    last_command = Some(cmd);
                }
                if let Some(cmd) = last_command {
                    state_machine.apply(cmd);
                    if let Ok(mut state) = shared_state.write() {
                        *state = state_machine.state;
                    }
                }

                if state_machine.state == EngineState::Idle {
                    // Stopped midway, abort current cycle.
                    break;
                }

                if state_machine.state == EngineState::Paused {
                    let pause_start = std::time::Instant::now();
                    // Block until unpaused or stopped
                    if let Ok(cmd) = cmd_receiver.recv() {
                        let mut last_cmd = cmd;
                        while let Ok(c) = cmd_receiver.try_recv() {
                            last_cmd = c;
                        }
                        state_machine.apply(last_cmd);
                        if let Ok(mut state) = shared_state.write() {
                            *state = state_machine.state;
                        }

                        if state_machine.state == EngineState::Idle {
                            break;
                        }
                        // Resume: shift the cycle start time by the pause duration
                        cycle_start_time += pause_start.elapsed();
                    } else {
                        break; // Disconnected
                    }
                    continue; // Re-evaluate state at this point index
                }

                if state_machine.state == EngineState::Running {
                    let target_time = cycle_start_time + dt * i as u32;
                    let now = std::time::Instant::now();

                    if target_time > now {
                        std::thread::sleep(target_time - now);
                    }

                    // Move mouse and click
                    let (x, y) = points[i];
                    let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
                    let _ = enigo.button(enigo::Button::Left, enigo::Direction::Click);

                    i += 1;
                }
            }
        }
    }
}
