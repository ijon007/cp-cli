use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_project_directory(name: &str) -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let project_path = current_dir.join(name);

    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }

    fs::create_dir_all(&project_path)
        .with_context(|| format!("Failed to create directory: {}", project_path.display()))?;

    Ok(project_path)
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    Ok(())
}

pub fn create_directory(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    Ok(())
}
