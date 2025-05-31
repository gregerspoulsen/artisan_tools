use anyhow::Result;
use console::{Emoji, Style, StyledObject};
use dialoguer::{Confirm, Select};
use std::{fs, io::Write};
use strum::VariantArray;

use crate::{config::AtConfig, utils::ProjectRootDir};

#[derive(Debug, Clone, Copy)]
pub struct DryRun(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct Yes(pub bool);

pub fn handle_artisan_init(dry_run: DryRun, yes: Yes) -> Result<()> {
    let project_root: ProjectRootDir = std::env::current_dir()?.try_into()?;
    let (detected_project_type, project_file) = detect_project_type(&project_root);
    let project_type_final = if yes.0 {
        detected_project_type.unwrap_or(ProjectType::ManualSetup)
    } else {
        user_confirmed_project_type(detected_project_type)?
    };

    init_project(
        &project_root,
        &project_type_final,
        project_file.as_deref(),
        dry_run,
    )
}

fn init_project(
    project_root: &ProjectRootDir,
    project_type: &ProjectType,
    project_file: Option<&str>,
    dry_run: DryRun,
) -> Result<()> {
    let default_config = AtConfig::customize_for(project_type, project_file)?;

    let styled_config_name = Style::new().bold().cyan().apply_to(AtConfig::NAME);
    let config_loc = project_root.0.join(AtConfig::NAME);
    let styled_config_loc = Style::new().underlined().apply_to(config_loc.display());
    if dry_run.0 {
        println!("Would write {styled_config_name} to {styled_config_loc}");
    } else {
        fs::write(config_loc, default_config.to_string())?;
        println!("Created {styled_config_name}");
    }

    try_add_raw_version_file_to_gitignore(project_root, dry_run)?;

    let start_cmd = Style::new().bold().yellow().apply_to("at sync");
    let prefix_emoji = Emoji("✅", "=>");
    println!("{prefix_emoji} Init complete! Try running {start_cmd}");
    Ok(())
}

fn try_add_raw_version_file_to_gitignore(
    project_root: &ProjectRootDir,
    dry_run: DryRun,
) -> Result<()> {
    let raw_version_file = AtConfig::DEFAULT_RAW_VERSION_FILE;
    let styled_raw_version_file = Style::new().bold().apply_to(raw_version_file);

    let gitignore = ".gitignore";
    let styled_gitignore = Style::new().underlined().apply_to(gitignore);

    if project_root.contains_file(gitignore) {
        if dry_run.0 {
            println!("Would add {styled_raw_version_file} to {styled_gitignore}");
        } else {
            let mut gitignore = fs::OpenOptions::new()
                .append(true)
                .open(project_root.0.join(gitignore))?;
            gitignore.write_all(raw_version_file.as_bytes())?;
            println!("Added {styled_raw_version_file} to {styled_gitignore}");
        }
    } else {
        let prefix_emoji = Emoji("⚠️", "!");
        println!(
            "{prefix_emoji}  No {styled_gitignore}, consider adding one and adding {styled_raw_version_file} to it"
        );
    }
    Ok(())
}

pub fn prompt_hint_styled(txt: &str) -> StyledObject<&str> {
    Style::new().dim().italic().apply_to(txt)
}

fn user_confirmed_project_type(project_type: Option<ProjectType>) -> Result<ProjectType> {
    if let Some(project_type) = project_type {
        let project_type_styled = project_type.styled_label();
        let prefix_emoji = Emoji("📦", "");
        let project_type_pre = Style::new()
            .bold()
            .cyan()
            .apply_to("Detected project type:");

        let hint = prompt_hint_styled("('no' to select the project type manually)");
        let question = Style::new().yellow().apply_to("continue?");

        let prompt =
            format!("{prefix_emoji} {project_type_pre} {project_type_styled}, {question} {hint}");

        let confirmation = Confirm::new()
            .with_prompt(prompt)
            .wait_for_newline(true)
            .interact()?;

        if confirmation {
            return Ok(project_type);
        }
    }
    prompt_manual_select_project_type()
}

fn prompt_manual_select_project_type() -> Result<ProjectType> {
    let project_types = ProjectType::VARIANTS;
    let items: Vec<String> = project_types.iter().map(|pt| pt.styled_label()).collect();

    let prefix_emoji = Emoji("🧩", "");
    let prompt_style = Style::new().green().bold().apply_to("Kind of project?");
    let hint = prompt_hint_styled("(choose manual to skip tailored setup)");

    let prompt = format!("{prefix_emoji} {prompt_style} {hint}");

    let selection = Select::new()
        .with_prompt(prompt)
        .default(0)
        .items(&items)
        .interact()?;
    Ok(project_types[selection])
}

#[derive(Debug, Clone, Copy, strum::Display, strum::VariantArray)]
pub enum ProjectType {
    #[strum(to_string = "Go")]
    Go,
    #[strum(to_string = "JavaScript/TypeScript")]
    JavaScriptOrTypeScript,
    #[strum(to_string = "Rust")]
    Rust,
    #[strum(to_string = "Python")]
    Python,
    #[strum(to_string = "Manual setup")]
    ManualSetup,
}

impl ProjectType {
    /// Get a console styled string describing the project type
    pub fn styled_label(&self) -> String {
        let style = Style::new();
        let (emoji, style) = match self {
            ProjectType::Go => (Emoji("🔵", ""), style.cyan()),
            ProjectType::JavaScriptOrTypeScript => (Emoji("🌐", ""), style.yellow()),
            ProjectType::Rust => (Emoji("🦀", ""), style.color256(208)),
            ProjectType::Python => (Emoji("🐍", ""), style.green()),
            ProjectType::ManualSetup => (Emoji("⚙️", ""), style.blue()),
        };
        format!("{emoji} {}", style.bold().apply_to(self.to_string()))
    }
}

const PYTHON_PROJECT_FILES: &[&str] = &["pyproject.toml", "setup.py"];
// JavaScript/TypeScript project files.
// 'package.json' covers npm, yarn, bun...
// 'deno.json' covers deno
const JS_TS_PROJECT_FILES: &[&str] = &["package.json", "deno.json"];

// Detects the project type of the project root dir based on configuration files that are unique to each language/type of project
fn detect_project_type(root: &ProjectRootDir) -> (Option<ProjectType>, Option<String>) {
    if root.contains_file("Cargo.toml") {
        return (Some(ProjectType::Rust), None);
    } else if root.contains_file("go.mod") {
        return (Some(ProjectType::Go), None);
    }

    for file in PYTHON_PROJECT_FILES {
        if root.contains_file(file) {
            return (Some(ProjectType::Python), Some(file.to_string()));
        }
    }

    for file in JS_TS_PROJECT_FILES {
        if root.contains_file(file) {
            return (
                Some(ProjectType::JavaScriptOrTypeScript),
                Some(file.to_string()),
            );
        }
    }

    (None, None)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;
    use test_log::test;
    use testresult::TestResult;

    use super::*;

    #[test]
    fn test_customize_config_for_rust() -> TestResult {
        let expected_cfg = r###"[version]
# Files with the current project version (semantic version), updated when an `at merge` is performed.
# Example: link = ["pyproject.toml", "rs/Cargo.toml", "RELEASE"]
#   This configuration would keep the version updated in 3 places:
#    - The pyproject.toml at the project root
#    - The Cargo.toml in the 'rs' subdirectory
#    - the custom file named RELEASE, which would simply contain the version and nothing more
#
#link = "[ <STANDARDIZED_PROJECT_FILE> | <FILE>, .. ]"
link = ["Cargo.toml"]

# This table concerns the version with build metadata in the format:
# <SEMVER>+<BRANCH>-<GIT_SHORT_SHA>[-dirty]
[version.extended]
# Files where the version with the build metadata is written to as is
# (will truncate the file)
#raw = [<FILE>, .. ]
raw = ["VERSION"]
"###;
        let customized_cfg = AtConfig::customize_for(&ProjectType::Rust, None)?;

        assert_str_eq!(expected_cfg, customized_cfg.to_string());

        Ok(())
    }

    #[test]
    fn test_customize_config_for_python() -> TestResult {
        let expected_cfg = r###"[version]
# Files with the current project version (semantic version), updated when an `at merge` is performed.
# Example: link = ["pyproject.toml", "rs/Cargo.toml", "RELEASE"]
#   This configuration would keep the version updated in 3 places:
#    - The pyproject.toml at the project root
#    - The Cargo.toml in the 'rs' subdirectory
#    - the custom file named RELEASE, which would simply contain the version and nothing more
#
#link = "[ <STANDARDIZED_PROJECT_FILE> | <FILE>, .. ]"
link = ["pyproject.toml"]

# This table concerns the version with build metadata in the format:
# <SEMVER>+<BRANCH>-<GIT_SHORT_SHA>[-dirty]
[version.extended]
# Files where the version with the build metadata is written to as is
# (will truncate the file)
#raw = [<FILE>, .. ]
raw = ["VERSION"]
"###;
        let customized_cfg = AtConfig::customize_for(&ProjectType::Python, Some("pyproject.toml"))?;

        assert_str_eq!(expected_cfg, customized_cfg.to_string());

        Ok(())
    }
}
