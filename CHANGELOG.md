# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2]

### Changed
- **BREAKING**: `short_id_ordered()` now uses **microseconds** (8 bytes) instead of seconds (4 bytes) for the timestamp
  - Provides excellent time resolution (1,000,000 steps per second) for high-frequency ID generation
  - Timestamp: 8 bytes microseconds (was 4 bytes seconds), Random: 2 bytes (was 6 bytes)
  - Still 10 bytes total, still 14 characters after encoding
  - IDs created within the same **microsecond** differ by their random component (65,536 variations)
  - Range: ~584,542 years from Unix epoch (plenty for any application)

## [0.1.0]

### Added
- Initial release of `short-id` crate
- `short_id()` function for generating random, URL-safe short IDs
  - Generates 10 random bytes encoded as 14-character base64url string
  - Cryptographically secure using `OsRng`
  - No special characters (URL-safe: only `A-Z`, `a-z`, `0-9`, `-`, `_`)
- `short_id_ordered()` function for time-ordered short IDs
  - Includes 4-byte Unix timestamp prefix + 6 random bytes
  - Requires `std` feature (enabled by default)
  - Useful for IDs with temporal information
- `no_std` support with `alloc`
  - `std` feature enabled by default
  - `short_id()` works in `no_std` + `alloc` environments
  - `short_id_ordered()` requires `std` feature
- Comprehensive documentation
  - Crate-level docs with quick start guide
  - Function-level docs with multiple examples
  - All examples validated as doctests
- Complete test coverage
  - Unit tests for both functions
  - Integration tests
  - Doc tests (10 examples)
- Example code (`examples/basic.rs`)
  - Demonstrates both `short_id()` and `short_id_ordered()`
  - Works in both `std` and `no_std` modes
- GitHub Actions CI workflow
  - Tests on stable, beta, and nightly Rust
  - Runs formatting, clippy, and all tests
  - Validates both `std` and `no_std` builds
- MIT License
- Professional README with usage examples

[Unreleased]: https://github.com/lioriz/short-id/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/lioriz/short-id/compare/v0.1.0...v0.2.2
[0.1.0]: https://github.com/lioriz/short-id/releases/tag/v0.1.0

