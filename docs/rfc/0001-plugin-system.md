# RFC 0001: Plugin System Architecture

- **Start Date**: 2025-07-22
- **Status**: Draft
- **Author**: TaskRush Team

## Summary

This RFC proposes a comprehensive plugin system for TaskRush that allows users to extend functionality through Rust-based plugins and WebAssembly modules.

## Motivation

TaskRush has established itself as a solid task runner, but users need extensibility for:

1. **Custom Integrations**: Slack notifications, GitHub status updates, Jira transitions
2. **Specialized Tools**: Language-specific linters, custom deployment scripts
3. **Enterprise Features**: Security scanning, compliance checks, audit logging
4. **Community Contributions**: Shared solutions for common problems

## Detailed Design

### Plugin Architecture

```rust
// Core plugin trait
pub trait TaskRushPlugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    
    // Plugin lifecycle
    fn initialize(&mut self, config: &PluginConfig) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
    
    // Task lifecycle hooks
    fn before_task(&mut self, context: &TaskContext) -> Result<()>;
    fn after_task(&mut self, context: &TaskContext, result: &TaskResult) -> Result<()>;
    fn on_task_failure(&mut self, context: &TaskContext, error: &TaskError) -> Result<()>;
    
    // Custom commands
    fn commands(&self) -> Vec<PluginCommand>;
}

// Plugin context passed to hooks
pub struct TaskContext {
    pub task_name: String,
    pub command: String,
    pub working_dir: PathBuf,
    pub env_vars: HashMap<String, String>,
    pub config: TaskConfig,
}

// Plugin command definition
pub struct PluginCommand {
    pub name: String,
    pub description: String,
    pub handler: Box<dyn Fn(&[String]) -> Result<()>>,
}
```

### Plugin Types

#### 1. Native Rust Plugins
- Compiled as dynamic libraries (.so, .dylib, .dll)
- Full access to TaskRush APIs
- Best performance
- Complex deployment

#### 2. WebAssembly Plugins
- Compiled to WASM for portability
- Sandboxed execution
- Easy distribution
- Limited API access

#### 3. External Plugins
- Separate processes communicating via JSON-RPC
- Any language (Python, Node.js, Go)
- Isolated execution
- Network overhead

### Plugin Configuration

```yaml
# .rush file with plugins
plugins:
  # Global plugins
  - name: slack-notify
    version: "^1.0"
    config:
      webhook_url: ${SLACK_WEBHOOK}
      channels: ["#builds", "#deployments"]
  
  - name: security-scanner
    type: wasm
    path: ./plugins/security-scanner.wasm
    config:
      severity_threshold: "medium"

tasks:
  deploy:
    cmd: kubectl apply -f deployment.yaml
    # Task-specific plugin config
    plugins:
      slack-notify:
        message: "🚀 Deployed ${VERSION} to production"
        on_success: true
        on_failure: true
```

### Plugin Discovery and Installation

```bash
# Plugin management commands
taskrush plugin search security          # Search plugin registry
taskrush plugin install slack-notify     # Install from registry
taskrush plugin install ./my-plugin.so   # Install local plugin
taskrush plugin list                      # List installed plugins
taskrush plugin update                    # Update all plugins
taskrush plugin remove slack-notify      # Remove plugin

# Plugin development
taskrush plugin new my-plugin             # Create plugin template
taskrush plugin build                     # Build plugin
taskrush plugin test                      # Test plugin
taskrush plugin publish                   # Publish to registry
```

### Plugin Registry

```rust
// Plugin manifest
#[derive(Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    
    // Runtime requirements
    pub taskrush_version: String,
    pub rust_version: Option<String>,
    
    // Plugin metadata
    pub plugin_type: PluginType,
    pub entry_point: String,
    pub dependencies: Vec<PluginDependency>,
    
    // Permissions requested
    pub permissions: Vec<Permission>,
}

#[derive(Serialize, Deserialize)]
pub enum Permission {
    FileSystem { paths: Vec<String> },
    Network { hosts: Vec<String> },
    Environment { vars: Vec<String> },
    Process { commands: Vec<String> },
}
```

### Security Model

