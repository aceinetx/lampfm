use egui::ScrollArea;
use std::path::PathBuf;

use egui::Key;

use crate::LampFM;

impl LampFM {
    pub(crate) fn draw_side_bar(&mut self, ui: &mut eframe::egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            {
                ui.label("places");
                let mut clicked_place = Option::<PathBuf>::None;
                for place in self.config.places.iter() {
                    if ui.button(&place.0).clicked() {
                        clicked_place = Some(PathBuf::from(&place.1));
                    }
                }
                if let Some(path) = clicked_place {
                    self.change_dir(path);
                }
            }
            {
                ui.label("marked files");
                let mut clicked_path = Option::<PathBuf>::None;
                let mut removed_path = Option::<PathBuf>::None;
                for file in self.marked_files.iter().rev() {
                    let resp = ui.button(file.file_name().unwrap().to_string_lossy());
                    if resp.clicked() {
                        clicked_path = Some(file.parent().unwrap().to_path_buf());
                    }

                    if resp.hovered() && ui.ctx().input(|i| i.key_pressed(Key::Q)) {
                        removed_path = Some(file.clone());
                    }
                }
                if let Some(path) = clicked_path {
                    self.change_dir(path);
                }
                if let Some(path) = removed_path {
                    if let Some(index) = self.marked_files.iter().position(|x| *x == path) {
                        self.marked_files.remove(index);
                    }
                }
            }
        });
    }
}
