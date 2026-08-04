use crate::LampFM;
use egui_material_icons::icons::{ICON_ARROW_UPWARD, ICON_HOME};
use std::path::PathBuf;

impl LampFM {
    pub(crate) fn draw_top_bar(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button(ICON_ARROW_UPWARD).clicked()
                && let Some(path) = self.current_path.parent()
            {
                self.change_dir(path.to_path_buf());
            }

            if ui.button(ICON_HOME).clicked() {
                self.change_to_home_dir();
            }

            let mut path_str = self.current_path.to_str().unwrap_or_default().to_string();
            let resp =
                ui.add(egui::TextEdit::singleline(&mut path_str).desired_width(f32::INFINITY));
            if resp.lost_focus() && resp.changed() {
                self.change_dir(PathBuf::from(path_str));
            }
        });
    }
}