1. **Sandboxing**: WASM plugins run in isolated environments
2. **Permissions**: Explicit permission system for filesystem, network, environment
3. **Code Signing**: Plugins must be signed by trusted developers
4. **Audit Trail**: Log all plugin actions for security review
5. **Safe Defaults**: Minimal permissions by default

### Plugin Examples

#### Slack Notification Plugin

```rust
use taskrush_plugin_api::*;

pub struct SlackNotifyPlugin {
    webhook_url: String,
    channels: Vec<String>,
}

impl TaskRushPlugin for SlackNotifyPlugin {
    fn name(&self) -> &str { "slack-notify" }
    fn version(&self) -> &str { "1.0.0" }
    
    fn initialize(&mut self, config: &PluginConfig) -> Result<()> {
        self.webhook_url = config.get_string("webhook_url")?;
        self.channels = config.get_array("channels")?;
        Ok(())
    }
    
    fn after_task(&mut self, ctx: &TaskContext, result: &TaskResult) -> Result<()> {
        let message = match result.status {
            TaskStatus::Success => format!("✅ Task '{}' completed successfully", ctx.task_name),
            TaskStatus::Failure => format!("❌ Task '{}' failed", ctx.task_name),
        };
        
        self.send_slack_message(&message)?;
        Ok(())
    }
}
```

#### Git Hook Plugin

```rust
pub struct GitHookPlugin {
    hooks: HashMap<String, Vec<String>>,
}

impl TaskRushPlugin for GitHookPlugin {
    fn before_task(&mut self, ctx: &TaskContext) -> Result<()> {
        if let Some(hooks) = self.hooks.get(&ctx.task_name) {
            for hook in hooks {
                self.run_git_hook(hook, ctx)?;
            }
        }
        Ok(())
    }
}
```

## Implementation Plan

### Phase 1: Core Plugin Infrastructure
- [ ] Define plugin API traits and types
- [ ] Implement plugin loader for native Rust plugins
- [ ] Basic plugin configuration parsing
- [ ] Plugin lifecycle management

### Phase 2: WebAssembly Support
- [ ] WASM runtime integration (wasmtime)
- [ ] WASM plugin API bindings
- [ ] Sandboxing and security model
- [ ] WASM plugin examples

### Phase 3: Plugin Registry
- [ ] Plugin manifest format
- [ ] Plugin discovery and installation
- [ ] Version management and updates
- [ ] Plugin signing and verification

### Phase 4: Advanced Features
- [ ] External process plugins
- [ ] Plugin hot-reloading
- [ ] Plugin performance metrics
- [ ] Visual plugin management

## Drawbacks

1. **Complexity**: Plugins add significant architectural complexity
2. **Security Risks**: Third-party code execution requires careful security design
3. **Compatibility**: Plugin API changes may break existing plugins
4. **Performance**: Plugin overhead may slow down task execution
5. **Support Burden**: More surface area for bugs and support requests

## Alternatives

### 1. Shell Script Hooks
Simple but limited:
```yaml
tasks:
  deploy:
    cmd: kubectl apply -f deployment.yaml
    before: ./scripts/pre-deploy.sh
    after: ./scripts/post-deploy.sh
```

### 2. External Command Integration
Use existing tools:
```yaml
tasks:
  notify:
    cmd: slack-cli send "Build completed"
    depends_on: [build]
```

### 3. Configuration-Only Extensions
Extend through configuration:
```yaml
notifications:
  slack:
    webhook: ${SLACK_WEBHOOK}
    on_success: [build, deploy]
    on_failure: [test]
```

## Unresolved Questions

1. **Plugin API Stability**: How do we maintain compatibility across TaskRush versions?
2. **Performance Impact**: What's the acceptable overhead for plugin execution?
3. **Distribution Model**: Should we host our own registry or use existing ones?
4. **Cross-Platform Support**: How do we handle platform-specific plugins?
5. **Plugin Communication**: Should plugins be able to communicate with each other?

## Future Extensions

- **Plugin Composition**: Chain multiple plugins together
- **Plugin Marketplace**: Visual browser for plugins with ratings/reviews
- **Plugin Templates**: Scaffolding for common plugin patterns
- **Plugin Testing Framework**: Automated testing for plugin compatibility
- **Plugin Analytics**: Usage statistics and performance monitoring

---

*This RFC is open for community feedback. Please share your thoughts on the TaskRush GitHub discussions.*
