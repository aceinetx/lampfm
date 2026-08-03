use crate::LampFM;
use imgui::Ui;
use imgui::WindowFlags;
use std::fs::DirEntry;

impl LampFM {
    pub(crate) fn draw_content(&mut self, ui: &Ui) {
        ui.child_window("content")
            .flags(
                WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_DOCKING,
            )
            .build(|| {
                let mut clicked_item: Option<&DirEntry> = None;

                for entry in self.dir_content.iter() {
                    ui.set_next_item_width(-1.0);
                    if ui.button(entry.path().to_str().unwrap()) {
                        clicked_item = Some(&entry);
                    }
                }

                if let Some(entry) = clicked_item {
                    let path = entry.path();
                    if path.is_dir() {
                        self.change_dir(path);
                    } else {
                        // TODO
                    }
                }
            });
    }
}
