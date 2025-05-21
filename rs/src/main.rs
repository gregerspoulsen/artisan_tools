use artisan_tools::{version, changeset};
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
    
    /// Manage changeset operations
    #[command(subcommand)]
    Changeset(ChangesetCommand),
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

/// Subcommands under `artisan-tools changeset`
#[derive(Subcommand)]
enum ChangesetCommand {
    /// Initialize a new changeset file
    Init {
        /// The type of version bump
        #[arg(value_enum)]
        target: changeset::BumpTarget,
    },
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
        Command::Changeset(subcmd) => match subcmd {
            ChangesetCommand::Init { target } => {
                changeset::create_changeset_template("at-changeset", target)
                    .context("Failed to create changeset template")?;
                println!("Created new changeset file with target: {}", target.as_str());
            }
        },
    }

    Ok(())
}
