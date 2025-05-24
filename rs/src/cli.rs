use crate::changeset::BumpTarget;
use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, Subcommand,
};

/// CLI for Artisan Tools.
#[derive(Parser, Clone, Copy)]
#[command(name = "artisan-tools")]
#[command(about = "Artisan Tools CLI", version)]
#[command(styles=STYLES)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level commands for the CLI.
#[derive(Subcommand, Clone, Copy)]
pub enum Command {
    /// Manage version-related operations
    #[command(subcommand, visible_alias = "ver")]
    Version(VersionCommand),

    /// Generate changeset file with the type of bump target
    Changeset {
        #[arg(value_enum, default_value_t = BumpTarget::Patch)]
        target: BumpTarget,
    },
}

/// Subcommands under `artisan-tools version`
#[derive(Subcommand, Clone, Copy)]
pub enum VersionCommand {
    /// Print current version to stdout
    Get {
        /// Include git information in version string
        #[arg(long)]
        git_info: bool,
    },
    /// Update version in `VERSION` file
    Update,
}

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());
