# TaskRush Version Management

This document explains how TaskRush manages versions across all its components to ensure consistency.

## Components with Versions

TaskRush has multiple components that need to stay in sync:

1. **Cargo.toml** - Rust package version
2. **npm-package/package.json** - npm package version
3. **npm-package/install.js** - VERSION constant for binary downloads
4. **vscode-extension/package.json** - VS Code extension version
5. **Git tags** - Release tags (e.g., v0.2.3)
6. **GitHub releases** - Release names and binary versions

## Version Sync Workflow

### Automated Release (Recommended)

Use the GitHub Actions workflow for automated, consistent releases:

1. **Go to GitHub Actions** → **Version Sync** → **Run workflow**
2. **Enter the new version** (e.g., `0.2.4`)
3. **Choose if it's a pre-release** (optional)
4. **Click Run workflow**

This will:
- ✅ Update all component versions
- ✅ Create a git commit and tag
- ✅ Trigger release builds
- ✅ Publish to npm
- ✅ Create GitHub release
- ✅ Verify everything works

### Manual Process

If you prefer manual control:

```bash
# 1. Check current version status
./scripts/check-versions.sh

# 2. Sync all versions to a specific version
./scripts/sync-versions.sh 0.2.4

# 3. Review and commit changes
git diff
git add .
git commit -m "Sync versions to v0.2.4"
git push

# 4. Create and push tag
git tag v0.2.4
git push origin v0.2.4

# This triggers the automated build and publish workflows
```

## Version Checking

### Check Version Consistency

```bash
./scripts/check-versions.sh
```

This script will:
- Show all component versions
- Identify any mismatches
- Provide guidance on fixing issues

### Example Output

```
🔍 Checking version consistency across TaskRush components...

📦 Component versions:
  Cargo.toml:        0.2.3
  npm package.json:  0.2.3
  npm install.js:    0.2.3
  VS Code extension: 0.2.3

✅ All versions are synchronized: 0.2.3
```

## Troubleshooting

### Version Mismatch Detected

If versions are out of sync:

1. **Use the automated workflow**: Go to GitHub Actions → Version Sync
2. **Or use the manual script**: `./scripts/sync-versions.sh <version>`

### Failed Release

If a release fails partway through:

1. **Check the GitHub Actions logs** for specific errors
2. **Verify all secrets are set** (NPM_TOKEN, CARGO_REGISTRY_TOKEN)
3. **Run version check** to ensure consistency
4. **Re-run the workflow** if needed

### Binary Version Still Wrong

If the installed binary shows a different version:

1. **Wait for release workflow** to complete fully
2. **Check GitHub release assets** are all present
3. **Clear npm cache**: `npm cache clean --force`
4. **Reinstall**: `npm uninstall -g taskrush && npm install -g taskrush`

## Semantic Versioning

TaskRush follows [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Examples

- `0.2.4` → `0.2.5`: Bug fix
- `0.2.4` → `0.3.0`: New features
- `0.2.4` → `1.0.0`: Breaking changes or stable release

## Pre-releases

For testing versions before official release:

1. **Use pre-release flag** in GitHub Actions
2. **Or append pre-release identifier**: `0.2.4-beta.1`

Pre-releases:
- Don't trigger package manager updates
- Are marked as pre-release on GitHub
- Can be installed with `npm install -g taskrush@0.2.4-beta.1`

## Scripts Reference

| Script | Purpose |
|--------|---------|
| `./scripts/check-versions.sh` | Check version consistency |
| `./scripts/sync-versions.sh <version>` | Sync all versions |

## Workflow Reference

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| **Version Sync** | Manual | Complete version management |
| **Release** | Tag push | Build binaries and create release |
| **NPM Publish** | Tag push | Publish to npm registry |
| **Package Managers** | Tag push | Update other package managers |

## Best Practices

1. **Always use version sync** before releasing
2. **Test the automated workflow** in a fork first
3. **Check version consistency** regularly
4. **Document breaking changes** in release notes
5. **Use pre-releases** for testing major changes

## Emergency Procedures

### Rollback a Release

If a release has critical issues:

1. **Create hotfix version**: Increment patch version
2. **Use version sync workflow**: With the hotfix version
3. **Mark previous release** as pre-release on GitHub
4. **Notify users** via release notes

### Manual Version Fix

If automation fails and you need to fix versions manually:

```bash
# Update each file individually
sed -i 's/version = ".*"/version = "0.2.4"/' Cargo.toml
sed -i 's/"version": ".*"/"version": "0.2.4"/' npm-package/package.json
sed -i "s/const VERSION = 'v.*'/const VERSION = 'v0.2.4'/" npm-package/install.js
sed -i 's/"version": ".*"/"version": "0.2.4"/' vscode-extension/package.json

# Verify consistency
./scripts/check-versions.sh
```
