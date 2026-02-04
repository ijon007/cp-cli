use crate::config::ProjectConfig;
use crate::templates;
use crate::utils::{fs, git};
use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

pub fn generate_project(config: ProjectConfig) -> Result<PathBuf> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    // Step 1: Create project directory
    spinner.set_message("Creating project directory...");
    let project_path = fs::create_project_directory(&config.name)?;
    spinner.finish_with_message("✓ Project directory created");

    // Step 2: Generate framework-specific files
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Generating project files...");
    templates::generate_project(&config, &project_path)?;
    spinner.finish_with_message("✓ Project files generated");

    // Step 3: Initialize git repository if requested
    if config.init_git {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        spinner.set_message("Initializing git repository...");
        match git::init_git_repo(&project_path) {
            Ok(_) => {
                spinner.finish_with_message("✓ Git repository initialized");
            }
            Err(e) => {
                let msg = format!("⚠ Git initialization skipped: {}", e);
                spinner.finish_with_message(msg);
            }
        }
    }

    Ok(project_path)
}

pub fn print_success_message(config: &ProjectConfig, _project_path: &PathBuf) {
    println!();
    println!(
        "{} {}",
        style("✨").green(),
        style("Project created successfully!").green().bold()
    );
    println!();
    println!("{}", style("Next steps:").cyan().bold());
    println!("  cd {}", config.name);
    println!("  {}", config.package_manager.install_cmd());
    println!("  {}", config.package_manager.dev_cmd());
    println!();
}
