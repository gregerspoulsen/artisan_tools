use std::fs;

use crate::{
    changeset,
    cli::{Command, VersionCommand},
    version::AtVersion,
};
use anyhow::{Context, Result};

/// Run an Artisan Tools command
pub fn command(cmd: Command) -> Result<()> {
    match cmd {
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
