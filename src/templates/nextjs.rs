use crate::config::ProjectConfig;
use crate::utils::fs;
use anyhow::Result;
use serde_json::json;
use std::path::Path;

pub fn generate_nextjs_project(config: &ProjectConfig, project_path: &Path) -> Result<()> {

    // Generate package.json
    let mut deps = serde_json::Map::new();
    deps.insert("next".to_string(), json!("latest"));
    deps.insert("react".to_string(), json!("latest"));
    deps.insert("react-dom".to_string(), json!("latest"));

    if config.use_tailwind {
        deps.insert("tailwindcss".to_string(), json!("latest"));
        deps.insert("postcss".to_string(), json!("latest"));
        deps.insert("autoprefixer".to_string(), json!("latest"));
    }

    if config.use_clerk {
        deps.insert("@clerk/nextjs".to_string(), json!("latest"));
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
            "dev": "next dev",
            "build": "next build",
            "start": "next start",
            "lint": "next lint"
        },
        "dependencies": deps,
        "devDependencies": {
            "typescript": "latest",
            "@types/node": "latest",
            "@types/react": "latest",
            "@types/react-dom": "latest",
            "eslint": "latest",
            "eslint-config-next": "latest"
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
            "plugins": [
                {
                    "name": "next"
                }
            ],
            "paths": {
                "@/*": ["./*"]
            }
        },
        "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
        "exclude": ["node_modules"]
    });

    fs::write_file(
        &project_path.join("tsconfig.json"),
        &serde_json::to_string_pretty(&tsconfig)?,
    )?;

    // Generate next.config.js
    let next_config = r#"/** @type {import('next').NextConfig} */
const nextConfig = {}

module.exports = nextConfig
"#;
    fs::write_file(&project_path.join("next.config.js"), next_config)?;

    // Create app directory structure
    fs::create_directory(&project_path.join("app"))?;

    // Generate app/layout.tsx
    let layout_content = r#"import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: '{{PROJECT_NAME}}',
  description: 'Generated with cp-cli',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
"#;
    let layout_content = layout_content.replace("{{PROJECT_NAME}}", &config.name);
    fs::write_file(&project_path.join("app/layout.tsx"), &layout_content)?;

    // Generate app/page.tsx
    let page_content = r#"export default function Home() {
  return (
    <main>
      <h1>Welcome to {{PROJECT_NAME}}</h1>
      <p>Get started by editing app/page.tsx</p>
    </main>
  )
}
"#;
    let page_content = page_content.replace("{{PROJECT_NAME}}", &config.name);
    fs::write_file(&project_path.join("app/page.tsx"), &page_content)?;

    // Generate globals.css
    let globals_css = if config.use_tailwind {
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
    fs::write_file(&project_path.join("app/globals.css"), globals_css)?;

    // Add Tailwind config if needed
    if config.use_tailwind {
        let tailwind_config = r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
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

    // Add Clerk middleware if needed
    if config.use_clerk {
        let middleware_content = r#"import { clerkMiddleware } from '@clerk/nextjs/server'

export default clerkMiddleware()

export const config = {
  matcher: [
    '/((?!_next|[^?]*\\.(?:html?|css|js(?!on)|jpe?g|webp|png|gif|svg|ttf|woff2?|ico|csv|docx?|xlsx?|zip|webmanifest)).*)',
    '/(api|trpc)(.*)',
  ],
}
"#;
        fs::write_file(&project_path.join("middleware.ts"), middleware_content)?;
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
/.pnp
.pnp.js

# testing
/coverage

# next.js
/.next/
/out/

# production
/build

# misc
.DS_Store
*.pem

# debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# local env files
.env*.local

# vercel
.vercel

# typescript
*.tsbuildinfo
next-env.d.ts
"#;
    fs::write_file(&project_path.join(".gitignore"), gitignore)?;

    // Generate .env.local if needed
    if config.use_clerk || matches!(config.database, crate::config::Database::NeonDrizzle) {
        let mut env_content = String::new();
        if config.use_clerk {
            env_content.push_str("NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY=\n");
            env_content.push_str("CLERK_SECRET_KEY=\n");
        }
        if matches!(config.database, crate::config::Database::NeonDrizzle) {
            env_content.push_str("DATABASE_URL=\n");
        }
        fs::write_file(&project_path.join(".env.local"), &env_content)?;
    }

    Ok(())
}
