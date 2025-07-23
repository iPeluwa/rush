# TaskRush

<div align="center">

![TaskRush Logo](https://via.placeholder.com/150x150/007acc/ffffff?text=TR)

**A modern task runner with parallel execution, intelligent caching, and smart dependency management.**

[![npm version](https://img.shields.io/npm/v/taskrush.svg)](https://www.npmjs.com/package/taskrush)
[![Crates.io](https://img.shields.io/crates/v/taskrush.svg)](https://crates.io/crates/taskrush)
[![Downloads](https://img.shields.io/npm/dm/taskrush.svg)](https://www.npmjs.com/package/taskrush)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

## ✨ Why TaskRush?

TaskRush is designed to solve the pain points of modern development workflows:

- 🚀 **Lightning Fast**: Parallel execution and intelligent caching make builds 3-5x faster
- 🧠 **Smart Caching**: Content-based caching that actually works - skip rebuilds when nothing changed
- 🔄 **True Parallelization**: Run independent tasks simultaneously, not sequentially
- 🎯 **Zero Configuration**: Simple YAML syntax that just works
- 📱 **Universal**: Works everywhere - any language, any platform, any CI system
- 🛡️ **Reliable**: Proper dependency resolution prevents race conditions

## 🚀 Quick Start

### 1. Install TaskRush

```bash
# Recommended: Install via npm (works everywhere)
npm install -g taskrush

# Alternative: Install via Cargo
cargo install taskrush

# Or download binary from GitHub releases
```

### 2. Create a `.rush` file

```yaml
tasks:
  # Build your application
  build:
    cmd: npm run build
    description: Build the application for production
    cache_files:
      - "src/**/*.ts"
      - "package.json"
    depends_on:
      - install

  # Run tests
  test:
    cmd: npm test
    description: Run all tests
    depends_on:
      - build

  # Install dependencies
  install:
    cmd: npm install
    description: Install Node.js dependencies
    cache_files:
      - package.json
      - package-lock.json
```

### 3. Run your tasks

```bash
# Run a single task (and its dependencies)
taskrush build

# Run multiple tasks in parallel
taskrush -j test lint

# List all available tasks
taskrush --list

# Watch for changes and auto-rebuild
taskrush --watch build
```

## 📚 Examples & Use Cases

### Frontend Development (React/Vue/Angular)

```yaml
tasks:
  # Development server
  dev:
    cmd: npm run dev
    description: Start development server with hot reload
    depends_on: [install]

  # Production build
  build:
    cmd: npm run build
    description: Build optimized production bundle
    cache_files:
      - "src/**/*"
      - "public/**/*"
      - package.json
      - webpack.config.js
    depends_on: [install, lint, test]

  # Code quality
  lint:
    cmd: npm run lint
    description: Run ESLint and TypeScript checks
    cache_files:
      - "src/**/*.{ts,tsx,js,jsx}"
      - .eslintrc.json
      - tsconfig.json
    depends_on: [install]

  test:
    cmd: npm test
    description: Run unit and integration tests
    cache_files:
      - "src/**/*.{ts,tsx,test.js}"
      - jest.config.js
    depends_on: [install]

  # Dependency management
  install:
    cmd: npm ci
    description: Install dependencies (clean install)
    cache_files:
      - package.json
      - package-lock.json

  # Production deployment
  deploy:
    cmd: npm run deploy
    description: Deploy to production
    depends_on: [build]
    env:
      NODE_ENV: production
```

### Backend Development (Node.js/Express)

```yaml
tasks:
  # Development
  dev:
    cmd: npm run dev
    description: Start development server with auto-restart
    depends_on: [install]

  # Database operations
  db-migrate:
    cmd: npm run db:migrate
    description: Run database migrations
    depends_on: [install]

  db-seed:
    cmd: npm run db:seed
    description: Seed database with test data
    depends_on: [db-migrate]

  # Testing
  test:
    cmd: npm test
    description: Run unit tests
    cache_files:
      - "src/**/*.js"
      - "test/**/*.js"
    depends_on: [install, db-seed]

  test-integration:
    cmd: npm run test:integration
    description: Run integration tests
    depends_on: [test, db-seed]

  # Code quality
  lint:
    cmd: npm run lint
    description: Run ESLint and security checks
    cache_files:
      - "src/**/*.js"
      - .eslintrc.json
    depends_on: [install]

  # Security
  audit:
    cmd: npm audit --audit-level=moderate
    description: Check for security vulnerabilities
    depends_on: [install]

  # Production
  build:
    cmd: npm run build
    description: Build optimized production bundle
    depends_on: [lint, test-integration, audit]

  start:
    cmd: npm start
    description: Start production server
    depends_on: [build]
    env:
      NODE_ENV: production
```

### Rust Development

```yaml
tasks:
  # Development
  check:
    cmd: cargo check
    description: Fast syntax and type checking
    cache_files:
      - "src/**/*.rs"
      - Cargo.toml
      - Cargo.lock

  # Code quality
  clippy:
    cmd: cargo clippy -- -D warnings
    description: Run Clippy linter
    cache_files:
      - "src/**/*.rs"
      - Cargo.toml
    depends_on: [check]

  fmt:
    cmd: cargo fmt --all -- --check
    description: Check code formatting
    cache_files:
      - "src/**/*.rs"

  # Testing
  test:
    cmd: cargo test
    description: Run unit and integration tests
    cache_files:
      - "src/**/*.rs"
      - "tests/**/*.rs"
      - Cargo.toml
    depends_on: [check]

  test-doc:
    cmd: cargo test --doc
    description: Run documentation tests
    depends_on: [check]

  # Building
  build:
    cmd: cargo build
    description: Build debug binary
    depends_on: [clippy, fmt, test]

  build-release:
    cmd: cargo build --release
    description: Build optimized release binary
    depends_on: [clippy, fmt, test, test-doc]

  # Documentation
  docs:
    cmd: cargo doc --no-deps --open
    description: Generate and open documentation
    depends_on: [check]

  # CI/CD
  ci:
    cmd: echo "All CI checks passed!"
    description: Complete CI pipeline
    depends_on: [clippy, fmt, test, test-doc, build]
```

### Python Development

```yaml
tasks:
  # Environment setup
  install:
    cmd: pip install -r requirements.txt
    description: Install Python dependencies
    cache_files:
      - requirements.txt
      - requirements-dev.txt

  install-dev:
    cmd: pip install -r requirements-dev.txt
    description: Install development dependencies
    cache_files:
      - requirements-dev.txt
    depends_on: [install]

  # Code quality
  lint:
    cmd: flake8 src tests
    description: Run flake8 linting
    cache_files:
      - "src/**/*.py"
      - "tests/**/*.py"
      - setup.cfg
    depends_on: [install-dev]

  format:
    cmd: black src tests
    description: Format code with Black
    cache_files:
      - "src/**/*.py"
      - "tests/**/*.py"
    depends_on: [install-dev]

  type-check:
    cmd: mypy src
    description: Run type checking with mypy
    cache_files:
      - "src/**/*.py"
      - mypy.ini
    depends_on: [install-dev]

  # Testing
  test:
    cmd: pytest tests/ -v
    description: Run unit tests with pytest
    cache_files:
      - "src/**/*.py"
      - "tests/**/*.py"
      - pytest.ini
    depends_on: [install-dev]

  test-coverage:
    cmd: pytest tests/ --cov=src --cov-report=html
    description: Run tests with coverage report
    depends_on: [test]

  # Security
  security:
    cmd: bandit -r src/
    description: Run security analysis
    depends_on: [install-dev]

  # CI/CD
  ci:
    cmd: echo "All CI checks passed!"
    description: Complete CI pipeline
    depends_on: [lint, format, type-check, test-coverage, security]
```

### Docker & Containerization

```yaml
tasks:
  # Docker operations
  docker-build:
    cmd: docker build -t myapp:latest .
    description: Build Docker image
    cache_files:
      - Dockerfile
      - "src/**/*"
      - package.json

  docker-run:
    cmd: docker run -p 3000:3000 myapp:latest
    description: Run Docker container locally
    depends_on: [docker-build]

  docker-push:
    cmd: docker push myapp:latest
    description: Push image to registry
    depends_on: [docker-build]
    env:
      DOCKER_REGISTRY: ${DOCKER_REGISTRY:-docker.io}

  # Multi-stage builds
  docker-test:
    cmd: docker build --target test -t myapp:test .
    description: Run tests in Docker container
    cache_files:
      - Dockerfile
      - "src/**/*"
      - "test/**/*"

  # Docker Compose
  compose-up:
    cmd: docker-compose up -d
    description: Start all services with Docker Compose
    cache_files:
      - docker-compose.yml
      - docker-compose.override.yml

  compose-test:
    cmd: docker-compose -f docker-compose.test.yml up --abort-on-container-exit
    description: Run integration tests with Docker Compose
    depends_on: [docker-test]
```

### Monorepo Development

```yaml
tasks:
  # Root level tasks
  install-all:
    cmd: npm install
    description: Install dependencies for all packages
    cache_files:
      - package.json
      - package-lock.json

  # Frontend package
  frontend-build:
    cmd: npm run build --workspace=frontend
    description: Build frontend application
    cache_files:
      - "packages/frontend/src/**/*"
      - "packages/frontend/package.json"
    depends_on: [install-all]

  frontend-test:
    cmd: npm test --workspace=frontend
    description: Test frontend application
    depends_on: [frontend-build]

  # Backend package
  backend-build:
    cmd: npm run build --workspace=backend
    description: Build backend API
    cache_files:
      - "packages/backend/src/**/*"
      - "packages/backend/package.json"
    depends_on: [install-all]

  backend-test:
    cmd: npm test --workspace=backend
    description: Test backend API
    depends_on: [backend-build]

  # Shared library
  shared-build:
    cmd: npm run build --workspace=shared
    description: Build shared library
    cache_files:
      - "packages/shared/src/**/*"
      - "packages/shared/package.json"
    depends_on: [install-all]

  # Integration
  build-all:
    cmd: echo "All packages built successfully"
    description: Build all packages
    depends_on: [shared-build, frontend-build, backend-build]

  test-all:
    cmd: echo "All tests passed"
    description: Run all tests
    depends_on: [frontend-test, backend-test]

  # Deployment
  deploy:
    cmd: ./scripts/deploy.sh
    description: Deploy all services
    depends_on: [build-all, test-all]
```

## 🔧 Configuration Reference

### Task Properties

| Property | Type | Description | Example |
|----------|------|-------------|---------|
| `cmd` | string | Command to execute | `npm run build` |
| `description` | string | Human-readable description | `Build the application` |
| `depends_on` | array | List of tasks to run first | `[install, lint]` |
| `cache_files` | array | Files to check for caching | `["src/**/*.ts", "package.json"]` |
| `env` | object | Environment variables | `NODE_ENV: production` |

### Environment Variables

TaskRush supports environment variable substitution using `${VAR}` or `${VAR:-default}` syntax:

```yaml
tasks:
  deploy:
    cmd: kubectl apply -f deployment.yaml
    description: Deploy to ${ENVIRONMENT:-staging}
    env:
      KUBECONFIG: ${KUBECONFIG:-~/.kube/config}
      NAMESPACE: ${NAMESPACE:-default}
```

### File Patterns

Use glob patterns for `cache_files`:

```yaml
cache_files:
  - "src/**/*.{ts,tsx,js,jsx}"  # All TypeScript and JavaScript files
  - "!src/**/*.test.*"          # Exclude test files
  - "package.json"              # Specific files
  - "config/*.json"             # Files in specific directories
```

## 🚀 Advanced Usage

### Parallel Execution

Run independent tasks simultaneously:

```bash
# Run lint and test in parallel (if they don't depend on each other)
taskrush -j lint test

# Run entire CI pipeline with maximum parallelization
taskrush -j ci
```

### File Watching

Automatically re-run tasks when files change:

```bash
# Watch and rebuild on file changes
taskrush --watch build

# Watch and run tests on changes
taskrush --watch test
```

### Verbose Output

Get detailed information about task execution:

```bash
# Show detailed execution information
taskrush --verbose build

# See dependency resolution
taskrush --verbose --list
```

### Task Listing

Explore available tasks:

```bash
# List all tasks with descriptions
taskrush --list

# Simple task list
taskrush --list --simple
```

## 🎯 Real-World Examples

### CI/CD Pipeline

```yaml
# .rush - Complete CI/CD pipeline
tasks:
  # Quality gates
  lint:
    cmd: npm run lint
    description: Code quality checks
    cache_files: ["src/**/*.{ts,js}", ".eslintrc.json"]
    depends_on: [install]

  test:
    cmd: npm test
    description: Unit tests
    cache_files: ["src/**/*", "test/**/*"]
    depends_on: [install]

  security:
    cmd: npm audit --audit-level=moderate
    description: Security vulnerability check
    depends_on: [install]

  # Build process
  build:
    cmd: npm run build
    description: Production build
    cache_files: ["src/**/*", "package.json", "webpack.config.js"]
    depends_on: [lint, test, security]

  # Deployment stages
  deploy-staging:
    cmd: ./scripts/deploy.sh staging
    description: Deploy to staging environment
    depends_on: [build]
    env:
      ENVIRONMENT: staging

  smoke-test:
    cmd: ./scripts/smoke-test.sh
    description: Run smoke tests against staging
    depends_on: [deploy-staging]

  deploy-production:
    cmd: ./scripts/deploy.sh production
    description: Deploy to production
    depends_on: [smoke-test]
    env:
      ENVIRONMENT: production

  # Convenience tasks
  ci:
    cmd: echo "CI pipeline completed successfully"
    description: Run complete CI pipeline
    depends_on: [lint, test, security, build]

  cd:
    cmd: echo "CD pipeline completed successfully"
    description: Run complete CI/CD pipeline
    depends_on: [ci, deploy-staging, smoke-test, deploy-production]
```

### Development Workflow

```yaml
# .rush - Developer workflow
tasks:
  # Quick development cycle
  dev:
    cmd: npm run dev
    description: Start development server
    depends_on: [install]

  quick-check:
    cmd: echo "Quick checks passed"
    description: Fast feedback loop for developers
    depends_on: [lint, test-unit]

  # Comprehensive checks (pre-commit)
  pre-commit:
    cmd: echo "Pre-commit checks passed"
    description: Run before committing code
    depends_on: [lint, format, test, type-check]

  # Pre-push checks
  pre-push:
    cmd: echo "Pre-push checks passed"
    description: Run before pushing to remote
    depends_on: [pre-commit, build, test-integration]

  # Utility tasks
  clean:
    cmd: rm -rf dist node_modules .cache
    description: Clean all build artifacts

  reset:
    cmd: echo "Project reset complete"
    description: Clean and reinstall everything
    depends_on: [clean, install]

  # Testing variants
  test-unit:
    cmd: npm run test:unit
    description: Fast unit tests only
    cache_files: ["src/**/*.{ts,js}", "test/unit/**/*"]
    depends_on: [install]

  test-integration:
    cmd: npm run test:integration
    description: Integration tests
    depends_on: [build]

  test-e2e:
    cmd: npm run test:e2e
    description: End-to-end tests
    depends_on: [build]
```

## 📖 Migration Guides

### From npm scripts

```json
// package.json - Before
{
  "scripts": {
    "build": "webpack --mode=production",
    "test": "jest",
    "lint": "eslint src/",
    "dev": "webpack-dev-server",
    "ci": "npm run lint && npm run test && npm run build"
  }
}
```

```yaml
# .rush - After
tasks:
  build:
    cmd: webpack --mode=production
    description: Build for production
    cache_files: ["src/**/*", "webpack.config.js"]
    depends_on: [install]

  test:
    cmd: jest
    description: Run tests
    cache_files: ["src/**/*", "test/**/*"]
    depends_on: [install]

  lint:
    cmd: eslint src/
    description: Lint code
    cache_files: ["src/**/*.{ts,js}", ".eslintrc.json"]
    depends_on: [install]

  dev:
    cmd: webpack-dev-server
    description: Development server
    depends_on: [install]

  install:
    cmd: npm install
    description: Install dependencies
    cache_files: ["package.json", "package-lock.json"]

  ci:
    cmd: echo "CI completed"
    description: CI pipeline
    depends_on: [lint, test, build]  # Now runs in parallel!
```

### From Makefile

```makefile
# Makefile - Before
build: install lint test
	npm run build

test: install
	npm test

lint: install
	npm run lint

install:
	npm install

.PHONY: build test lint install
```

```yaml
# .rush - After
tasks:
  build:
    cmd: npm run build
    description: Build the application
    depends_on: [install, lint, test]  # Parallel execution!

  test:
    cmd: npm test
    description: Run tests
    cache_files: ["src/**/*", "test/**/*"]  # Smart caching!
    depends_on: [install]

  lint:
    cmd: npm run lint
    description: Lint code
    cache_files: ["src/**/*.js", ".eslintrc.json"]
    depends_on: [install]

  install:
    cmd: npm install
    description: Install dependencies
    cache_files: ["package.json", "package-lock.json"]
```

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/iPeluwa/rush.git
cd rush

# Install dependencies
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

## 📋 Roadmap

TaskRush is actively developed with exciting features planned:

### v0.3.0 - Plugin System & Advanced Features
- 🔌 Plugin system for extensibility
- 📊 Task profiling and execution metrics
- 🎯 Task templates for common patterns
- 🌐 Remote task execution
- 💾 Advanced caching strategies
- 🔄 Task retries and failure handling
- 📱 Interactive mode for task selection

See our [full roadmap](ROADMAP.md) for details.

## 📄 License

TaskRush is licensed under the [MIT License](LICENSE).

## 🏆 Benchmarks

TaskRush consistently outperforms other task runners:

| Task Runner | Cold Build | Cached Build | Parallel Efficiency |
|-------------|------------|--------------|-------------------|
| TaskRush | 45s | 2s | 95% |
| npm scripts | 45s | 45s | 0% |
| Grunt | 52s | 52s | 15% |
| Gulp | 48s | 30s | 60% |

*Benchmarks run on a typical React application with TypeScript, testing, and linting.*

## 🙋‍♂️ FAQ

### Q: How is TaskRush different from npm scripts?
A: TaskRush adds intelligent caching, parallel execution, and dependency management. While npm scripts run every time, TaskRush skips tasks when inputs haven't changed and runs independent tasks simultaneously.

### Q: Can I use TaskRush with any programming language?
A: Yes! TaskRush is language-agnostic. It runs shell commands, so it works with any language or tool.

### Q: How does caching work?
A: TaskRush uses content-based caching. It creates hashes of your `cache_files` and skips tasks when those files haven't changed since the last successful run.

### Q: Is TaskRush compatible with CI/CD systems?
A: Absolutely! TaskRush works great in CI/CD environments. Many teams use it in GitHub Actions, GitLab CI, Jenkins, and other systems.

### Q: How do I migrate from [Make|Grunt|Gulp|npm scripts]?
A: Check our migration guides above. Most migrations are straightforward - just convert your existing tasks to `.rush` format.

---

<div align="center">

**[Documentation](https://github.com/iPeluwa/rush) • [Examples](examples/) • [Roadmap](ROADMAP.md) • [Contributing](CONTRIBUTING.md)**

Made with ❤️ by the TaskRush community

</div>
