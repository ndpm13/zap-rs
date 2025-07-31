use tokio::fs;

use crate::{AppImage, Result, index_dir};

#[derive(Debug, Default)]
pub struct Index {}

impl Index {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn get(&self, appname: &str) -> Result<AppImage> {
        let index_file_path = index_dir().join(format!("{appname}.json"));
        let index_file_content = fs::read_to_string(&index_file_path).await?;
        let appimage: AppImage = serde_json::from_str(&index_file_content)?;

        Ok(appimage)
    }
    pub fn exists(&self, executable: &str) -> bool {
        index_dir().join(format!("{}.json", &executable)).exists()
    }
    pub async fn add(&self, appimage: &AppImage, appname: &str) -> Result<()> {
        fs::create_dir_all(&index_dir()).await?;

        let index_file = &index_dir().join(format!("{appname}.json"));

        let json = serde_json::to_string_pretty(appimage)?;
        fs::write(index_file, json).await?;

        Ok(())
    }
    pub async fn remove(&self, appname: &str) -> Result<()> {
        let index_file_path = index_dir().join(format!("{appname}.json"));
        fs::remove_file(index_file_path).await?;

        Ok(())
    }
}
