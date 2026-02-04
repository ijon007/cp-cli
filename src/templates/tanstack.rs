use crate::config::ProjectConfig;
use crate::utils::fs;
use anyhow::Result;
use serde_json::json;
use std::path::Path;

pub fn generate_tanstack_project(config: &ProjectConfig, project_path: &Path) -> Result<()> {
    // Generate package.json
    let mut deps = serde_json::Map::new();
    deps.insert("@tanstack/start".to_string(), json!("latest"));
    deps.insert("@tanstack/router".to_string(), json!("latest"));
    deps.insert("react".to_string(), json!("latest"));
    deps.insert("react-dom".to_string(), json!("latest"));
    deps.insert("vinxi".to_string(), json!("latest"));

    if config.use_tailwind {
        deps.insert("tailwindcss".to_string(), json!("latest"));
        deps.insert("postcss".to_string(), json!("latest"));
        deps.insert("autoprefixer".to_string(), json!("latest"));
    }

    if config.use_clerk {
        deps.insert("@clerk/clerk-react".to_string(), json!("latest"));
    }

    match config.database {
        crate::config::Database::Convex => {
            deps.insert("convex".to_string(), json!("latest"));
        }
        crate::config::Database::NeonDrizzle => {
            deps.insert("drizzle-orm".to_string(), json!("latest"));
            deps.insert("@neondatabase/serverless".to_string(), json!("latest"));
            deps.insert("drizzle-kit".to_string(), json!("latest"));
        }
        crate::config::Database::None => {}
    }

    let package_json = json!({
        "name": config.name,
        "version": "0.1.0",
        "private": true,
        "scripts": {
            "dev": "vinxi dev",
            "build": "vinxi build",
            "start": "vinxi start"
        },
        "dependencies": deps,
        "devDependencies": {
            "typescript": "latest",
            "@types/node": "latest",
            "@types/react": "latest",
            "@types/react-dom": "latest"
        }
    });

    fs::write_file(
        &project_path.join("package.json"),
        &serde_json::to_string_pretty(&package_json)?,
    )?;

    // Generate tsconfig.json
    let tsconfig = json!({
        "compilerOptions": {
            "target": "ES2017",
            "lib": ["dom", "dom.iterable", "esnext"],
            "allowJs": true,
            "skipLibCheck": true,
            "strict": true,
            "noEmit": true,
            "esModuleInterop": true,
            "module": "esnext",
            "moduleResolution": "bundler",
            "resolveJsonModule": true,
            "isolatedModules": true,
            "jsx": "preserve",
            "incremental": true,
            "paths": {
                "@/*": ["./*"]
            }
        },
        "include": ["**/*.ts", "**/*.tsx"],
        "exclude": ["node_modules"]
    });

    fs::write_file(
        &project_path.join("tsconfig.json"),
        &serde_json::to_string_pretty(&tsconfig)?,
    )?;

    // Generate app.tsx
    let app_content = r#"import { createRouter, RouterProvider, Outlet } from '@tanstack/react-router'
import { createRootRoute, createRoute } from '@tanstack/react-router'
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './app.css'

const rootRoute = createRootRoute({
  component: () => {
    return (
      <>
        <Outlet />
      </>
    )
  },
})

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: () => {
    return (
      <div>
        <h1>Welcome to {{PROJECT_NAME}}</h1>
        <p>Get started by editing app.tsx</p>
      </div>
    )
  },
})

const routeTree = rootRoute.addChildren([indexRoute])

const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

function App() {
  return <RouterProvider router={router} />
}

const rootElement = document.getElementById('root')!
createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
"#;
    let app_content = app_content.replace("{{PROJECT_NAME}}", &config.name);
    fs::write_file(&project_path.join("app.tsx"), &app_content)?;

    // Generate app.css
    let app_css = if config.use_tailwind {
        r#"@tailwind base;
@tailwind components;
@tailwind utilities;
"#
    } else {
        r#"* {
  box-sizing: border-box;
  padding: 0;
  margin: 0;
}
"#
    };
    fs::write_file(&project_path.join("app.css"), app_css)?;

    // Generate index.html
    let index_html = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{PROJECT_NAME}}</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="./app.tsx"></script>
  </body>
</html>
"#;
    let index_html = index_html.replace("{{PROJECT_NAME}}", &config.name);
    fs::write_file(&project_path.join("index.html"), &index_html)?;

    // Generate app.config.ts (TanStack Start uses app.config.ts)
    let app_config = r#"import { defineConfig } from '@tanstack/start/config'
import { vitePlugin } from '@tanstack/start/vite'

export default defineConfig({
  vite: {
    plugins: [vitePlugin()],
  },
})
"#;
    fs::write_file(&project_path.join("app.config.ts"), app_config)?;

    // Add Tailwind config if needed
    if config.use_tailwind {
        let tailwind_config = r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './**/*.{js,ts,jsx,tsx,html}',
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
"#;
        fs::write_file(&project_path.join("tailwind.config.js"), tailwind_config)?;

        let postcss_config = r#"module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
"#;
        fs::write_file(&project_path.join("postcss.config.js"), postcss_config)?;
    }

    // Add database setup if needed
    match config.database {
        crate::config::Database::Convex => {
            fs::create_directory(&project_path.join("convex"))?;
            let schema_content = r#"import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  // Define your tables here
  // example: exampleTable: defineTable({ name: v.string() }),
});
"#;
            fs::write_file(&project_path.join("convex/schema.ts"), schema_content)?;
        }
        crate::config::Database::NeonDrizzle => {
            fs::create_directory(&project_path.join("db"))?;
            let schema_content = r#"import { pgTable, serial, text, timestamp } from 'drizzle-orm/pg-core';

export const users = pgTable('users', {
  id: serial('id').primaryKey(),
  name: text('name').notNull(),
  email: text('email').notNull(),
  createdAt: timestamp('created_at').defaultNow(),
});
"#;
            fs::write_file(&project_path.join("db/schema.ts"), schema_content)?;

            let drizzle_config = r#"import type { Config } from 'drizzle-kit';

export default {
  schema: './db/schema.ts',
  out: './drizzle',
  driver: 'pg',
  dbCredentials: {
    connectionString: process.env.DATABASE_URL!,
  },
} satisfies Config;
"#;
            fs::write_file(&project_path.join("drizzle.config.ts"), drizzle_config)?;
        }
        crate::config::Database::None => {}
    }

    // Generate .gitignore
    let gitignore = r#"# dependencies
/node_modules

# build
/dist
/.vinxi

# misc
.DS_Store
*.pem

# debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# local env files
.env*.local

# typescript
*.tsbuildinfo
"#;
    fs::write_file(&project_path.join(".gitignore"), gitignore)?;

    // Generate .env.local if needed
    if config.use_clerk || matches!(config.database, crate::config::Database::NeonDrizzle) {
        let mut env_content = String::new();
        if config.use_clerk {
            env_content.push_str("VITE_CLERK_PUBLISHABLE_KEY=\n");
            env_content.push_str("CLERK_SECRET_KEY=\n");
        }
        if matches!(config.database, crate::config::Database::NeonDrizzle) {
            env_content.push_str("DATABASE_URL=\n");
        }
        fs::write_file(&project_path.join(".env.local"), &env_content)?;
    }

    Ok(())
}
