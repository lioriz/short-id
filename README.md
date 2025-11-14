# short-id

[![Crates.io](https://img.shields.io/crates/v/short-id.svg)](https://crates.io/crates/short-id)
[![Documentation](https://docs.rs/short-id/badge.svg)](https://docs.rs/short-id)

A tiny Rust library for generating short, URL-safe, unique identifiers.

## What is this?

Unlike full UUIDs (which are 36 characters and include hyphens), `short-id` gives you compact 14-character strings that are easy to copy, paste, and use in URLs.

This library has two main goals:

1. **Make it very easy to generate short random IDs** for things like request IDs, user-facing tokens, test data, and log correlation.

2. **Provide an optional "ordered" variant** where IDs include a timestamp prefix, so when you sort them as strings they roughly follow creation time.

It is intentionally minimal - no configuration, no custom alphabets, no complex API. You just call:
- `short_id()` for a random URL-safe ID
- `short_id_ordered()` for a URL-safe ID that is roughly time-ordered

This crate is for you if you want something simpler and shorter than UUIDs, and you don't need strict UUID semantics or reversibility.

## Quick Start

```rust
use short_id::short_id;

// Generate a random ID
let id = short_id();
println!("Request ID: {}", id);
// Example output: "X7K9mP2nQwE-Tg"
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
short-id = "0.1"
```

## Usage

### Random IDs

Perfect for request IDs, session tokens, or any unique identifier:

```rust
use short_id::short_id;

let request_id = short_id();
let session_id = short_id();
let token = short_id();

// Each ID is 14 characters, URL-safe, and unique
assert_eq!(request_id.len(), 14);
```

### Time-Ordered IDs

For IDs that roughly sort by creation time:

```rust
use short_id::short_id_ordered;

let id1 = short_id_ordered();
std::thread::sleep(std::time::Duration::from_secs(1));
let id2 = short_id_ordered();

// IDs sort chronologically
assert!(id1 < id2);
```

This is useful for:
- Log entries that should sort by time
- Event IDs in chronological order  
- Resource IDs where temporal order matters

## API

### `short_id()`

Generates a random 10-byte ID encoded with base64url (14 characters).

- Always exactly 14 characters
- URL-safe characters only: `A-Z`, `a-z`, `0-9`, `-`, `_`
- Cryptographically secure random
- Works in both `std` and `no_std` (with `alloc`)

### `short_id_ordered()`

Generates an ID with a timestamp prefix (4 bytes) plus random bytes (6 bytes), base64url-encoded to 14 characters.

- Includes Unix timestamp, so IDs roughly sort by creation time
- Still cryptographically unique due to random component
- Requires the `std` feature (enabled by default)

## `no_std` Support

This crate works in `no_std` environments with `alloc`:

```toml
[dependencies]
short-id = { version = "0.1", default-features = false }
```

**Note:** In `no_std` mode, only `short_id()` is available. The `short_id_ordered()` function requires the `std` feature because it needs `std::time::SystemTime`.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### For Maintainers

Releases are automated via GitHub Actions. See [RELEASE.md](RELEASE.md) for full details.

**Quick release:**
```bash
# Update Cargo.toml and CHANGELOG.md
git add Cargo.toml CHANGELOG.md
git commit -m "Release v0.1.1"
git push origin main

# Tag triggers automatic publish to crates.io
git tag v0.1.1
git push origin v0.1.1
```

Or use the release script:
```bash
./scripts/release.sh 0.1.1
```

## License

MIT
