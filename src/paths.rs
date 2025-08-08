use std::path::PathBuf;

use crate::Result;

pub fn zap_rs_home() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(home).join(".local/share/zap-rs"))
}

pub fn index_dir() -> Result<PathBuf> {
    Ok(zap_rs_home()?.join("index"))
}

pub fn appimages_dir() -> Result<PathBuf> {
    Ok(zap_rs_home()?.join("appimages"))
}

pub fn desktops_dir() -> Result<PathBuf> {
    Ok(zap_rs_home()?.join("desktops"))
}

pub fn icons_dir() -> Result<PathBuf> {
    Ok(zap_rs_home()?.join("icons"))
}
