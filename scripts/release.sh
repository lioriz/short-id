#!/bin/bash
# Quick release script for short-id
# Usage: ./scripts/release.sh 0.1.1

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.1"
    exit 1
fi

VERSION=$1
VERSION_TAG="v$VERSION"

echo "🚀 Preparing release $VERSION"
echo ""

# Check if on main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "❌ Error: Not on main branch (currently on $BRANCH)"
    echo "Please checkout main first: git checkout main"
    exit 1
fi

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ Error: Working directory has uncommitted changes"
    echo "Please commit or stash your changes first"
    exit 1
fi

# Pull latest
echo "📥 Pulling latest changes..."
git pull origin main

# Run tests
echo ""
echo "🧪 Running tests..."
cargo test --all-features
cargo test --no-default-features --lib

# Run checks
echo ""
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo ""
echo "✨ Running fmt check..."
cargo fmt --check

# Update version in Cargo.toml
echo ""
echo "📝 Updating Cargo.toml version to $VERSION..."
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Prompt for CHANGELOG update
echo ""
echo "⚠️  Don't forget to update CHANGELOG.md with:"
echo "   - New version header: ## [$VERSION] - $(date +%Y-%m-%d)"
echo "   - List of changes"
echo "   - Comparison link at bottom"
echo ""
read -p "Press Enter when CHANGELOG.md is updated..."

# Show diff
echo ""
echo "📄 Changes to be committed:"
git diff Cargo.toml CHANGELOG.md

echo ""
read -p "Commit these changes? [y/N] " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Aborted"
    # Restore Cargo.toml
    git checkout Cargo.toml
    exit 1
fi

# Commit
echo ""
echo "💾 Committing release..."
git add Cargo.toml CHANGELOG.md
git commit -m "Release $VERSION_TAG"

# Tag
echo ""
echo "🏷️  Creating tag $VERSION_TAG..."
git tag "$VERSION_TAG"

# Dry run publish
echo ""
echo "🔬 Running publish dry-run..."
cargo publish --dry-run

echo ""
echo "✅ Pre-release checks complete!"
echo ""
echo "Next steps:"
echo "  1. Push changes:      git push origin main"
echo "  2. Push tag:          git push origin $VERSION_TAG"
echo "  3. Publish to crates.io:  cargo publish"
echo "  4. Create GitHub release at:"
echo "     https://github.com/lioriz/short-id/releases/new?tag=$VERSION_TAG"
echo ""
read -p "Push to GitHub now? [y/N] " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "📤 Pushing to GitHub..."
    git push origin main
    git push origin "$VERSION_TAG"
    
    echo ""
    read -p "Publish to crates.io now? [y/N] " -n 1 -r
    echo ""
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "📦 Publishing to crates.io..."
        cargo publish
        echo ""
        echo "🎉 Release $VERSION published successfully!"
        echo ""
        echo "Don't forget to create a GitHub release:"
        echo "https://github.com/lioriz/short-id/releases/new?tag=$VERSION_TAG"
    else
        echo ""
        echo "⏸️  Skipped crates.io publish. Run manually:"
        echo "   cargo publish"
    fi
else
    echo ""
    echo "⏸️  Skipped GitHub push. Run manually:"
    echo "   git push origin main && git push origin $VERSION_TAG"
fi

