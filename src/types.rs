use futures_util::StreamExt;
use serde::{Serialize, Deserialize};
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

impl AppImage {
    pub async fn save_to_index(&self, appname: &str) -> Result<(), Box<dyn std::error::Error>> {
        let index_file = &index_dir().join(format!("{appname}.json"));

        let json = serde_json::to_string_pretty(self)?;
        fs::write(index_file, json).await?;

        Ok(())
    }
    pub async fn download_from_url(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&appimages_dir()).await?;

        // Try to extract filename from URL or use default
        let url = &self.source.meta.url;
        let filename = match url.split('/').next_back() {
            Some(name) => name.to_string(),
            None => format!("{}.AppImage", &self.executable),
        };
        let file_path = &appimages_dir().join(filename);

        let resp = reqwest::get(&url.to_string()).await?;
        let total_size = resp.content_length().unwrap_or(0);

        let bar = make_progress_bar(total_size);
        let mut out = tokio::fs::File::create(&file_path).await?;

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
            let mut perms = fs::metadata(&file_path).await?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&file_path, perms).await?;
        }

        Ok(())
    }
    pub async fn create_symlink(&self) -> Result<(), Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")?;
        let local_bin = PathBuf::from(home).join(".local/bin");

        fs::create_dir_all(&local_bin).await?;

        let symlink_path = local_bin.join(&self.executable);

        #[cfg(unix)]
        {
            use tokio::fs;

            if symlink_path.exists() {
                fs::remove_file(&symlink_path).await?;
            }

            std::os::unix::fs::symlink(&self.file_path, &symlink_path)?;
        }

        Ok(())
    }
    pub async fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")?;
        let symlink_path = PathBuf::from(home).join(".local/bin").join(&self.executable);
        let index_path = index_dir().join(format!("{}.json", &self.executable));

        fs::remove_file(&self.file_path).await?;
        fs::remove_file(symlink_path).await?;
        fs::remove_file(index_path).await?;

        Ok(())
    }
}
