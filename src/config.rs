use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub show_dotfiles: bool,
    pub places: Vec<(String, PathBuf)>,
}

impl Config {
    pub fn load(&mut self) -> bool {
        let dirs = xdg::BaseDirectories::with_prefix("lampfm");
        let path = dirs.place_config_file("config.toml").unwrap();
        if !std::fs::exists(&path).unwrap() {
            self.save();
            return false;
        }

        let text = match std::fs::read(&path) {
            Ok(vec) => String::from_utf8_lossy_owned(vec),
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        };

        drop(path);

        match toml::from_str::<Config>(&text) {
            Ok(cfg) => {
                *self = cfg;
            }
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        }
        true
    }

    pub fn save(&self) {
        match toml::to_string_pretty(self) {
            Ok(cfg) => {
                let dirs = xdg::BaseDirectories::with_prefix("lampfm");
                if let Err(e) = std::fs::write(dirs.place_config_file("config.toml").unwrap(), cfg)
                {
                    eprintln!("{}", e);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut instance = Self {
            show_dotfiles: false,
            places: Vec::new(),
        };
        instance.places.push((
            "Projects".to_string(),
            std::env::home_dir().unwrap().join("Projects"),
        ));
        instance.places.push((
            "Downloads".to_string(),
            std::env::home_dir().unwrap().join("Downloads"),
        ));
        instance.places.push((
            "Pictures".to_string(),
            std::env::home_dir().unwrap().join("Pictures"),
        ));
        instance.places.push((
            "Documents".to_string(),
            std::env::home_dir().unwrap().join("Documents"),
        ));

        instance
    }
}
