use futures_util::StreamExt;
use std::path::PathBuf;
use tokio::{fs, io::AsyncWriteExt};

use crate::{appimages_dir, make_progress_bar};

#[derive(Debug, Default)]
pub struct Downloader {}

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
