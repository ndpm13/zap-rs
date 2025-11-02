use std::{
    io::{self, Write},
    path::PathBuf,
};
use tokio::fs;

use crate::{
    AppImage, Downloader, Error, Index, Result, SymlinkManager, desktops_dir,
    get_github_release_url, icons_dir, index_dir,
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

        print!("Do you want to integrate this appimage? (y/N) ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.to_lowercase().trim() == "y" || input.to_lowercase().trim() == "yes" {
            appimage.integrate_desktop().await?;
        }

        Ok(())
    }
    pub async fn remove(&self, appname: &str) -> Result<()> {
        let appimage = self.index.get(appname).await?;

        fs::remove_file(&appimage.file_path).await?;
        self.symlink_manager.remove(&appimage.executable).await?;
        self.index.remove(appname).await?;

        if fs::try_exists(desktops_dir()?.join(format!("{}.desktop", appimage.executable))).await? {
            fs::remove_file(desktops_dir()?.join(format!("{}.desktop", appimage.executable)))
                .await?;
        }
        if fs::try_exists(PathBuf::from(std::env::var("HOME")?).join(format!(
            ".local/share/applications/{}.desktop",
            appimage.executable
        )))
        .await?
        {
            fs::remove_file(PathBuf::from(std::env::var("HOME")?).join(format!(
                ".local/share/applications/{}.desktop",
                appimage.executable
            )))
            .await?;
        }
        if fs::try_exists(icons_dir()?.join(format!("{}.png", appimage.executable))).await? {
            fs::remove_file(icons_dir()?.join(format!("{}.png", appimage.executable))).await?;
        }

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
    pub async fn update(&self, appimage: &mut AppImage) -> Result<()> {
        if appimage.source.identifier != "git.github".to_string() {
            return Err(Error::CantUpdatePkg);
        }

        self.downloader
            .download_with_progress(
                &get_github_release_url(appimage).await?,
                &appimage.file_path,
            )
            .await?;

        Ok(())
    }
}
