use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

/// Rusty Node Version Manager
#[derive(Parser)]
#[clap(name = "rnv", version, about = "Manage multiple Node.js versions, Rust-style")]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List installed Node versions
    List,
    /// Install a specific Node version (e.g. 16.14.0)
    Install { version: String },
    /// Uninstall a version
    Uninstall { version: String },
    /// Activate a version in your shell
    Use { version: String },
    /// Show currently active version
    Current,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::List => list_versions()?,
        Command::Install { version } => install(&version)?,
        Command::Uninstall { version } => uninstall(&version)?,
        Command::Use { version } => use_version(&version)?,
        Command::Current => show_current()?,
    }

    Ok(())
}

// -- Command implementations --

fn list_versions() -> Result<()> {
    let base = get_base_dir()?;
    let dir = base.join("versions");
    for entry in fs::read_dir(&dir)? {
        println!("{}", entry?.file_name().to_string_lossy());
    }
    Ok(())
}

fn install(version: &str) -> Result<()> {
    println!("(stub) Installing Node v{}…", version);
    // TODO: download & unpack here
    Ok(())
}

fn uninstall(version: &str) -> Result<()> {
    println!("(stub) Uninstalling v{}…", version);
    // TODO: remove the directory here
    Ok(())
}

fn use_version(version: &str) -> Result<()> {
    println!("(stub) Switching to Node v{}…", version);
    // TODO: write .current or update shims
    Ok(())
}

fn show_current() -> Result<()> {
    println!("(stub) Current version: <read from .current>");
    Ok(())
}

// -- Helper --

fn get_base_dir() -> Result<PathBuf> {
    let proj = directories::ProjectDirs::from("com", "you", "rnv")
        .ok_or_else(|| anyhow::anyhow!("Could not determine base directory"))?;
    Ok(proj.data_local_dir().to_path_buf())
}
