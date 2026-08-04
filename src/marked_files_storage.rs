use std::{
    path::PathBuf,
    slice::{Iter, IterMut},
};

#[derive(Default)]
pub struct MarkedFilesStorage {
    files: Vec<PathBuf>,
}

impl MarkedFilesStorage {
    pub fn mark(&mut self, path: PathBuf) {
        self.files.push(path);
    }

    pub fn unmark(&mut self, path: &PathBuf) {
        if let Some(index) = self.files.iter().position(|x| *x == *path) {
            self.files.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }

    pub fn contains(&mut self, path: &PathBuf) -> bool {
        self.files.contains(path)
    }

    pub fn remove(&mut self, index: usize) -> PathBuf {
        self.files.remove(index)
    }

    pub fn iter(&self) -> Iter<'_, PathBuf> {
        self.files.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, PathBuf> {
        self.files.iter_mut()
    }
}
