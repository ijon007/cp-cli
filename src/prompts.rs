use crate::config::{Database, Framework, PackageManager, ProjectConfig};
use dialoguer::{Confirm, Input, MultiSelect, Select};

pub fn collect_project_config(
    name: Option<String>,
    pm_override: Option<PackageManager>,
    skip_prompts: bool,
) -> anyhow::Result<ProjectConfig> {
    let project_name = if let Some(name) = name {
        name
    } else if skip_prompts {
        "my-app".to_string()
    } else {
        Input::new()
            .with_prompt("Project name")
            .default("my-app".to_string())
            .interact_text()?
    };

    let package_manager = if let Some(pm) = pm_override {
        pm
    } else if skip_prompts {
        PackageManager::detect()
    } else {
        let pm_options = vec!["npm", "pnpm", "yarn", "bun"];
        let selected = Select::new()
            .with_prompt("Package manager")
            .items(&pm_options)
            .default(0)
            .interact()?;

        match selected {
            0 => PackageManager::Npm,
            1 => PackageManager::Pnpm,
            2 => PackageManager::Yarn,
            3 => PackageManager::Bun,
            _ => PackageManager::Npm,
        }
    };

    if skip_prompts {
        return Ok(ProjectConfig::default(project_name, package_manager));
    }

    // Framework selection
    let framework_options = vec!["Next.js", "TanStack Start"];
    let framework_selected = Select::new()
        .with_prompt("Select framework")
        .items(&framework_options)
        .default(0)
        .interact()?;

    let framework = match framework_selected {
        0 => Framework::NextJs,
        1 => Framework::TanStackStart,
        _ => Framework::NextJs,
    };

    // Database selection
    let database_options = vec!["None", "Convex", "Neon + Drizzle"];
    let database_selected = Select::new()
        .with_prompt("Select database")
        .items(&database_options)
        .default(0)
        .interact()?;

    let database = match database_selected {
        0 => Database::None,
        1 => Database::Convex,
        2 => Database::NeonDrizzle,
        _ => Database::None,
    };

    // Optional features
    let feature_options = vec!["Clerk (Auth)", "Tailwind CSS", "shadcn/ui"];
    let feature_selections = MultiSelect::new()
        .with_prompt("Select optional features (space to select, enter to confirm)")
        .items(&feature_options)
        .interact()?;

    let use_clerk = feature_selections.contains(&0);
    let use_tailwind = feature_selections.contains(&1);
    let use_shadcn = feature_selections.contains(&2);

    // Git initialization
    let init_git = Confirm::new()
        .with_prompt("Initialize git repository?")
        .default(true)
        .interact()?;

    Ok(ProjectConfig {
        name: project_name,
        framework,
        database,
        use_clerk,
        use_tailwind,
        use_shadcn,
        package_manager,
        init_git,
    })
}
