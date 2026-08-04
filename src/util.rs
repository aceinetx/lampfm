use std::path::PathBuf;

pub fn expand_tilde(p: &PathBuf) -> PathBuf {
    let p = &p.to_string_lossy().to_string();
    let home = std::env::home_dir().unwrap();

    if let Some(rest) = p.strip_prefix("~/") {
        home.join(rest)
    } else if p == "~" {
        home
    } else {
        PathBuf::from(p)
    }
}
