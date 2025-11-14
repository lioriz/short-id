# Contributing to short-id

Thank you for your interest in contributing to `short-id`! This document provides guidelines and instructions for contributing.

## Project Goals

`short-id` is intentionally minimal. The goal is to provide:
- Two simple functions for generating short, URL-safe IDs
- No configuration, no complex API
- Clean, readable, well-tested code

When contributing, please keep this philosophy in mind.

## Getting Started

### Prerequisites

- Rust 1.56+ (for edition 2021 support)
- Git

### Setting Up

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone git@github.com:YOUR-USERNAME/short-id.git
   cd short-id
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream git@github.com:lioriz/short-id.git
   ```

### Building and Testing

```bash
# Run tests
cargo test --all-features

# Run tests in no_std mode
cargo test --no-default-features --lib

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build documentation
cargo doc --no-deps --open
```

## How to Contribute

### Reporting Bugs

Open an issue on GitHub with:
- A clear description of the bug
- Steps to reproduce
- Expected vs actual behavior
- Your Rust version (`rustc --version`)

### Suggesting Features

Before opening a feature request, consider:
- Does this fit the "minimal" philosophy?
- Is this something many users would benefit from?
- Can it be done without adding configuration?

If yes, open an issue describing:
- The use case
- Proposed API
- Why it belongs in this crate

### Pull Requests

1. **Create a branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes:**
   - Write clean, idiomatic Rust code
   - Add tests for new functionality
   - Update documentation if needed
   - Keep changes focused and atomic

3. **Test your changes:**
   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt
   ```

4. **Commit your changes:**
   ```bash
   git add -A
   git commit -m "Add feature: brief description"
   ```

5. **Push and create PR:**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then open a Pull Request on GitHub.

### PR Guidelines

- **Title:** Clear and descriptive
- **Description:** Explain what and why, not just how
- **Tests:** Include tests for new functionality
- **Documentation:** Update docs if API changes
- **Formatting:** Run `cargo fmt` before committing
- **Lints:** Fix all clippy warnings

## Code Style

- Follow Rust standard style (enforced by `rustfmt`)
- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions small and focused

### Documentation

- All public functions must have doc comments
- Include examples in doc comments
- Examples must work as doc tests
- Update README.md if public API changes

## Testing

- Write tests for all new functionality
- Aim for good coverage, not 100% coverage
- Test both `std` and `no_std` modes where applicable
- Include edge cases

### Test Structure

```rust
#[test]
fn test_descriptive_name() {
    // Arrange
    let input = ...;
    
    // Act
    let result = short_id();
    
    // Assert
    assert_eq!(result.len(), 14);
}
```

## Branching Strategy

- `main` - stable, always passing CI
- Feature branches - named `feature/description` or `fix/description`
- Release branches - not used (we tag directly on main)

## Release Process

Only maintainers can publish releases. See [RELEASE.md](RELEASE.md) for the full process.

If you want to see a release:
1. Open an issue
2. Maintainers will decide on version bump
3. Maintainers will handle the release

## Community Guidelines

- Be respectful and constructive
- Help others learn
- Give credit where due
- Keep discussions on-topic

## Questions?

- Open an issue for questions about the crate
- Tag issues with `question` label
- Check existing issues first

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be listed in release notes and can be added to a CONTRIBUTORS file upon request.

Thank you for contributing to `short-id`! 🎉

