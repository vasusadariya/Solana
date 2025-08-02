use clap::{Parser, Subcommand};
use anyhow::Result;             // <-- use anyhow’s Result
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "rnv", version, about = "Rusty Node Version Manager")]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    List,
    Install { version: String },
    Uninstall { version: String },
    Use { version: String },
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

fn list_versions() -> Result<()> {
    let dir = PathBuf::from("./versions");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
        println!("(created {})", dir.display());
    }
    for entry in fs::read_dir(&dir)? {
        println!("{}", entry?.file_name().to_string_lossy());
    }
    Ok(())
}

fn install(version: &str) -> Result<()> {
    let dir = PathBuf::from("./versions");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    let target = dir.join(version);
    if target.exists() {
        println!("Version {} already installed", version);
        return Ok(());
    }
    fs::create_dir(&target)?;
    println!("Version {} installed at {}", version, target.display());
    Ok(())
}

fn uninstall(version: &str) -> Result<()> {
    let dir = PathBuf::from("./versions");
    let target = dir.join(version);
    if target.exists() {
        fs::remove_dir_all(&target)?;
        println!("Version {} uninstalled", version);
        return Ok(());
    }
    anyhow::bail!("Version {} not found", version);
}

fn use_version(version: &str) -> Result<()> {
    let dir = PathBuf::from("./versions");
    let target = dir.join(version);
    if target.exists() {
        println!("Version {} is now active", version);
        // TODO: write .current or update shims
        return Ok(());
    }
    anyhow::bail!("Version {} not found", version);
}

fn show_current() -> Result<()> {
    println!("Current version: <not yet implemented>");
    Ok(())
}
