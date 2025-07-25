use std::path::PathBuf;


pub fn ndpm_ai_home() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".local/share/ndpm-appimage")
}

pub fn index_dir() -> PathBuf {
    ndpm_ai_home().join("index")
}

pub fn appimages_dir() -> PathBuf {
    ndpm_ai_home().join("appimages")
}
