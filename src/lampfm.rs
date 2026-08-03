use crate::SortKind;
use raylib::prelude::*;
use raylib_imgui::RaylibGui;
use std::cmp::Ordering;
use std::fs::DirEntry;
use std::path::PathBuf;

pub struct LampFM {
    pub(crate) window_size: Vector2,
    pub(crate) current_path: PathBuf,
    pub(crate) dir_content: Vec<DirEntry>,
    pub(crate) sort_by: SortKind,
}

impl LampFM {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self) {
        self.refresh();

        let (mut rl, thread) = raylib::init()
            .size(800, 600)
            .title("LampFM")
            .resizable()
            .build();
        self.window_size.x = 800.0;
        self.window_size.y = 600.0;
        let mut gui = RaylibGui::new(&mut rl, &thread);

        while !rl.window_should_close() {
            let ui = gui.begin(&mut rl);

            self.draw_ui(ui);

            self.window_size.x = rl.get_screen_width() as f32;
            self.window_size.y = rl.get_screen_height() as f32;
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            gui.end();
        }
    }

    pub(crate) fn refresh(&mut self) {
        let mut entries = Vec::<DirEntry>::new();

        if let Ok(mut dir) = std::fs::read_dir(&self.current_path) {
            while let Some(Ok(entry)) = dir.next() {
                entries.push(entry);
            }
        }

        // Sort by type, directories first
        entries.sort_by(|a, b| b.path().is_dir().cmp(&a.path().is_dir()));

        match self.sort_by {
            SortKind::Name => {
                // Sort alphabetically
                entries.sort_by(|a, b| {
                    let a = a.file_name().to_ascii_lowercase();
                    let b = b.file_name().to_ascii_lowercase();
                    a.cmp(&b)
                });
            }
            SortKind::Time => {
                // Sort by time
                entries.sort_by(|a, b| {
                    if let Ok(a_meta) = a.metadata()
                        && let Ok(b_meta) = b.metadata()
                        && let Ok(a_modified) = a_meta.modified()
                        && let Ok(b_modified) = b_meta.modified()
                    {
                        b_modified.cmp(&a_modified)
                    } else {
                        Ordering::Less
                    }
                });
            }
        }

        self.dir_content = entries;
    }

    pub(crate) fn change_dir(&mut self, path: PathBuf) {
        self.current_path = path;
        self.refresh();
    }
}

impl Default for LampFM {
    fn default() -> Self {
        Self {
            window_size: Vector2 { x: 0.0, y: 0.0 },
            current_path: std::env::home_dir().unwrap(),
            dir_content: vec![],
            sort_by: SortKind::Time,
        }
    }
}
