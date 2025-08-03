use futures_util::StreamExt;
use std::path::PathBuf;
use tokio::{fs, io::AsyncWriteExt};

use crate::{Error, Result, appimages_dir, make_progress_bar};

#[derive(Debug, Default)]
pub struct Downloader {}

impl Downloader {
    pub fn new() -> Self {
        Self {}
    }
    pub fn prepare_path(&self, url: &str, executable: &str) -> Result<PathBuf> {
        // Try to extract filename from URL or use default
        let filename = match url.split('/').next_back() {
            Some(name) => name.to_string(),
            None => format!("{executable}.AppImage"),
        };

        Ok(appimages_dir()?.join(filename))
    }
    pub async fn download_with_progress(&self, url: &str, path: &PathBuf) -> Result<()> {
        fs::create_dir_all(&appimages_dir()?).await?;

        let resp = reqwest::get(&url.to_string())
            .await
            .map_err(|source| Error::Download {
                url: url.to_string(),
                source,
            })?;
        let total_size = resp.content_length().unwrap_or(0);

        let bar = make_progress_bar(total_size)?;
        let mut out = tokio::fs::File::create(&path).await?;

        // Stream download with progress updates
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|source| Error::Download {
                url: url.to_string(),
                source,
            })?;
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
