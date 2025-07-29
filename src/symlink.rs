use std::path::PathBuf;
use tokio::fs;

use crate::AppImage;

#[derive(Debug, Default)]
pub struct SymlinkManager {}

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
