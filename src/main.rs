mod config;
mod generator;
mod prompts;
mod templates;
mod utils;

use clap::{Parser, Subcommand};
use config::PackageManager;
use console::style;
use generator::{generate_project, print_success_message};
use prompts::collect_project_config;

#[derive(Parser)]
#[command(name = "cp-cli")]
#[command(version = "0.1.0")]
#[command(about = "A fast CLI tool for scaffolding web projects")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    Create {
        /// Project name
        name: Option<String>,
        /// Package manager (npm, pnpm, yarn, bun)
        #[arg(short, long)]
        pm: Option<String>,
        /// Skip prompts and use defaults
        #[arg(short, long)]
        yes: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create { name, pm, yes }) => {
            let pm_override = pm.and_then(|p| {
                match p.as_str() {
                    "npm" => Some(PackageManager::Npm),
                    "pnpm" => Some(PackageManager::Pnpm),
                    "yarn" => Some(PackageManager::Yarn),
                    "bun" => Some(PackageManager::Bun),
                    _ => None,
                }
            });

            match collect_project_config(name, pm_override, yes) {
                Ok(config) => {
                    match generate_project(config.clone()) {
                        Ok(project_path) => {
                            print_success_message(&config, &project_path);
                        }
                        Err(e) => {
                            eprintln!("{} {}", style("✗").red(), style("Error:").red().bold());
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", style("✗").red(), style("Error:").red().bold());
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            println!(
                "{} {}",
                style("✨").green(),
                style("cp-cli v0.1.0").cyan().bold()
            );
            println!("{}", style("Welcome to the project scaffolder CLI!").green());
            println!();
            println!("{}", style("Usage:").cyan().bold());
            println!("  cp-cli create <name>     Create a new project");
            println!("  cp-cli create <name> --pm bun    Override package manager");
            println!("  cp-cli create <name> --yes       Skip prompts");
        }
    }
}
