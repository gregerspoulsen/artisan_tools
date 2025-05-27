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
                VersionCommand::Get { git_info } => write!(f, "version get git_info={git_info}"),
                VersionCommand::Update => write!(f, "version update"),
            },
            Command::Changeset { target } => write!(f, "changeset {}", target.as_str()),
        }
    }
}
