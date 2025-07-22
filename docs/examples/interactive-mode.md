# Interactive Mode Examples

## Task Selection Interface

```
$ taskrush

┌─ TaskRush Interactive Mode ─────────────────────────────────────────┐
│                                                                     │
│ 📁 Project: my-awesome-app                                          │
│ 🌿 Branch: feature/new-ui                                           │
│ 📋 Available Tasks (12)                                             │
│                                                                     │
│ ❯ 🏗️  build                Build the application                    │
│   🧪 test                 Run all tests                            │
│   🔍 lint                 Run ESLint and type checking            │
│   🎨 format               Format code with Prettier               │
│   📦 package              Package for distribution                │
│   🚀 deploy               Deploy to production                    │
│   🧹 clean                Clean build artifacts                   │
│   🔄 ci                   Run complete CI pipeline                │
│   📊 analyze              Analyze bundle size                     │
│   🔧 dev                  Start development server                │
│   📚 docs                 Generate documentation                  │
│   🏥 health-check         Check service health                    │
│                                                                     │
│ [↑↓] Navigate [Enter] Run [Tab] Multi-select [?] Help [q] Quit      │
└─────────────────────────────────────────────────────────────────────┘
```

## Task Details View

```
┌─ Task Details: build ───────────────────────────────────────────────┐
│                                                                     │
│ 📝 Description: Build the application                              │
│ 💻 Command: npm run build                                          │
│ 📁 Cache Files: src/**/*.ts, package.json, tsconfig.json          │
│ 🔗 Dependencies: lint, test                                        │
│ ⏱️  Last Run: 2 minutes ago (✅ Success, 23.4s)                    │
│ 📊 Success Rate: 87% (13/15 recent runs)                          │
│                                                                     │
│ 📈 Performance History:                                            │
│   ████████████████████████████████▍ 23.4s (current)               │
│   ██████████████████████████████▌   21.2s                         │
│   ████████████████████████████████▌ 22.8s                         │
│   ██████████████████████████████▌   21.5s                         │
│   ████████████████████████████████▊ 24.1s                         │
│                                                                     │
│ [Enter] Run [d] Dependencies [h] History [←] Back                   │
└─────────────────────────────────────────────────────────────────────┘
```

## Multi-Task Selection

```
$ taskrush --tui

┌─ Select Tasks to Run ───────────────────────────────────────────────┐
│                                                                     │
│ 📋 Select multiple tasks with [Space], run with [Enter]             │
│                                                                     │
│ ❯ ☑️ lint                 Run ESLint and type checking            │
│   ☑️ test                 Run all tests                            │
│   ☑️ build                Build the application                    │
│   ☐ package              Package for distribution                │
│   ☐ deploy               Deploy to production                    │
│   ☐ docs                 Generate documentation                  │
│                                                                     │
│ 🔗 Execution Plan:                                                  │
│   1. lint, test (parallel)                                         │
│   2. build (depends on lint, test)                                 │
│                                                                     │
│ ⏱️  Estimated Time: ~45s                                            │
│ 💾 Cache Status: lint (cached), test (cached), build (needs run)    │
│                                                                     │
│ [Space] Toggle [Enter] Run [a] Select All [n] None [q] Quit         │
└─────────────────────────────────────────────────────────────────────┘
```

## Real-time Execution View

```
┌─ Running Tasks ─────────────────────────────────────────────────────┐
│                                                                     │
│ 🏃 Running CI Pipeline (3/5 tasks)                                  │
│                                                                     │
│ ✅ lint        ████████████████████████████████ 100% (5.2s)         │
│ ✅ test        ████████████████████████████████ 100% (12.1s)        │
│ 🔄 build       ██████████████████████▌         68% (15.4s)          │
│ ⏳ package     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0% (waiting)        │
│ ⏳ deploy      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0% (waiting)        │
│                                                                     │
│ 📊 Overall Progress: ████████████████▌          60% (32.7s elapsed) │
│                                                                     │
│ 📝 Current Output (build):                                         │
│ │ ✓ TypeScript compilation successful                               │
│ │ ✓ Bundling assets...                                             │
│ │ ⚠️  Large bundle detected: main.js (2.3MB)                       │
│ │ ℹ️  Consider code splitting for better performance               │
│                                                                     │
│ [Ctrl+C] Cancel [l] Logs [d] Details                               │
└─────────────────────────────────────────────────────────────────────┘
```

