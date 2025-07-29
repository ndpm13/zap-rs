use clap::Parser;
use tokio::fs;

use zap_rs::{
    AppImage, Cli, Command, PackageManager, Source, SourceMetadata, appimages_dir, index_dir,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Install(args) => {
            let options = AppImage {
                file_path: appimages_dir().join(
                    args.from
                        .split('/')
                        .next_back()
                        .filter(|s| !s.is_empty())
                        .unwrap_or("app.AppImage"),
                ),
                executable: args.executable.unwrap_or(args.appname.clone()),
                source: Source {
                    identifier: "raw_url".to_string(),
                    meta: SourceMetadata { url: args.from },
                },
            };

            PackageManager::install(&options, &args.appname).await?;
        }
        Command::Remove(args) => {
            PackageManager::remove(&args.appname).await?;
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
