use crate::LampFM;
use eframe::egui::Ui;
use egui::{Id, Modal};

impl LampFM {
    pub(crate) fn draw_action_modal(&mut self, ui: &Ui) {
        if let Some(action) = &mut self.action_context {
            // let is_marked = self.marked_files.contains(&action.path);
            let mut closed = false;

            Modal::new(Id::new(&action.path)).show(ui.ctx(), |ui| {
                ui.set_min_width(0.0);
                ui.set_min_height(0.0);

                ui.vertical_centered(|ui| {
                    ui.label(action.path.to_string_lossy());
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut action.rename_input);
                        if ui.button("rename").clicked() {
                            let mut new_path = action.path.clone();
                            new_path.pop();
                            new_path.push(&action.rename_input);

                            match std::fs::rename(&action.path, new_path) {
                                Ok(_) => {
                                    closed = true;
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            }
                        }
                    });

                    if ui.button("close").clicked() {
                        closed = true;
                    }
                });
            });

            if closed {
                self.refresh();
                self.action_context = None;
            }
        }
    }
}
