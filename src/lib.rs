mod args;
mod tui;
mod types;

pub use crate::args::*;
pub use crate::tui::*;
pub use crate::types::*;

use std::path::PathBuf;

pub fn zap_rs_home() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".local/share/zap-rs")
}

pub fn index_dir() -> PathBuf {
    zap_rs_home().join("index")
}

pub fn appimages_dir() -> PathBuf {
    zap_rs_home().join("appimages")
}
