use crate::config::Config;
use eframe::egui;

pub struct AutoclickerApp {
    config: Config,
}

impl AutoclickerApp {
    pub fn new(config: Config) -> Self {
        Self { config }
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

        ui.label(format!("Start: {}", self.config.start_hotkey.join(" + ")));
        ui.label(format!("Pause: {}", self.config.pause_hotkey.join(" + ")));
        ui.label(format!("Stop: {}", self.config.stop_hotkey.join(" + ")));
    }
}

pub fn run_ui(config: Config) {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Autoclicker",
        options,
        Box::new(|_cc| Ok(Box::new(AutoclickerApp::new(config)))),
    ).unwrap();
}
