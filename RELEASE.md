# Release Process

This document describes how to release new versions of the `short-id` crate.

## Semantic Versioning

We follow [Semantic Versioning](https://semver.org/):

```
MAJOR.MINOR.PATCH
```

**When to bump each number:**

- **PATCH** (0.1.0 → 0.1.1)
  - Bug fixes
  - Documentation improvements
  - Internal refactoring
  - No API changes

- **MINOR** (0.1.0 → 0.2.0)
  - New features (backward compatible)
  - New functions added
  - Deprecations (but not removals)

- **MAJOR** (1.0.0 → 2.0.0)
  - Breaking changes to public API
  - Function signature changes
  - Removing deprecated features
  - Changing behavior in incompatible ways

**Note:** Before 1.0.0, MINOR versions may contain breaking changes, but we try to avoid surprises.

## Setup (One-Time)

### Security Setup (Highly Recommended)

Before your first release, configure repository protections to ensure only authorized people can trigger releases:

**📖 See [docs/SECURITY_SETUP.md](docs/SECURITY_SETUP.md) for detailed instructions.**

**Quick checklist:**
- [ ] Set up tag protection rules (Settings → Tags)
- [ ] Enable branch protection for `main` (Settings → Branches)
- [ ] Configure environment protection (Settings → Environments)

### crates.io Token Setup

Before you can use the automated release workflow, you need to set up a crates.io token:

1. **Get your crates.io API token:**
   - Go to https://crates.io/me
   - Click "Account Settings" → "API Tokens"
   - Create a new token with "publish-update" scope
   - Copy the token (you won't see it again!)

2. **Add token to GitHub Secrets:**
   - Go to your repository on GitHub
   - Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `CARGO_REGISTRY_TOKEN`
   - Value: paste your crates.io token
   - Click "Add secret"

This only needs to be done once.

## Release Workflow

### Option A: Automated Release (Recommended)

This workflow uses GitHub Actions to automatically publish to crates.io when you push a tag.

1. **Develop and merge changes** to `main`
2. **Update version and changelog:**
   ```bash
   # Edit Cargo.toml (bump version)
   # Edit CHANGELOG.md (add release notes)
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release v0.1.1"
   git push origin main
   ```
3. **Tag the release:**
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```
4. **GitHub Actions automatically:**
   - Runs all tests and checks
   - Publishes to crates.io
   - Creates a GitHub Release with changelog notes

That's it! The release happens automatically.

### Option B: Manual Release

If you prefer manual control:

### Step 1: Develop the Changes

Work on a feature branch:

```bash
git checkout -b feature/add-something
# Make your changes
git add -A
git commit -m "Add new feature"
git push origin feature/add-something
```

Open a PR, get it reviewed, and merge into `main`.

### Step 2: Prepare the Release

On `main`, prepare the release commit:

1. **Update version in `Cargo.toml`:**

```toml
[package]
version = "0.1.1"  # Was 0.1.0
```

2. **Update `CHANGELOG.md`:**

Add the new version with date and changes:

```markdown
## [0.1.1] - 2025-XX-XX

### Fixed
- Fixed bug in short_id() generation

### Added
- New helper function for X

[0.1.1]: https://github.com/lioriz/short-id/compare/v0.1.0...v0.1.1
```

3. **Commit the version bump:**

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Release v0.1.1"
git push origin main
```

### Step 3: Tag the Release

Create and push a git tag:

```bash
git tag v0.1.1
git push origin v0.1.1
```

### Step 4: Publish to crates.io

Run a dry-run first to check everything:

```bash
cargo publish --dry-run
```

If that passes, publish for real:

```bash
cargo publish
```

**Important:** You can never overwrite a published version. Once `0.1.1` is published, it's permanent.

### Step 5: Create GitHub Release (Optional)

Go to https://github.com/lioriz/short-id/releases and create a new release:

1. Choose tag `v0.1.1`
2. Title: `v0.1.1`
3. Copy release notes from CHANGELOG.md
4. Publish release

## What if Something Goes Wrong?

### If you published a broken version

You can **yank** it (but not delete it):

```bash
cargo yank --vers 0.1.1
```

Yanked versions:
- Are still downloadable (for reproducibility)
- Won't be selected by Cargo for new projects
- Should be followed by a fixed version (0.1.2)

### If you haven't published yet

- Just fix the issue and amend your commit
- Update the tag: `git tag -f v0.1.1` and `git push -f origin v0.1.1`

## Pre-Release Checklist

Before publishing, ensure:

- [ ] All tests pass: `cargo test --all-features`
- [ ] Clippy is clean: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] Version bumped in `Cargo.toml`
- [ ] CHANGELOG.md updated
- [ ] Changes committed to `main`
- [ ] Git tag created
- [ ] Dry-run passes: `cargo publish --dry-run`

## Quick Reference

### Automated Release (via GitHub Actions)

```bash
# Edit Cargo.toml (bump version)
# Edit CHANGELOG.md (add release notes)
git add Cargo.toml CHANGELOG.md
git commit -m "Release v0.X.Y"
git push origin main

git tag v0.X.Y
git push origin v0.X.Y
# GitHub Actions handles the rest!
```

### Manual Release

```bash
# Full release workflow
git checkout main
git pull origin main

# Edit Cargo.toml (bump version)
# Edit CHANGELOG.md (add release notes)

git add Cargo.toml CHANGELOG.md
git commit -m "Release v0.X.Y"
git push origin main

git tag v0.X.Y
git push origin v0.X.Y

cargo publish --dry-run
cargo publish

# Create GitHub Release manually
```

### Using Release Script

```bash
./scripts/release.sh 0.X.Y
# Follow prompts (choose not to publish if using GitHub Actions)
```

## Version History

- **0.1.0** - Initial release (2025-11-13)

