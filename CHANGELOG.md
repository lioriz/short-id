# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0]

### Added
- **Advanced API for custom-length IDs**:
  - `short_id_with_bytes(num_bytes: usize) -> String` - Generate random IDs with custom byte count (1-32 bytes)
  - `short_id_ordered_with_bytes(num_bytes: usize) -> String` - Generate time-ordered IDs with custom byte count (8-32 bytes)
  - Both functions support configurable ID lengths while maintaining URL-safety and security
- Comprehensive documentation for the new advanced API:
  - Security notes about entropy and collision probability at different byte counts
  - Examples showing shorter IDs (6 bytes → 8 chars) and longer IDs (16 bytes → 22 chars)
  - Clear warnings that the standard functions are recommended for most users
- New constant `MAX_BYTES` (32) to limit maximum ID size
- 12 new unit tests covering:
  - Custom byte lengths (6, 8, 10, 16, 32 bytes)
  - URL-safety verification for all sizes
  - Uniqueness verification
  - Panic behavior for invalid inputs (0 bytes, >32 bytes, <8 bytes for ordered)
  - Timestamp inclusion for ordered IDs with custom lengths

### Changed
- **Internal refactoring** (no breaking changes to existing API):
  - Extracted `generate_random_id(num_bytes: usize)` helper function
  - Extracted `generate_ordered_id(num_bytes: usize)` helper function (requires `std`)
  - `short_id()` and `short_id_ordered()` now call the helper functions with `num_bytes = 10`
- Enhanced crate-level documentation with "Advanced: Custom Length IDs" section
- Updated README with advanced API usage guide and examples
- Improved time-ordered ID example to use microsecond sleep (2μs) to better demonstrate microsecond precision

## [0.3.0]

### Added
- **Convenience macros** for ergonomic ID generation:
  - `id!()` - Shorthand for `short_id()`
  - `ordered_id!()` - Shorthand for `short_id_ordered()`
- **`ShortId` newtype wrapper** for typed ID handling:
  - `ShortId::random()` - Creates a random ID
  - `ShortId::ordered()` - Creates a time-ordered ID
  - `as_str(&self) -> &str` - Returns string slice
  - `into_string(self) -> String` - Consumes and returns inner String
  - Implements: `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `AsRef<str>`
  - `From<String>` and `From<ShortId> for String` conversions
- Enhanced examples demonstrating all API variants
- Comprehensive integration tests for new features

### Changed
- Updated `examples/basic.rs` to showcase functions, macros, and typed wrapper
- Expanded documentation with examples for all API variants

## [0.2.3]

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

[Unreleased]: https://github.com/lioriz/short-id/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/lioriz/short-id/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/lioriz/short-id/compare/v0.2.3...v0.3.0
[0.2.0]: https://github.com/lioriz/short-id/compare/v0.1.0...v0.2.3
[0.1.0]: https://github.com/lioriz/short-id/releases/tag/v0.1.0

