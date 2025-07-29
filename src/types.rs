use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::{fs, io::AsyncWriteExt};

use crate::{appimages_dir, index_dir, make_progress_bar};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppImage {
    pub file_path: PathBuf,
    pub executable: String,
    pub source: Source,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub identifier: String,
    pub meta: SourceMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceMetadata {
    pub url: String,
}

#[derive(Debug)]
pub struct PackageManager {
    pub downloader: Downloader,
    pub index: Index,
    pub symlink_manager: SymlinkManager,
}

#[derive(Debug, Default)]
pub struct Downloader {}

#[derive(Debug, Default)]
pub struct Index {}

#[derive(Debug, Default)]
pub struct SymlinkManager {}

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

impl Downloader {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn download_with_progress(
        &self,
        url: &str,
        path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&appimages_dir()).await?;

        let resp = reqwest::get(&url.to_string()).await?;
        let total_size = resp.content_length().unwrap_or(0);

        let bar = make_progress_bar(total_size);
        let mut out = tokio::fs::File::create(&path).await?;

        // Stream download with progress updates
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let len = chunk.len() as u64;
            out.write_all(&chunk).await?;
            bar.inc(len);
        }

        bar.finish_with_message("Download complete!");

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path).await?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&path, perms).await?;
        }

        Ok(())
    }
}

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

impl SymlinkManager {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn create(&self, appimage: &AppImage) -> Result<(), Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")?;
        let local_bin = PathBuf::from(home).join(".local/bin");

        fs::create_dir_all(&local_bin).await?;

        let symlink_path = local_bin.join(&appimage.executable);

        #[cfg(unix)]
        {
            use tokio::fs;

            if symlink_path.exists() {
                fs::remove_file(&symlink_path).await?;
            }

            std::os::unix::fs::symlink(&appimage.file_path, &symlink_path)?;
        }

        Ok(())
    }
}
