use super::*;
use std::fmt;

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at {}", self.command)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Version(version_command) => match version_command {
                VersionCommand::Get { git_info } => write!(
                    f,
                    "version get {if_git_info}",
                    if_git_info = if *git_info { "--git-info" } else { "" }
                ),
                VersionCommand::Update => write!(f, "version update"),
            },
            Command::Changeset { target } => write!(f, "changeset {}", target.as_str()),
            Command::Init { dry_run, yes } => {
                write!(
                    f,
                    "init {if_dry_run} {if_yes}",
                    if_dry_run = if *dry_run { "--dry-run" } else { "" },
                    if_yes = if *yes { "--yes" } else { "" }
                )
            }
        }
    }
}
