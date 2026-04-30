use eframe::egui;

pub struct AutoclickerApp {
    // Basic state or channels can be stored here to communicate with engine
}

impl Default for AutoclickerApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for AutoclickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // eframe 0.34
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui, _frame);
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Autoclicker");
        ui.label("F5 to Start, F6 to Pause, F7 to Stop");
    }
}

pub fn run_ui() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Autoclicker",
        options,
        Box::new(|_cc| Ok(Box::new(AutoclickerApp::default()))),
    ).unwrap();
}
