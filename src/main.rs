use clap::Parser;
use tokio::fs;

use zap_rs::{AppImage, Cli, Command, Source, SourceMetadata, appimages_dir, index_dir};

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

            if index_dir()
                .join(format!("{}.json", &options.executable))
                .exists()
            {
                eprintln!("{} is already installed.", &options.executable);
            } else {
                options.download_from_url().await?;
                options.save_to_index(&args.appname).await?;
                options.create_symlink().await?;
            }
        }
        Command::Remove(args) => {
            let index_file_path = index_dir().join(format!("{}.json", args.appname));
            let index_file_content = fs::read_to_string(&index_file_path).await?;
            let appimage: AppImage = serde_json::from_str(&index_file_content)?;

            appimage.remove().await?;
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
