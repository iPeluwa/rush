# TaskRush Examples

This directory contains real-world examples of TaskRush configurations for different types of projects. Each example demonstrates best practices and common patterns.

## 📁 Available Examples

### 🌐 [React App](react-app/.rush)
**Frontend development workflow for React applications**

Features demonstrated:
- Development server with hot reload
- TypeScript type checking and linting
- Jest testing with coverage reports
- Webpack bundling and optimization
- End-to-end testing with Cypress
- Production deployment pipeline

**Use this for**: React, Vue, Angular, or any frontend framework

### 🦀 [Rust CLI](rust-cli/.rush)
**Complete Rust development workflow for CLI applications**

Features demonstrated:
- Fast syntax checking with `cargo check`
- Code formatting with `rustfmt`
- Linting with Clippy
- Unit, integration, and documentation tests
- Release builds and cross-compilation
- Security auditing and dependency management
- Publishing to crates.io

**Use this for**: Rust CLI tools, libraries, or applications

### 🐍 [Python API](python-api/.rush)
**Python web API development with FastAPI/Django**

Features demonstrated:
- Virtual environment management
- Code formatting with Black and isort
- Type checking with mypy
- Testing with pytest and coverage
- Security scanning with bandit
- Database migrations with Alembic
- Development and production servers
- Docker containerization

**Use this for**: Python web APIs, Django apps, Flask applications

### 📦 [Monorepo](monorepo/.rush)
**Multi-package repository management**

Features demonstrated:
- Workspace dependency management
- Cross-package build coordination
- Shared library building
- Independent service development
- Integration testing across packages
- Coordinated deployment strategies

**Use this for**: Large projects with multiple services, shared libraries

## 🚀 Getting Started with Examples

### 1. Copy an Example

```bash
# Copy the React example to your project
cp examples/react-app/.rush /path/to/your/project/

# Or copy the Rust example
cp examples/rust-cli/.rush /path/to/your/rust/project/
```

### 2. Customize for Your Project

Edit the `.rush` file to match your project structure:

```yaml
tasks:
  build:
    cmd: your-build-command  # Change this
    cache_files:
      - "your-src/**/*"      # Update file patterns
    depends_on: [your-deps]  # Update dependencies
```

### 3. Run Tasks

```bash
# List available tasks
taskrush --list

# Run a specific task
taskrush build

# Run CI pipeline
taskrush ci
```

## 📖 Example Breakdown

### Task Structure Pattern

All examples follow a consistent pattern:

```yaml
tasks:
  # Quick development tasks
  dev:
    cmd: start-dev-server
    depends_on: [install]
  
  # Code quality
  lint:
    cmd: run-linter
    cache_files: ["src/**/*"]
    depends_on: [install]
  
  # Testing
  test:
    cmd: run-tests
    cache_files: ["src/**/*", "test/**/*"]
    depends_on: [install]
  
  # Building
  build:
    cmd: build-project
    depends_on: [lint, test]
  
  # Workflow tasks
  ci:
    cmd: echo "CI complete"
    depends_on: [lint, test, build]
```

### Caching Strategy

Examples show effective caching patterns:

- **Source files**: `"src/**/*.{ts,js,py,rs}"`
- **Configuration**: `"package.json"`, `"Cargo.toml"`, `"requirements.txt"`
- **Test files**: `"test/**/*"`, `"tests/**/*"`
- **Build configs**: `"webpack.config.js"`, `"tsconfig.json"`

### Dependency Management

Common dependency patterns:

```yaml
# Linear dependencies
lint → test → build → deploy

# Parallel execution
install → [lint, format, type-check] → build
```

## 💡 Tips for Adapting Examples

### 1. Update Commands

Replace example commands with your actual build tools:

```yaml
# From example
cmd: npm run build

# Your project might use
cmd: yarn build
cmd: pnpm build
cmd: make build
```

### 2. Adjust File Patterns

Update cache file patterns for your project structure:

```yaml
# Standard structure
cache_files: ["src/**/*"]

# Custom structure
cache_files: ["lib/**/*", "app/**/*"]
```

### 3. Environment Variables

Add project-specific environment configuration:

```yaml
tasks:
  deploy:
    cmd: deploy-script
    env:
      API_URL: ${API_URL:-https://api.example.com}
      ENVIRONMENT: ${NODE_ENV:-production}
```

### 4. Platform-Specific Commands

Handle different operating systems:

```yaml
# Cross-platform
clean:
  cmd: rm -rf dist || rmdir /s dist

# Or use platform-specific tools
clean:
  cmd: npx rimraf dist
```

## 🔧 Common Patterns

### Development Workflow

```yaml
# Fast feedback loop
quick-check:
  depends_on: [lint, test-unit]

# Pre-commit validation
pre-commit:
  depends_on: [format-check, lint, test]

# Complete CI pipeline
ci:
  depends_on: [lint, test, build, security-check]
```

### Multi-Stage Builds

```yaml
# Progressive validation
check → lint → test-unit → test-integration → build → deploy
```

### Parallel Execution

```yaml
# Independent tasks run in parallel
ci:
  depends_on: [lint, test, type-check, security-scan]
```

## 🐛 Troubleshooting Examples

### Task Not Running

Check dependencies and file paths:

```bash
# Debug task execution
taskrush --verbose your-task

# List all tasks and dependencies
taskrush --list
```

### Caching Issues

Verify cache file patterns:

```yaml
# Too broad (inefficient)
cache_files: ["**/*"]

# Too narrow (misses dependencies)
cache_files: ["src/main.js"]

# Just right
cache_files: ["src/**/*.js", "package.json"]
```

### Dependency Loops

Avoid circular dependencies:

```yaml
# ❌ Circular dependency
a: { depends_on: [b] }
b: { depends_on: [a] }

# ✅ Linear dependency
a: { depends_on: [b] }
b: { depends_on: [c] }
```

## 📚 Further Reading

- [TaskRush Configuration Reference](../README.md#configuration-reference)
- [Advanced Usage Guide](../README.md#advanced-usage)
- [Migration Guides](../README.md#migration-guides)

## 🤝 Contributing Examples

Have a great TaskRush configuration? Share it with the community!

1. Create a new directory in `examples/`
2. Add your `.rush` file with comments
3. Include a brief README explaining the use case
4. Submit a pull request

We especially welcome examples for:
- Go applications
- Java/Kotlin projects
- C/C++ builds
- Mobile development (React Native, Flutter)
- Infrastructure as Code (Terraform, Pulumi)
- Machine Learning workflows
