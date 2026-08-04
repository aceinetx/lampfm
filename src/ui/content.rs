use crate::LampFM;
use egui::{Button, ScrollArea};
use egui_material_icons::icons::{ICON_FOLDER, ICON_INSERT_DRIVE_FILE};

use std::fs::DirEntry;

impl LampFM {
    pub(crate) fn draw_content(&mut self, ui: &mut eframe::egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            let mut clicked_item: Option<&DirEntry> = None;

            for entry in self.dir_content.iter() {
                let icon = if entry.path().is_dir() {
                    ICON_FOLDER
                } else {
                    ICON_INSERT_DRIVE_FILE
                };

                if ui
                    .add_sized(
                        egui::vec2(ui.available_width(), 0.0),
                        Button::new(format!(
                            "{} {}",
                            icon.codepoint,
                            entry.file_name().to_string_lossy()
                        ))
                        .right_text(""),
                    )
                    .clicked()
                {
                    clicked_item = Some(entry);
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
