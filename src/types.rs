use serde::Serialize;
use std::path::PathBuf;
use tokio::fs;

use crate::{index_dir};

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

impl AppImage {
    pub async fn save_to_index(&self, appname: &str) -> Result<(), Box<dyn std::error::Error>> {
        let index_file = &index_dir().join(format!("{appname}.json"));

        let json = serde_json::to_string_pretty(self)?;
        fs::write(index_file, json).await?;

        Ok(())
    }
}
