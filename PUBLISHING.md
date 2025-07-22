# Publishing TaskRush

This document describes how to publish TaskRush to various package managers.

## NPM Publishing

TaskRush is automatically published to NPM when a GitHub release is created.

### Setup (One-time)

1. Create an NPM account and generate an access token
2. Add the token as `NPM_TOKEN` secret in GitHub repository settings
3. Ensure the npm package name is available (currently: `taskrush`)

### Publishing Process

1. **Create a Git Tag**: The release process starts with creating a Git tag
   ```bash
   git tag v0.2.1
   git push origin v0.2.1
   ```

2. **GitHub Actions Build**: This triggers the release workflow which:
   - Builds binaries for all platforms
   - Creates a GitHub release with assets
   - Triggers the NPM publishing workflow

3. **NPM Publishing**: The NPM workflow:
   - Updates the package version to match the Git tag
   - Downloads the release binaries during `npm install`
   - Publishes to NPM registry

### Manual NPM Publishing

If needed, you can publish manually:

```bash
cd npm-package

# Update version to match git tag
npm version 0.2.1 --no-git-tag-version

# Update VERSION in install.js
sed -i 's/const VERSION = .*/const VERSION = "v0.2.1";/' install.js

# Publish
npm publish
```

## Other Package Managers

### Homebrew

- Formula is automatically updated via the release workflow
- Located at `Formula/taskrush.rb`
- Users can install with: `brew install taskrush`

### Cargo (Rust)

- Published manually to crates.io
- Install with: `cargo install taskrush`

### Manual Binary Installation

Users can download binaries directly from GitHub releases:
- Linux: `taskrush-x86_64-unknown-linux-gnu.tar.gz`
- macOS: `taskrush-x86_64-apple-darwin.tar.gz` or `taskrush-aarch64-apple-darwin.tar.gz`
- Windows: `taskrush-x86_64-pc-windows-msvc.zip`

## Installation Methods Summary

| Method | Command | Platforms |
|--------|---------|-----------|
| NPM | `npm install -g taskrush` | All |
| Cargo | `cargo install taskrush` | All |
| Homebrew | `brew install taskrush` | macOS/Linux |
| Binary | Download from releases | All |

## Testing Installation

After publishing, test the installation:

```bash
# Test NPM installation
npm install -g taskrush
taskrush --version

# Test in a new project
mkdir test-project
cd test-project
echo 'tasks:\n  test:\n    cmd: echo "Hello TaskRush!"' > .rush
taskrush test
```

## Troubleshooting

### NPM Installation Issues

1. **Binary not found**: Check if the GitHub release has the expected assets
2. **Download fails**: Verify the release tag matches the VERSION in install.js
3. **Permission errors**: User may need `sudo` for global installation

### Version Mismatches

- Ensure Cargo.toml, package.json, and install.js versions are synchronized
- Git tag should match the version being published
- GitHub release should be created from the correct tag
