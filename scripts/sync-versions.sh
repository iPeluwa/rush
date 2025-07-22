#!/bin/bash

# Script to synchronize versions across all TaskRush components
# Usage: ./scripts/sync-versions.sh <version>

set -e

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.4"
    exit 1
fi

NEW_VERSION="$1"

# Validate version format
if [[ ! $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "❌ Invalid version format. Use semantic versioning (e.g., 1.2.3)"
    exit 1
fi

echo "🔄 Synchronizing all components to version $NEW_VERSION..."
echo

# Update Cargo.toml
echo "📦 Updating Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak

# Update npm package.json
echo "📦 Updating npm package.json..."
cd npm-package
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" package.json
rm -f package.json.bak

# Update npm install.js
echo "📦 Updating npm install.js..."
sed -i.bak "s/const VERSION = 'v.*'/const VERSION = 'v$NEW_VERSION'/" install.js
rm -f install.js.bak
cd ..

# Update VS Code extension
echo "📦 Updating VS Code extension..."
cd vscode-extension
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" package.json
rm -f package.json.bak
cd ..

# Update README version references
echo "📦 Updating README..."
sed -i.bak "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$NEW_VERSION/g" README.md
rm -f README.md.bak

# Verify Cargo.lock consistency
echo "🔍 Verifying Cargo.lock consistency..."
cargo check --quiet

echo
echo "✅ All versions synchronized to $NEW_VERSION"
echo
echo "📋 Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Commit changes: git add . && git commit -m 'Sync versions to v$NEW_VERSION'"
echo "  3. Push changes: git push"
echo "  4. Create tag: git tag v$NEW_VERSION && git push origin v$NEW_VERSION"
echo
echo "Or use the GitHub Actions Version Sync workflow for automated release."
