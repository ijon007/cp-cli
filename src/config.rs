use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Framework {
    NextJs,
    TanStackStart,
}

impl fmt::Display for Framework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Framework::NextJs => write!(f, "Next.js"),
            Framework::TanStackStart => write!(f, "TanStack Start"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Database {
    Convex,
    NeonDrizzle,
    None,
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Database::Convex => write!(f, "Convex"),
            Database::NeonDrizzle => write!(f, "Neon + Drizzle"),
            Database::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageManager::Npm => write!(f, "npm"),
            PackageManager::Pnpm => write!(f, "pnpm"),
            PackageManager::Yarn => write!(f, "yarn"),
            PackageManager::Bun => write!(f, "bun"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub framework: Framework,
    pub database: Database,
    pub use_clerk: bool,
    pub use_tailwind: bool,
    pub use_shadcn: bool,
    pub package_manager: PackageManager,
    pub init_git: bool,
}

impl ProjectConfig {
    pub fn default(name: String, package_manager: PackageManager) -> Self {
        Self {
            name,
            framework: Framework::NextJs,
            database: Database::None,
            use_clerk: false,
            use_tailwind: false,
            use_shadcn: false,
            package_manager,
            init_git: true,
        }
    }
}
