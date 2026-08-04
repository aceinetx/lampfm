use crate::{ActionContext, Config, SortKind};
use std::cmp::Ordering;
use std::fs::DirEntry;
use std::path::PathBuf;

pub struct LampFM {
    pub(crate) current_path: PathBuf,
    pub(crate) sort_by: SortKind,
    pub(crate) dir_content: Vec<DirEntry>,
    pub(crate) action_context: Option<ActionContext>,
    pub config: Config,
}

impl LampFM {
    pub(crate) fn change_to_home_dir(&mut self) {
        self.change_dir(std::env::home_dir().unwrap());
    }

    fn sort_file_entries(&self, entries: &mut [DirEntry]) {
        entries.sort_by_key(|a| std::cmp::Reverse(a.path().is_dir()));

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
    }

    pub(crate) fn refresh(&mut self) {
        let mut dirs = Vec::<DirEntry>::new();
        let mut files = Vec::<DirEntry>::new();

        if let Ok(mut dir) = std::fs::read_dir(&self.current_path) {
            while let Some(Ok(entry)) = dir.next() {
                if entry.path().is_dir() {
                    dirs.push(entry)
                } else {
                    files.push(entry)
                };
            }
        }

        self.sort_file_entries(&mut dirs);
        self.sort_file_entries(&mut files);

        // Directories first
        let mut entries = dirs;
        entries.append(&mut files);

        if !self.config.show_dotfiles {
            // Hide dotfiles
            entries = entries
                .drain(..)
                .filter(|a| !a.file_name().to_string_lossy().starts_with('.'))
                .collect();
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
        let mut instance = Self {
            current_path: Default::default(),
            sort_by: SortKind::Name,
            dir_content: vec![],
            config: Config::default(),
            action_context: None,
        };
        instance.change_to_home_dir();
        instance.config.load();
        instance
    }
}
