# Rush

A modern task runner with parallel execution, intelligent caching, and smart dependency management.

## Why Rush?

- **Content-based caching**: Skip rebuilds when nothing actually changed
- **True parallelization**: Run independent tasks simultaneously
- **Intelligent dependencies**: Automatic task ordering and execution
- **Cross-platform**: No shell quirks or platform-specific gotchas
- **Lightweight**: Fast startup, simple configuration

## Installation

**From crates.io (recommended):**
```bash
cargo install taskrush
```

**From git (latest):**
```bash
cargo install --git https://github.com/iPeluwa/rush.git
```

**Pre-built binaries:**
Download from [GitHub releases](https://github.com/iPeluwa/rush/releases) for:
- Linux (x86_64, musl)
- macOS (Intel, Apple Silicon)  
- Windows (x86_64)

**Package managers:**
```bash
# macOS (Homebrew)
brew install iPeluwa/tap/taskrush

# Windows (Chocolatey) 
choco install taskrush

# Arch Linux (AUR)
yay -S taskrush

# Snap (Universal Linux)
snap install taskrush
```

**From source:**
```bash
git clone https://github.com/iPeluwa/rush.git
cd rush
cargo install --path .
```

## Getting Started

Create a `.rush` file in your project root:

**Rust project:**
```yaml
tasks:
  build:
    cmd: cargo build
    deps: [check]
    
  test:
    cmd: cargo test
    deps: [build]
    
  check:
    cmd: cargo check
    cache: [Cargo.toml, Cargo.lock]
```

**Node.js project:**
```yaml
tasks:
  build:
    cmd: npm run build
    deps: [install]
    
  test:
    cmd: npm test
    deps: [build]
    
  install:
    cmd: npm install
    cache: package-lock.json
```

Run tasks:
```bash
rush build              # Run build and its dependencies
rush test               # Run test, build, and install in optimal order
rush -j ci              # Run all tasks in parallel where possible
rush --list             # List all tasks with descriptions
rush --watch build      # Watch for changes and rebuild automatically
```

## Features

- ⚡ **True parallel execution** - Independent tasks run simultaneously
- 🧠 **Hash-based incremental builds** - Skip unchanged tasks automatically  
- 📁 **Project-aware file watching** - Auto-rebuild on file changes
- 🎯 **Smart dependency resolution** - Respects task dependencies perfectly
- 🔧 **Environment variable substitution** - `${VAR:-default}` syntax support
- 📋 **Rich task descriptions** - Self-documenting task configurations
- 🖥️ **Cross-platform** - Works on Linux, macOS, and Windows
- ⚙️ **Zero-config** - Simple YAML, works out of the box

## Status

✅ **v0.2.0 Released** - Full-featured task runner with parallel execution, file watching, and intelligent caching!
