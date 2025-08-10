use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};
use tokio::fs;

use crate::{Error, InstallArgs, Result, desktops_dir, icons_dir};

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
    pub fn new(options: &InstallArgs) -> Self {
        Self {
            file_path: PathBuf::new(),
            executable: options
                .executable
                .as_ref()
                .unwrap_or(&options.appname)
                .to_string(),
            source: Source {
                identifier: if options.github {
                    "git.github".to_string()
                } else {
                    "raw_url".to_string()
                },
                meta: SourceMetadata {
                    url: options.from.clone(),
                },
            },
        }
    }
    async fn extract_assets(&self) -> Result<PathBuf> {
        let temp_dir = std::env::temp_dir().join("zap-rs");

        fs::create_dir_all(&temp_dir).await?;

        // Extract desktop file
        Command::new(&self.file_path)
            .arg("--appimage-extract")
            .arg("*.desktop")
            .current_dir(&temp_dir)
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
        Command::new(&self.file_path)
            .arg("--appimage-extract")
            .arg("usr/share/applications/*.desktop")
            .current_dir(&temp_dir)
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;

        // Extract icon
        Command::new(&self.file_path)
            .arg("--appimage-extract")
            .arg("usr/share/icons/hicolor/*/apps/*.png")
            .current_dir(&temp_dir)
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;

        Ok(temp_dir)
    }
    async fn fix_desktop(&self, desktop_file_path: &PathBuf, icon_found: bool) -> Result<()> {
        let file_content = fs::read_to_string(&desktop_file_path).await?;

        let appimage_path = self.file_path.to_str().ok_or(Error::InvalidPath)?;

        let icon_path = icons_dir()?
            .join(format!("{}.png", self.executable))
            .to_str()
            .ok_or(Error::InvalidPath)?
            .to_string();

        let fixed_file_content: Vec<String> = file_content
            .lines()
            .map(|line| {
                if line.contains("Exec=") {
                    if let Some(exec_line) = line.split_once(" ") {
                        if let Some(exec_arg) = exec_line.0.split_once("=") {
                            format!("{}={} {}", exec_arg.0, appimage_path, exec_line.1)
                        } else {
                            line.to_string()
                        }
                    } else if let Some(exec_arg) = line.split_once("=") {
                        format!("{}={}", exec_arg.0, appimage_path)
                    } else {
                        line.to_string()
                    }
                } else if line.contains("Icon=") && icon_found {
                    if let Some(exec_arg) = line.split_once("=") {
                        format!("{}={}", exec_arg.0, icon_path)
                    } else {
                        line.to_string()
                    }
                } else {
                    line.to_string()
                }
            })
            .collect();

        fs::write(desktop_file_path, fixed_file_content.join("\n")).await?;

        Ok(())
    }
    pub async fn integrate_desktop(&self) -> Result<()> {
        let temp_dir = self.extract_assets().await?;
        let squashfs = &temp_dir.join("squashfs-root");

        fs::create_dir_all(desktops_dir()?).await?;
        fs::create_dir_all(icons_dir()?).await?;
        fs::create_dir_all(
            PathBuf::from(std::env::var("HOME")?).join(".local/share/applications/"),
        )
        .await?;

        let icon_path = icons_dir()?.join(format!("{}.png", self.executable));
        let desktop_file_paths = (
            desktops_dir()?.join(format!("{}.desktop", self.executable)),
            PathBuf::from(std::env::var("HOME")?).join(format!(
                ".local/share/applications/{}.desktop",
                self.executable
            )),
        );

        let icon_resolutions = [
            "1024", "720", "512", "256", "192", "128", "96", "72", "64", "48", "36", "32", "24",
            "22", "16",
        ];

        let mut icon_found = false;

        for res in icon_resolutions {
            let icon_dir = squashfs.join(format!("usr/share/icons/hicolor/{}x{}/apps", res, res));

            if fs::try_exists(&icon_dir).await? {
                let mut squashfs_icon_entries = fs::read_dir(&icon_dir).await?;
                while let Some(entry) = squashfs_icon_entries.next_entry().await? {
                    if entry.path().extension() == Some("png".as_ref()) {
                        fs::copy(entry.path(), &icon_path).await?;
                        icon_found = true;
                        break;
                    }
                }
                if icon_found {
                    break;
                }
            }
        }

        let mut squashfs_entries = fs::read_dir(&squashfs).await?;
        while let Some(entry) = squashfs_entries.next_entry().await? {
            if entry.path().extension() == Some("desktop".as_ref()) {
                fs::copy(fs::canonicalize(entry.path()).await?, &desktop_file_paths.0).await?;

                self.fix_desktop(&desktop_file_paths.0, icon_found).await?;

                fs::copy(&desktop_file_paths.0, &desktop_file_paths.1).await?;
            }
        }

        // Clean up
        fs::remove_dir_all(temp_dir).await?;

        Ok(())
    }
}
