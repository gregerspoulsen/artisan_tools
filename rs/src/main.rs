use anyhow::{Context, Result};
use artisan_tools::{
    changeset,
    cli::{Cli, Command, VersionCommand},
    version::AtVersion,
};
use clap::Parser;
use std::fs;

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
