# TaskRush

A modern task runner with parallel execution and intelligent caching.

## Installation

### Global Installation (Recommended)

```bash
npm install -g taskrush
```

### Local Installation

```bash
npm install taskrush
npx taskrush --help
```

## Quick Start

1. Create a `.rush` file in your project root:

```yaml
tasks:
  build:
    cmd: npm run build
    description: Build the application
    cache_files:
      - "src/**/*.js"
      - "package.json"
  
  test:
    cmd: npm test
    description: Run tests
    depends_on:
      - build
```

2. Run tasks:

```bash
taskrush build
taskrush test
taskrush --list    # Show all available tasks
```

## Features

- 🚀 **Parallel Execution**: Run independent tasks simultaneously
- 💾 **Intelligent Caching**: Skip tasks when inputs haven't changed
- 📋 **Dependency Management**: Automatic task ordering based on dependencies
- 🎯 **Selective Execution**: Run only the tasks you need
- 📝 **Clear Documentation**: Task descriptions and help text
- 🔍 **File Watching**: Automatic re-runs when files change
- 🌍 **Environment Variables**: Dynamic configuration support

## Command Line Usage

```bash
# Run a specific task
taskrush build

# List all available tasks
taskrush --list

# Run tasks in parallel when possible
taskrush -j build

# Watch for file changes and re-run
taskrush --watch build

# Verbose output for debugging
taskrush --verbose build

# Show help
taskrush --help
```

## Configuration

Create a `.rush` file in YAML format:

```yaml
tasks:
  task_name:
    cmd: "command to run"
    description: "Task description"
    depends_on:
      - other_task
    cache_files:
      - "file_pattern"
    env:
      KEY: value
```

## Programmatic Usage

```javascript
const { runTaskRush } = require('taskrush');

// Run a task programmatically
runTaskRush(['build'])
  .then(exitCode => {
    console.log(`Task completed with exit code: ${exitCode}`);
  })
  .catch(err => {
    console.error('Task failed:', err);
  });
```

## Examples

See the [examples directory](https://github.com/iPeluwa/rush/tree/master/examples) for sample configurations.

## Documentation

For complete documentation, visit: https://github.com/iPeluwa/rush

## License

MIT
