#!/bin/bash

# Script to check version consistency across all TaskRush components
# Usage: ./scripts/check-versions.sh

set -e

echo "🔍 Checking version consistency across TaskRush components..."
echo

# Get versions from different sources
CARGO_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
NPM_VERSION=$(cd npm-package && node -p "require('./package.json').version")
INSTALL_VERSION=$(cd npm-package && grep "const VERSION = " install.js | sed "s/.*'v\([^']*\)'.*/\1/")
VSCODE_VERSION=$(cd vscode-extension && node -p "require('./package.json').version")
FORMULA_VERSION=$(grep 'download/v' Formula/taskrush.rb | sed 's/.*download\/v\([0-9.]*\)\/.*/\1/')

echo "📦 Component versions:"
echo "  Cargo.toml:        $CARGO_VERSION"
echo "  npm package.json:  $NPM_VERSION"
echo "  npm install.js:    $INSTALL_VERSION"
echo "  VS Code extension: $VSCODE_VERSION"
echo "  Homebrew formula:  $FORMULA_VERSION"
echo

# Check if all versions match
ALL_MATCH=true

if [[ "$CARGO_VERSION" != "$NPM_VERSION" ]]; then
    echo "❌ Mismatch: Cargo ($CARGO_VERSION) vs npm package ($NPM_VERSION)"
    ALL_MATCH=false
fi

if [[ "$CARGO_VERSION" != "$INSTALL_VERSION" ]]; then
    echo "❌ Mismatch: Cargo ($CARGO_VERSION) vs npm install.js ($INSTALL_VERSION)"
    ALL_MATCH=false
fi

if [[ "$CARGO_VERSION" != "$VSCODE_VERSION" ]]; then
    echo "❌ Mismatch: Cargo ($CARGO_VERSION) vs VS Code extension ($VSCODE_VERSION)"
    ALL_MATCH=false
fi

if [[ "$CARGO_VERSION" != "$FORMULA_VERSION" ]]; then
    echo "❌ Mismatch: Cargo ($CARGO_VERSION) vs Homebrew formula ($FORMULA_VERSION)"
    ALL_MATCH=false
fi

if [[ "$ALL_MATCH" == "true" ]]; then
    echo "✅ All versions are synchronized: $CARGO_VERSION"
    echo
    echo "🎯 To create a new release:"
    echo "  1. Use GitHub Actions: Go to Actions → Version Sync → Run workflow"
    echo "  2. Or run: ./scripts/sync-versions.sh <new-version>"
    exit 0
else
    echo
    echo "🔧 To fix version mismatches:"
    echo "  1. Use GitHub Actions: Go to Actions → Version Sync → Run workflow"
    echo "  2. Or run: ./scripts/sync-versions.sh $CARGO_VERSION"
    exit 1
fi
