use anyhow::{Context, Result};
use artisan_tools::{changeset, version::AtVersion};
use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, Subcommand,
};
use std::fs;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

/// Simple CLI for Artisan Tools.
#[derive(Parser)]
#[command(name = "artisan-tools")]
#[command(about = "Artisan Tools CLI", version)]
#[command(styles=STYLES)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Top-level commands for the CLI.
#[derive(Subcommand)]
enum Command {
    /// Manage version-related operations
    #[command(subcommand, visible_alias = "ver")]
    Version(VersionCommand),

    /// Generate changeset file with the type of bump target
    Changeset {
        #[arg(value_enum, default_value_t = changeset::BumpTarget::Patch)]
        target: changeset::BumpTarget,
    },
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
                let vers = if git_info {
                    AtVersion::with_metadata(".")?
                } else {
                    AtVersion::new(".")?
                };
                println!("{vers}");
            }
            VersionCommand::Update => {
                // Update version in `VERSION` file
                let version_contents = AtVersion::new(".")?;

                fs::write("VERSION", version_contents.to_string())
                    .context("Failed to write to the VERSION file")?;
                println!("VERSION updated to {}", version_contents);
            }
        },
        Command::Changeset { target } => {
            changeset::create_changeset_template("at-changeset", target)
                .context("Failed to create changeset template")?;
            println!(
                "Created new changeset file with target: {}",
                target.as_str()
            );
        }
    }

    Ok(())
}
