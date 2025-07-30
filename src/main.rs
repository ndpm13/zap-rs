use std::path::PathBuf;

use clap::Parser;

use zap_rs::{AppImage, Cli, Command, PackageManager, Source, SourceMetadata};

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
            pm.list().await?;
        }
    };

    Ok(())
}
