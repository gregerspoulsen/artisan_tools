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

pub fn handle_artisan_init(dry_run: DryRun, Yes(yes): Yes) -> Result<()> {
    let project_root: ProjectRootDir = std::env::current_dir()?.try_into()?;
    let (detected_project_type, project_file) = detect_project_type(&project_root);
    let project_type_final = if yes {
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
    DryRun(dry_run): DryRun,
) -> Result<()> {
    let default_config = AtConfig::customize_for(project_type, project_file)?;

    let styled_config_name = Style::new().bold().cyan().apply_to(AtConfig::NAME);
    let config_loc = project_root.0.join(AtConfig::NAME);
    let styled_config_loc = Style::new().underlined().apply_to(config_loc.display());
    if dry_run {
        println!("Would write {styled_config_name} to {styled_config_loc}");
    } else {
        fs::write(config_loc, default_config.to_string())?;
        println!("Created {styled_config_name}");
    }

    try_add_raw_version_file_to_gitignore(project_root, DryRun(dry_run))?;

    let start_cmd = Style::new().bold().yellow().apply_to("at sync");
    let prefix_emoji = Emoji("✅", "=>");
    println!("{prefix_emoji} Init complete! Try running {start_cmd}");
    Ok(())
}

fn try_add_raw_version_file_to_gitignore(
    project_root: &ProjectRootDir,
    DryRun(dry_run): DryRun,
) -> Result<()> {
    let raw_version_file = AtConfig::DEFAULT_RAW_VERSION_FILE;
    let styled_raw_version_file = Style::new().bold().apply_to(raw_version_file);

    let gitignore = ".gitignore";
    let styled_gitignore = Style::new().underlined().apply_to(gitignore);

    if project_root.contains_file(gitignore) {
        if dry_run {
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
    use test_log::test;
    use testresult::TestResult;
    use toml::Value;

    use super::*;

    #[test]
    fn test_customize_config_for_rust() -> TestResult {
        let customized_cfg = AtConfig::customize_for(&ProjectType::Rust, None)?;
        let customized_cfg_str = customized_cfg.to_string();

        let parsed: Value = toml::from_str(&customized_cfg_str)?;

        // Assert version.link contains "Cargo.toml"
        let version_link = parsed
            .get("version")
            .and_then(|v| v.get("link"))
            .and_then(|l| l.as_array())
            .expect("version.link should be an array");

        assert_eq!(version_link.len(), 1);
        assert_eq!(version_link[0].as_str(), Some("Cargo.toml"));

        // Assert version.extended.raw contains "VERSION"
        let extended_raw = parsed
            .get("version")
            .and_then(|v| v.get("extended"))
            .and_then(|e| e.get("raw"))
            .and_then(|r| r.as_array())
            .expect("version.extended.raw should be an array");

        assert_eq!(extended_raw.len(), 1);
        assert_eq!(extended_raw[0].as_str(), Some("VERSION"));

        insta::assert_snapshot!("Default config customized for Rust", customized_cfg_str);

        Ok(())
    }

    #[test]
    fn test_customize_config_for_python() -> TestResult {
        let customized_cfg = AtConfig::customize_for(&ProjectType::Python, Some("pyproject.toml"))?;
        let customized_cfg_str = customized_cfg.to_string();

        let parsed: Value = toml::from_str(&customized_cfg_str)?;

        // Assert version.link contains "pyproject.toml"
        let version_link = parsed
            .get("version")
            .and_then(|v| v.get("link"))
            .and_then(|l| l.as_array())
            .expect("version.link should be an array");

        assert_eq!(version_link.len(), 1);
        assert_eq!(version_link[0].as_str(), Some("pyproject.toml"));

        // Assert version.extended.raw contains "VERSION"
        let extended_raw = parsed
            .get("version")
            .and_then(|v| v.get("extended"))
            .and_then(|e| e.get("raw"))
            .and_then(|r| r.as_array())
            .expect("version.extended.raw should be an array");

        assert_eq!(extended_raw.len(), 1);
        assert_eq!(extended_raw[0].as_str(), Some("VERSION"));

        insta::assert_snapshot!("Default config customized for Python", customized_cfg_str);

        Ok(())
    }

    #[test]
    fn test_customize_config_for_go() -> TestResult {
        let customized_cfg = AtConfig::customize_for(&ProjectType::Go, None)?;
        let customized_cfg_str = customized_cfg.to_string();

        // Parse the TOML and assert on specific values
        let parsed: Value = toml::from_str(&customized_cfg_str)?;

        // Go projects might have different configuration, adjust assertions as needed
        // For now, just assert that extended.raw contains "VERSION"
        let extended_raw = parsed
            .get("version")
            .and_then(|v| v.get("extended"))
            .and_then(|e| e.get("raw"))
            .and_then(|r| r.as_array())
            .expect("version.extended.raw should be an array");

        assert_eq!(extended_raw.len(), 1);
        assert_eq!(extended_raw[0].as_str(), Some("VERSION"));

        insta::assert_snapshot!("Default config customized for Go", customized_cfg_str);

        Ok(())
    }
}
