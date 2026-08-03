use crate::LampFM;
use imgui::Ui;
use std::path::PathBuf;

impl LampFM {
    pub(crate) fn draw_top_bar(&mut self, ui: &Ui) {
        if ui.button("^")
            && let Some(path) = self.current_path.parent()
        {
            self.change_dir(path.to_path_buf());
        }

        ui.same_line();

        let mut current_path_str = self.current_path.to_str().unwrap().to_string();
        ui.set_next_item_width(-1.0);
        if ui.input_text("##path", &mut current_path_str).build() {
            self.change_dir(PathBuf::from(&current_path_str));
        }
    }
}
