use std::path::PathBuf;

use clap::Parser;
use tokio::fs;

use zap_rs::{AppImage, Cli, Command, PackageManager, Source, SourceMetadata, index_dir};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let pm = PackageManager::new();

    match args.command {
        Command::Install(args) => {
            let mut options = AppImage {
                file_path: PathBuf::new(),
                executable: args.executable.unwrap_or(args.appname.clone()),
                source: Source {
                    identifier: "raw_url".to_string(),
                    meta: SourceMetadata { url: args.from },
                },
            };

            pm.install(&mut options, &args.appname).await?;
        }
        Command::Remove(args) => {
            pm.remove(&args.appname).await?;
        }
        Command::List => {
            let mut appimages = fs::read_dir(index_dir()).await?;

            while let Some(appimage) = appimages.next_entry().await? {
                if let Some(name) = appimage.file_name().to_str() {
                    println!("- {}", name.strip_suffix(".json").unwrap());
                }
            }
        }
    };

    Ok(())
}
