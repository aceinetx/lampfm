mod content;
mod top_bar;

use crate::LampFM;

impl eframe::App for LampFM {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.draw_top_bar(ui);
            self.draw_content(ui);
        });
    }
}
