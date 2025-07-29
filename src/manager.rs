use std::path::PathBuf;

use tokio::fs;

use crate::{AppImage, Downloader, Index, SymlinkManager, appimages_dir, index_dir};

#[derive(Debug)]
pub struct PackageManager {
    pub downloader: Downloader,
    pub index: Index,
    pub symlink_manager: SymlinkManager,
}

impl PackageManager {
    pub async fn install(
        appimage: &AppImage,
        appname: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if index_dir()
            .join(format!("{}.json", &appimage.executable))
            .exists()
        {
            Err(format!("{} is already installed.", &appimage.executable).into())
        } else {
            // Try to extract filename from URL or use default
            let url = &appimage.source.meta.url;
            let filename = match url.split('/').next_back() {
                Some(name) => name.to_string(),
                None => format!("{}.AppImage", appimage.executable),
            };
            let path = &appimages_dir().join(filename);

            let downloader = crate::Downloader::new();
            downloader.download_with_progress(url, path).await?;

            let index = crate::Index::new();
            index.add(appimage, appname).await?;

            let sm = crate::SymlinkManager::new();
            sm.create(appimage).await?;
            Ok(())
        }
    }
    pub async fn remove(appname: &str) -> Result<(), Box<dyn std::error::Error>> {
        let index_file_path = index_dir().join(format!("{appname}.json"));
        let index_file_content = fs::read_to_string(&index_file_path).await?;
        let appimage: AppImage = serde_json::from_str(&index_file_content)?;

        let home = std::env::var("HOME")?;
        let symlink_path = PathBuf::from(home)
            .join(".local/bin")
            .join(&appimage.executable);
        let index_path = index_dir().join(format!("{}.json", &appimage.executable));

        fs::remove_file(&appimage.file_path).await?;
        fs::remove_file(symlink_path).await?;
        fs::remove_file(index_path).await?;

        Ok(())
    }
}
