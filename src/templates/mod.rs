pub mod nextjs;
pub mod tanstack;

use crate::config::ProjectConfig;
use anyhow::Result;
use std::path::Path;

pub fn generate_project(config: &ProjectConfig, project_path: &Path) -> Result<()> {
    match config.framework {
        crate::config::Framework::NextJs => nextjs::generate_nextjs_project(config, project_path),
        crate::config::Framework::TanStackStart => {
            tanstack::generate_tanstack_project(config, project_path)
        }
    }
}
