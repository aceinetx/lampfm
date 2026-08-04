use crate::{LampFM, SortKind};
use egui_material_icons::icons::{ICON_ARROW_UPWARD, ICON_HOME, ICON_REFRESH};
use std::mem::take;
use std::path::PathBuf;

impl LampFM {
    pub(crate) fn draw_top_bar(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button(ICON_HOME).clicked() {
                self.change_to_home_dir();
            }

            if ui.button(ICON_ARROW_UPWARD).clicked()
                && let Some(path) = self.current_path.parent()
            {
                self.change_dir(path.to_path_buf());
            }

            if ui.button(ICON_REFRESH).clicked() {
                self.refresh();
            }

            let mut path_str = self.current_path.to_str().unwrap_or_default().to_string();
            let resp =
                ui.add(egui::TextEdit::singleline(&mut path_str).desired_width(f32::INFINITY));
            if resp.lost_focus() && resp.changed() {
                self.change_dir(PathBuf::from(path_str));
            }
        });
        ui.horizontal(|ui| {
            ui.label("sort by");
            if ui.button("name").clicked() {
                self.sort_by = SortKind::Name;
                self.refresh();
            }
            if ui.button("time").clicked() {
                self.sort_by = SortKind::Time;
                self.refresh();
            }
            ui.label("files");
            if ui.button("paste").clicked() {
                for file in self.marked_files.iter() {
                    if file.is_dir() {
                        continue;
                    }
                    let dest_path = self.current_path.join(file.file_name().unwrap());
                    if let Err(e) = std::fs::copy(file, dest_path) {
                        eprintln!("{}", e);
                    }
                }
                self.refresh();
            }
            if ui.button("move").clicked() {
                let marked_files = take(&mut self.marked_files);
                for file in marked_files.iter() {
                    let dest_path = self.current_path.join(file.file_name().unwrap());
                    if let Err(e) = std::fs::rename(file, &dest_path) {
                        eprintln!("{}", e);
                    } else {
                        self.marked_files.mark(dest_path);
                    }
                }
                self.refresh();
            }
            if ui.button("remove").clicked() {
                let marked_files = take(&mut self.marked_files);
                for file in marked_files.iter() {
                    if file.is_dir() {
                        continue;
                    }
                    if let Err(e) = std::fs::remove_file(file) {
                        eprintln!("{}", e);
                    }
                }
                self.refresh();
            }
            if ui.button("unmark all").clicked() {
                self.marked_files.clear();
            }
        });
    }
}
