use artisan_tools::version;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;

/// Simple CLI for Artisan Tools.
#[derive(Parser)]
#[command(name = "artisan-tools")]
#[command(about = "Artisan Tools CLI", version)]
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
    Get {
        /// Include git information in version string
        #[arg(long)]
        git_info: bool,
    },
    /// Update version in `VERSION` file
    Update,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Version(subcmd) => match subcmd {
            VersionCommand::Get { git_info } => {
                // Print current version to stdout
                let vers =
                    version::get(".", git_info).context("Failed to read the version file")?;
                println!("{}", vers);
            }
            VersionCommand::Update => {
                // Update version in `VERSION` file
                let version_contents =
                    version::get(".", false).context("Failed to read the version file")?;
                let version_contents = version_contents.trim_end();

                fs::write("VERSION", version_contents)
                    .context("Failed to write to the VERSION file")?;
                println!("VERSION updated to {}", version_contents);
            }
        },
    }

    Ok(())
}