## Search and Filter

```
$ taskrush search

┌─ Task Search ───────────────────────────────────────────────────────┐
│                                                                     │
│ 🔍 Search: test█                                                    │
│                                                                     │
│ 📋 Matching Tasks (3):                                             │
│                                                                     │
│ ❯ 🧪 test                 Run all tests                            │
│   🧪 test:unit            Run unit tests only                      │
│   🧪 test:integration     Run integration tests                    │
│                                                                     │
│ 📋 Matching in descriptions (2):                                   │
│                                                                     │
│   📦 package              Package for distribution (runs tests)    │
│   🔄 ci                   Run complete CI pipeline (includes tests) │
│                                                                     │
│ [Type] Search [↑↓] Navigate [Enter] Select [Esc] Cancel             │
└─────────────────────────────────────────────────────────────────────┘
```

## Task History and Favorites

```
┌─ Task History ──────────────────────────────────────────────────────┐
│                                                                     │
│ 📊 Recent Runs (Last 7 days)                                       │
│                                                                     │
│ ❯ 🧪 test                 2 mins ago    ✅ 12.1s                    │
│   🏗️  build                5 mins ago    ✅ 23.4s                    │
│   🔍 lint                 5 mins ago    ✅ 5.2s                     │
│   🚀 deploy               2 hours ago   ✅ 45.6s                    │
│   🧪 test                 2 hours ago   ✅ 11.8s                    │
│   🏗️  build                2 hours ago   ✅ 22.1s                    │
│   🔄 ci                   Yesterday     ✅ 1m 23s                   │
│   🚀 deploy               Yesterday     ❌ 12.3s (failed)           │
│                                                                     │
│ ⭐ Favorites:                                                        │
│   🔄 ci                   Complete CI pipeline                     │
│   🚀 deploy               Deploy to production                     │
│                                                                     │
│ [Enter] Run Again [f] Toggle Favorite [c] Clear History [←] Back    │
└─────────────────────────────────────────────────────────────────────┘
```

## Command-line Integration

```bash
# Quick task selection with fuzzy search
$ taskrush fzf
> test
  test
  test:unit  
  test:integration

# Run with confirmation
$ taskrush deploy --confirm
⚠️  You are about to run: deploy
📝 Description: Deploy to production
🎯 Target: production environment
Continue? [y/N]: y

# Interactive mode with pre-selected tasks
$ taskrush --select=ci
[Opens TUI with CI pipeline tasks pre-selected]

# Watch mode with interactive restart
$ taskrush --watch --interactive
[Watches files, shows interactive prompt on changes]
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑↓` | Navigate up/down |
| `Enter` | Run selected task |
| `Space` | Toggle task selection (multi-select mode) |
| `Tab` | Switch between tasks and details |
| `f` | Toggle favorite |
| `h` | Show task history |
| `d` | Show task details |
| `l` | Show task logs |
| `/` | Search tasks |
| `?` | Show help |
| `q` | Quit |
| `Ctrl+C` | Cancel running task |
| `Ctrl+R` | Refresh task list |

## Configuration

```yaml
# .rush
ui:
  interactive:
    default_view: "list"        # list, grid, tree
    show_descriptions: true
    show_last_run: true
    show_performance: true
    theme: "dark"               # dark, light, auto
    
  shortcuts:
    quit: ["q", "Ctrl+C"]
    run: ["Enter"]
    multi_select: ["Space"]
    search: ["/", "Ctrl+F"]
    
  colors:
    success: "green"
    failure: "red"
    running: "blue"
    cached: "yellow"
```
