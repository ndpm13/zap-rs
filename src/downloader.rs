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
            Some(name) => {
                if name.to_lowercase().ends_with(".appimage") {
                    name.to_string()
                } else {
                    format!("{executable}.AppImage")
                }
            }
            None => format!("{executable}.AppImage"),
        };

        Ok(appimages_dir()?.join(filename))
    }
    pub fn validate_response(&self, resp: &reqwest::Response) -> Result<()> {
        if !resp.status().is_success() {
            return Err(Error::Download {
                url: resp.url().to_string(),
                source: resp.error_for_status_ref().unwrap_err(),
            });
        }

        if let Some(len) = resp.content_length()
            && len < 1024
        {
            return Err(Error::InvalidAppImage);
        }

        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        let is_binary = matches!(
            content_type.as_str(),
            "application/octet-stream"
                | "application/vnd.appimage"
                | "application/x-executable"
                | "application/x-elf"
                | "binary/octet-stream"
                | "application/binary",
        );

        if !is_binary {
            return Err(Error::InvalidAppImage);
        }

        Ok(())
    }
    pub async fn download_with_progress(&self, url: &str, path: &PathBuf) -> Result<()> {
        fs::create_dir_all(&appimages_dir()?).await?;

        let temp_path = PathBuf::from(format!("{}.part", path.display()));

        let resp = reqwest::get(&url.to_string())
            .await
            .map_err(|source| Error::Download {
                url: url.to_string(),
                source,
            })?;

        self.validate_response(&resp)?;

        let total_size = resp.content_length().unwrap_or(0);

        let bar = make_progress_bar(total_size)?;
        let mut out = tokio::fs::File::create(&temp_path).await?;

        // Stream download with progress updates
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = match chunk {
                Ok(chunk) => chunk,
                Err(source) => {
                    fs::remove_file(temp_path).await?;
                    return Err(Error::Download {
                        url: url.to_string(),
                        source,
                    });
                }
            };
            let len = chunk.len() as u64;
            out.write_all(&chunk).await?;
            bar.inc(len);
        }

        bar.finish_with_message("Download complete!");

        fs::rename(temp_path, path).await?;

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
