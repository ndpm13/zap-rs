use clap::{Args, Parser, Subcommand};

/// A command line interface to install AppImages
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Installs an AppImage (alias: i)
    #[command(name = "install", alias = "i")]
    Install(InstallArgs),

    /// Removes an AppImage (alias: rm)
    #[command(name = "remove", alias = "rm")]
    Remove(RemoveArgs),

    /// List the installed AppImages (alias: ls)
    #[command(name = "list", alias = "ls")]
    List,
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    pub appname: String,

    /// Provide a repository slug, or a direct URL to an appimage.
    #[arg(long)]
    pub from: String,

    /// Name of the executable
    #[arg(long)]
    pub executable: Option<String>,

    /// Use --from as repository slug to fetch from GitHub
    #[arg(long, default_value_t = false)]
    pub github: bool,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub appname: String,
}
