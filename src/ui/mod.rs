use crate::config::Config;
use crate::engine::{EngineState, ScanParams};
use eframe::egui;
use std::sync::{Arc, RwLock};

pub struct AutoclickerApp {
    config: Config,
    shared_state: Arc<RwLock<EngineState>>,
    shared_params: Arc<RwLock<ScanParams>>,
}

impl AutoclickerApp {
    pub fn new(
        config: Config,
        shared_state: Arc<RwLock<EngineState>>,
        shared_params: Arc<RwLock<ScanParams>>,
    ) -> Self {
        Self {
            config,
            shared_state,
            shared_params,
        }
    }
}

impl eframe::App for AutoclickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui, _frame);
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Autoclicker");

        // Display current engine state
        let current_state = {
            if let Ok(state) = self.shared_state.read() {
                format!("{:?}", *state)
            } else {
                "Unknown".to_string()
            }
        };
        ui.label(format!("Engine State: {}", current_state));

        ui.separator();

        // Parameter inputs
        let mut params = {
            if let Ok(p) = self.shared_params.read() {
                *p
            } else {
                ScanParams::default()
            }
        };

        ui.horizontal(|ui| {
            ui.label("Total Time (s):");
            ui.add(egui::DragValue::new(&mut params.total_time_seconds).range(0.1..=10000.0));
        });

        ui.horizontal(|ui| {
            ui.label("Vertical Step (px):");
            ui.add(egui::DragValue::new(&mut params.y_step_pixels).range(1..=10000));
        });

        ui.horizontal(|ui| {
            ui.label("Horizontal Shift (px):");
            ui.add(egui::DragValue::new(&mut params.x_shift_pixels).range(1..=10000));
        });

        // Write parameters back if changed
        if let Ok(mut p) = self.shared_params.write() {
            *p = params;
        }

        ui.separator();

        if ui.button("Select Screen Area").clicked() {
            println!("Select Screen Area clicked");
        }

        ui.separator();

        ui.label(format!("Start: {}", self.config.start_hotkey.join(" + ")));
        ui.label(format!("Pause: {}", self.config.pause_hotkey.join(" + ")));
        ui.label(format!("Stop: {}", self.config.stop_hotkey.join(" + ")));

        // Request a repaint to update the state continuously
        ui.ctx().request_repaint();
    }
}

pub fn run_ui(
    config: Config,
    shared_state: Arc<RwLock<EngineState>>,
    shared_params: Arc<RwLock<ScanParams>>,
) {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Autoclicker",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(AutoclickerApp::new(
                config,
                shared_state,
                shared_params,
            )))
        }),
    )
    .unwrap();
}
