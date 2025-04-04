mod version_mod;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;

/// Simple CLI for Artisan Tools.
#[derive(Parser)]
#[command(name = "artisan-tools")]
#[command(about = "Artisan Tools CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Top-level commands for the CLI.
#[derive(Subcommand)]
enum Command {
    /// Manage version-related operations
    #[command(subcommand)]
    Version(VersionCommand),
}

/// Subcommands under `artisan-tools version`
#[derive(Subcommand)]
enum VersionCommand {
    /// Print current version to stdout
    Get,
    /// Update version in `VERSION` file
    Update,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Version(subcmd) => match subcmd {
            VersionCommand::Get => {
                // Print current version to stdout
                let version = version_mod::get()
                    .context("Failed to read the version file")?;
                println!("{}", version);
            }
            VersionCommand::Update => {
                // Update version in `VERSION` file
                let release_contents = fs::read_to_string("RELEASE")
                    .context("Failed to read the RELEASE file")?;
                let release_contents = release_contents.trim_end();

                fs::write("VERSION", release_contents)
                    .context("Failed to write to the VERSION file")?;
                println!("VERSION updated to {}", release_contents);
            }
        },
    }

    Ok(())
}
