# TaskRush Language Support

This extension provides language support for TaskRush `.rush` configuration files in Visual Studio Code.

## Features

- **Syntax Highlighting**: Custom syntax highlighting for TaskRush configuration files
- **Snippets**: Code snippets for common task patterns
- **IntelliSense**: Auto-completion for task properties
- **Hover Information**: Helpful information when hovering over keywords
- **Schema Validation**: JSON schema validation for proper syntax
- **Task Runner**: Run tasks directly from the editor

## Quick Start

1. Install the extension
2. Open or create a `.rush` file
3. Start typing `task` and use the snippets
4. Use `Ctrl+Shift+P` and run "TaskRush: Run Task" to execute tasks

## Snippets

| Prefix | Description |
|--------|-------------|
| `task` | Basic task template |
| `task-deps` | Task with dependencies |
| `task-cache` | Task with caching |
| `task-full` | Complete task with all properties |
| `build` | Common build task |
| `test` | Common test task |
| `ci` | CI pipeline task |

## Example

```yaml
tasks:
  build:
    cmd: npm run build
    description: Build the application
    cache_files:
      - "src/**/*"
      - "package.json"
  
  test:
    cmd: npm test
    description: Run tests
    depends_on:
      - build
```

## Commands

- `TaskRush: Run Task` - Open task picker and run selected task

## Requirements

- TaskRush installed (`npm install -g taskrush`)

## Release Notes

### 0.2.3

- Initial release
- Syntax highlighting for `.rush` files
- Code snippets and IntelliSense
- Task runner integration
