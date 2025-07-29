use tokio::fs;

use crate::{AppImage, index_dir};

#[derive(Debug, Default)]
pub struct Index {}

impl Index {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn add(
        &self,
        appimage: &AppImage,
        appname: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&index_dir()).await?;

        let index_file = &index_dir().join(format!("{appname}.json"));

        let json = serde_json::to_string_pretty(appimage)?;
        fs::write(index_file, json).await?;

        Ok(())
    }
}
