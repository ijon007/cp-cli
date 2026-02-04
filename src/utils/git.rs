use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub fn init_git_repo(path: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(path)
        .output()
        .with_context(|| "Failed to execute 'git init'. Is git installed?")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git initialization failed: {}", error);
    }

    Ok(())
}
