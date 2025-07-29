use tokio::fs;

use crate::{AppImage, Downloader, Index, SymlinkManager};

#[derive(Debug, Default)]
pub struct PackageManager {
    pub downloader: Downloader,
    pub index: Index,
    pub symlink_manager: SymlinkManager,
}

impl PackageManager {
    pub fn new() -> Self {
        Self {
            downloader: Downloader::new(),
            index: Index::new(),
            symlink_manager: SymlinkManager::new(),
        }
    }
    pub async fn install(
        &self,
        appimage: &mut AppImage,
        appname: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.index.exists(&appimage.executable) {
            return Err(format!("{} is already installed.", &appimage.executable).into());
        }

        appimage.file_path = self
            .downloader
            .prepare_path(&appimage.source.meta.url, &appimage.executable);
        self.downloader
            .download_with_progress(&appimage.source.meta.url, &appimage.file_path)
            .await?;

        self.index.add(appimage, appname).await?;
        self.symlink_manager.create(appimage).await?;
        Ok(())
    }
    pub async fn remove(&self, appname: &str) -> Result<(), Box<dyn std::error::Error>> {
        let appimage = self.index.get(appname).await?;

        fs::remove_file(&appimage.file_path).await?;
        self.symlink_manager.remove(&appimage.executable).await?;
        self.index.remove(appname).await?;

        Ok(())
    }
}
