use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::InstallArgs;

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
}
