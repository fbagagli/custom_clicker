# Autoclicker Architecture

## Overview
This application is a Windows desktop auto-clicker with a native GUI and global hotkeys, written in Rust. The project follows a strict modular structure to ensure maintainability, scalability, and separation of concerns.

## Threading Model and Decoupling
The core architectural principle of this application is the **strict decoupling of the UI thread from the Engine (clicking) thread**.

*   **UI Thread**: Runs the `egui` via `eframe`. It must remain responsive at all times and should never block. It handles rendering, configuration updates, and displaying status.
*   **Input Thread**: A background thread dedicated to listening for global keyboard events (using `rdev`). It intercepts hotkeys (e.g., F5 to Start, F6 to Pause, F7 to Stop).
*   **Engine Thread**: A background thread responsible for executing the actual mouse simulation (using `enigo`).

### Communication
These threads communicate asynchronously using message passing, specifically via `crossbeam-channel`.
- The **Input Thread** sends commands (`Start`, `Pause`, `Stop`) to the **Engine Thread**.
- (Future) The **UI Thread** may also send configuration updates or manual commands to the Engine Thread, and receive status updates from it.

### Engine State Machine
The Engine operates as a simple state machine:
*   **`Idle`**: The engine is stopped. It blocks entirely, waiting for a command via the channel.
*   **`Running`**: The engine is actively simulating clicks. It uses `try_recv()` to poll for commands without blocking the clicking loop.
*   **`Paused`**: The engine is temporarily suspended. Similar to `Idle`, it waits for a command (like `Start` or `Stop`).

## Folder Structure
*   `src/ui/`: Contains all `egui` and `eframe` related code.
*   `src/engine/`: Contains the state machine, clicking logic (`enigo`), and the main event loop for the Engine.
*   `src/input/`: Contains the global hotkey listener (`rdev`).
*   `src/config.rs`: Manages configuration loading and saving using `serde` and `toml`.
*   `src/main.rs`: The minimalist entry point that wires the channels, spawns the threads, and starts the application.
