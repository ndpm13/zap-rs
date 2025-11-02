use clap::Parser;
use colored::Colorize;

use zap_rs::{AppImage, Cli, Command, PackageManager, Result};

async fn run() -> Result<()> {
    let args = Cli::parse();
    let pm = PackageManager::new();

    match args.command {
        Command::Install(args) => {
            let mut appimage = AppImage::new(&args);

            pm.install(&mut appimage, &args.appname).await?;
        }
        Command::Update(args) => {
            let mut appimage = pm.index.get(&args.appname).await?;

            pm.update(&mut appimage).await?;
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

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
