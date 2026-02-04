# cp-cli

A fast, native CLI tool built with Rust for scaffolding modern web projects. Generate production-ready project templates with your preferred framework, database, and tooling in seconds.

## Features

- **Framework Support**
  - Next.js (App Router)
  - TanStack Start

- **Database Options**
  - Convex
  - Neon + Drizzle ORM
  - None

- **Optional Integrations**
  - Clerk Authentication
  - Tailwind CSS
  - shadcn/ui components

- **Package Manager Detection**
  - Automatically detects npm, pnpm, yarn, or bun
  - Supports manual override via CLI flag

- **Interactive Prompts**
  - User-friendly CLI interface
  - Skip prompts with `--yes` flag for quick setup

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

### Build from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd cp-cli
```

2. Build the project:
```bash
cargo build --release
```

3. Install globally (optional):
```bash
cargo install --path .
```

This will install `cp-cli` to `~/.cargo/bin/` (or `%USERPROFILE%\.cargo\bin` on Windows), which should already be in your PATH.

## Usage

### Basic Usage

Create a new project interactively:

```bash
cp-cli create my-app
```

This will prompt you to select:
- Framework (Next.js or TanStack Start)
- Database (Convex, Neon+Drizzle, or None)
- Optional features (Clerk, Tailwind CSS, shadcn/ui)
- Git initialization

### Override Package Manager

Specify a package manager explicitly:

```bash
cp-cli create my-app --pm bun
```

Available options: `npm`, `pnpm`, `yarn`, `bun`

### Skip Prompts

Use default settings and skip all prompts:

```bash
cp-cli create my-app --yes
```

Defaults:
- Framework: Next.js
- Database: None
- Features: None
- Git: Initialized

### Package Manager Detection

The CLI automatically detects your package manager by checking (in order):
1. Lock files in the current directory (`bun.lockb`, `pnpm-lock.yaml`, `yarn.lock`, `package-lock.json`)
2. `npm_config_user_agent` environment variable
3. Available binaries in PATH
4. Falls back to npm

## Examples

### Create a Next.js app with Clerk and Tailwind

```bash
cp-cli create my-nextjs-app
# Select: Next.js, None (database), Clerk + Tailwind
```

### Create a TanStack Start app with Convex

```bash
cp-cli create my-tanstack-app
# Select: TanStack Start, Convex
```

### Quick setup with defaults

```bash
cp-cli create my-app --yes
```

## Project Structure

Generated projects include:

- **Configuration files**: `package.json`, `tsconfig.json`, framework configs
- **Source code**: Framework-specific app structure
- **Styling**: Tailwind CSS setup (if selected)
- **Database**: Schema files for Convex or Drizzle (if selected)
- **Environment**: `.env.local` template (if needed)
- **Git**: Initialized repository (if selected)

## After Project Creation

Once your project is created, follow these steps:

```bash
cd my-app
npm install  # or pnpm/yarn/bun install
npm run dev  # or pnpm/yarn/bun dev
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run -- create my-test-app
```

## Project Architecture

The CLI is built with a modular architecture:

```
src/
├── main.rs              # CLI entry point and command parsing
├── config.rs            # Configuration structs and enums
├── prompts.rs           # Interactive user prompts
├── generator.rs         # Project generation orchestration
├── templates/
│   ├── mod.rs           # Template module exports
│   ├── nextjs.rs        # Next.js template generation
│   └── tanstack.rs      # TanStack Start template generation
└── utils/
    ├── mod.rs           # Utils module exports
    ├── fs.rs            # File system operations
    ├── git.rs           # Git repository initialization
    └── package_manager.rs # Package manager detection
```

## Requirements

- Rust 1.93.0 or later
- Git (optional, for repository initialization)

## License

[Add your license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Troubleshooting

### Command not found

If `cp-cli` is not found after installation, ensure `~/.cargo/bin` (or `%USERPROFILE%\.cargo\bin` on Windows) is in your PATH.

### Git initialization fails

If git initialization fails, ensure Git is installed and available in your PATH. The project will still be created successfully without git initialization.

### Package manager not detected

If the wrong package manager is detected, use the `--pm` flag to override it explicitly.
