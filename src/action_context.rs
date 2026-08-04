use std::path::PathBuf;

pub struct ActionContext {
    pub path: PathBuf,
    pub rename_input: String,
}

impl ActionContext {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path.clone(),
            rename_input: path.file_name().unwrap().to_str().unwrap().to_string(),
        }
    }
}
