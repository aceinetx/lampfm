mod action_modal;
mod content;
mod side_bar;
mod top_bar;

use eframe::egui::Ui;
use egui::Panel;

use crate::LampFM;

impl eframe::App for LampFM {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.draw_top_bar(ui);
            Panel::left("sidebar").show(ui, |ui| {
                self.draw_side_bar(ui);
            });
            self.draw_content(ui);
            self.draw_action_modal(ui);
        });
    }
}
