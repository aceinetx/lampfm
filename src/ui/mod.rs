use crate::LampFM;
use imgui::Condition;
use imgui::Ui;
use imgui::WindowFlags;

mod content;
mod top_bar;

impl LampFM {
    pub(crate) fn draw_ui(&mut self, ui: &Ui) {
        ui.window("LampFM")
            .flags(
                WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_DOCKING,
            )
            .size([self.window_size.x, self.window_size.y], Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .build(|| {
                self.draw_top_bar(ui);
                self.draw_content(ui);
            });
    }
}
