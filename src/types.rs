use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct AppImage {
    pub file_path: PathBuf,
    pub executable: String,
    pub source: Source,
}

#[derive(Debug, Serialize)]
pub struct Source {
    pub identifier: String,
    pub meta: SourceMetadata,
}

#[derive(Debug, Serialize)]
pub struct SourceMetadata {
    pub url: String,
}
