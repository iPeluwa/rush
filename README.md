# Rush

A modern task runner with parallel execution, intelligent caching, and smart dependency management.

## Why Rush?

- **Content-based caching**: Skip rebuilds when nothing actually changed
- **True parallelization**: Run independent tasks simultaneously
- **Intelligent dependencies**: Automatic task ordering and execution
- **Cross-platform**: No shell quirks or platform-specific gotchas
- **Lightweight**: Fast startup, simple configuration

## Installation

**From source (requires Rust):**
```bash
git clone https://github.com/iPeluwa/rush.git
cd rush
cargo install --path .
```

**From git (requires Rust):**
```bash
cargo install --git https://github.com/iPeluwa/rush.git
```

**Pre-built binaries:** Coming soon - will be available in GitHub releases

**Package managers:** Coming soon - homebrew, apt, etc.

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
rush build    # Run build and its dependencies
rush test     # Run test, build, and install in optimal order
rush -j       # Run all tasks in parallel where possible
```

## Features

- Hash-based incremental builds
- Automatic parallelization with dependency respect
- Project-aware file watching
- Clean output multiplexing for parallel tasks
- Zero-config for common patterns

## Status

🚧 **In Development** - Core functionality being implemented
