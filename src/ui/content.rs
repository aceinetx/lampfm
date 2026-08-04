use crate::{ActionContext, LampFM};
use eframe::egui::Ui;
use egui::{Button, Color32, Key, RichText, ScrollArea};
use egui_material_icons::icons::{ICON_FOLDER, ICON_INSERT_DRIVE_FILE};

use std::fs::DirEntry;

impl LampFM {
    pub(crate) fn draw_content(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            let mut clicked_item: Option<&DirEntry> = None;

            for entry in self.dir_content.iter() {
                let path = entry.path();

                let icon = if path.is_dir() {
                    ICON_FOLDER
                } else {
                    ICON_INSERT_DRIVE_FILE
                };

                let is_marked = self.marked_files.contains(&path);

                let mut text = RichText::new(format!(
                    "{} {}",
                    icon.codepoint,
                    entry.file_name().to_string_lossy()
                ));

                if is_marked {
                    text = text.color(Color32::from_rgb(255, 0, 255));
                }

                let resp = ui.add_sized(
                    egui::vec2(ui.available_width(), 0.0),
                    Button::new(text).right_text(""),
                );

                if resp.clicked() {
                    clicked_item = Some(entry);
                }
                if resp.secondary_clicked() {
                    self.action_context = Some(ActionContext::new(path.clone()));
                }
                if resp.hovered() && ui.ctx().input(|i| i.key_pressed(Key::Q)) {
                    if is_marked {
                        self.marked_files.unmark(&path);
                    } else {
                        self.marked_files.mark(path);
                    }
                }
            }

            if let Some(entry) = clicked_item
                && entry.path().is_dir()
            {
                self.change_dir(entry.path());
            }
        });
    }
}
