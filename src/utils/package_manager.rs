use crate::config::PackageManager;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

impl PackageManager {
    pub fn detect() -> Self {
        // 1. Check for lock files in current directory
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    match name {
                        "bun.lockb" => return PackageManager::Bun,
                        "pnpm-lock.yaml" => return PackageManager::Pnpm,
                        "yarn.lock" => return PackageManager::Yarn,
                        "package-lock.json" => return PackageManager::Npm,
                        _ => {}
                    }
                }
            }
        }

        // 2. Check npm_config_user_agent environment variable
        if let Ok(agent) = std::env::var("npm_config_user_agent") {
            if agent.contains("bun") {
                return PackageManager::Bun;
            } else if agent.contains("pnpm") {
                return PackageManager::Pnpm;
            } else if agent.contains("yarn") {
                return PackageManager::Yarn;
            }
        }

        // 3. Check which binaries exist
        if command_exists("bun") {
            return PackageManager::Bun;
        }
        if command_exists("pnpm") {
            return PackageManager::Pnpm;
        }
        if command_exists("yarn") {
            return PackageManager::Yarn;
        }

        // 4. Default to npm
        PackageManager::Npm
    }

    pub fn install_cmd(&self) -> &str {
        match self {
            PackageManager::Npm => "npm install",
            PackageManager::Pnpm => "pnpm install",
            PackageManager::Yarn => "yarn install",
            PackageManager::Bun => "bun install",
        }
    }

    pub fn dev_cmd(&self) -> &str {
        match self {
            PackageManager::Npm => "npm run dev",
            PackageManager::Pnpm => "pnpm dev",
            PackageManager::Yarn => "yarn dev",
            PackageManager::Bun => "bun run dev",
        }
    }

    pub fn run_cmd(&self, script: &str) -> String {
        match self {
            PackageManager::Npm => format!("npm run {}", script),
            PackageManager::Pnpm => format!("pnpm {}", script),
            PackageManager::Yarn => format!("yarn {}", script),
            PackageManager::Bun => format!("bun run {}", script),
        }
    }

    pub fn exec_cmd(&self, command: &str) -> String {
        match self {
            PackageManager::Npm => format!("npx {}", command),
            PackageManager::Pnpm => format!("pnpm exec {}", command),
            PackageManager::Yarn => format!("yarn {}", command),
            PackageManager::Bun => format!("bun {}", command),
        }
    }
}

fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|o| o.status.success())
        .unwrap_or_else(|_| {
            // On Windows, try 'where' command
            Command::new("where")
                .arg(command)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
}
