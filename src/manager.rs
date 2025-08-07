use tokio::fs;

use crate::{
    AppImage, Downloader, Index, Result, SymlinkManager, get_github_release_url, index_dir,
};

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
    pub async fn install(&self, appimage: &mut AppImage, appname: &str) -> Result<()> {
        if self.index.exists(&appimage.executable)? {
            println!("{} is already installed.", appimage.executable);
            return Ok(());
        }

        appimage.file_path = self
            .downloader
            .prepare_path(&appimage.source.meta.url, &appimage.executable)?;

        if appimage.source.identifier != "git.github" {
            self.downloader
                .download_with_progress(&appimage.source.meta.url, &appimage.file_path)
                .await?;
        } else {
            self.downloader
                .download_with_progress(
                    &get_github_release_url(appimage).await?,
                    &appimage.file_path,
                )
                .await?;
        }

        self.index.add(appimage, appname).await?;
        self.symlink_manager.create(appimage).await?;
        Ok(())
    }
    pub async fn remove(&self, appname: &str) -> Result<()> {
        let appimage = self.index.get(appname).await?;

        fs::remove_file(&appimage.file_path).await?;
        self.symlink_manager.remove(&appimage.executable).await?;
        self.index.remove(appname).await?;

        Ok(())
    }
    pub async fn list(&self) -> Result<()> {
        let mut appimages = fs::read_dir(index_dir()?).await?;

        while let Some(appimage) = appimages.next_entry().await? {
            if let Some(stem) = appimage.path().file_stem().and_then(|s| s.to_str()) {
                println!("- {stem}");
            }
        }

        Ok(())
    }
}
