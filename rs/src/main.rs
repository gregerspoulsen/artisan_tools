use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;

/// Simple CLI for Artisan Tools.
#[derive(Parser)]
#[command(name = "artisan-tools")]
#[command(about = "Artisan Tools CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Top-level commands for the CLI.
#[derive(Subcommand)]
enum Commands {
    /// Manage version-related operations
    #[command(subcommand)]
    Version(VersionCommands),
}

/// Subcommands under `artisan-tools version`
#[derive(Subcommand)]
enum VersionCommands {
    /// Print current version to stdout
    Get,
    /// Update version in `VERSION` file
    Update,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Version(subcmd) => match subcmd {
            VersionCommands::Get => {
                // Print current version to stdout
                let version = fs::read_to_string("VERSION")
                    .context("No VERSION file available - did you forget to run update?")?;
                println!("{}", version);
            }
            VersionCommands::Update => {
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
