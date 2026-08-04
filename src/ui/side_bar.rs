use std::path::PathBuf;

use crate::LampFM;

impl LampFM {
    pub(crate) fn draw_side_bar(&mut self, ui: &mut eframe::egui::Ui) {
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
}
