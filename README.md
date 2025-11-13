# short-id

[![Crates.io](https://img.shields.io/crates/v/short-id.svg)](https://crates.io/crates/short-id)
[![Documentation](https://docs.rs/short-id/badge.svg)](https://docs.rs/short-id)

A tiny Rust library for generating short, URL-safe IDs.

## Overview

`short-id` generates compact, URL-safe identifiers that are:
- **Shorter than UUIDs**: 14 characters vs 36 for UUIDs
- **URL-safe**: No special characters requiring encoding (`+`, `/`, `=`)
- **Cryptographically secure**: Uses `OsRng` for random bytes
- **Minimal**: Just two functions, no configuration needed
- **`no_std` compatible**: Works in embedded environments with `alloc`

## Quick Start

```rust
use short_id::short_id;

// Generate a random ID
let id = short_id();
println!("Generated ID: {}", id);
// Example output: "X7K9mP2nQwE-Tg"

// IDs are always 14 characters long
assert_eq!(id.len(), 14);

// IDs are URL-safe (no special characters)
assert!(!id.contains('+') && !id.contains('/') && !id.contains('='));
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
short-id = "0.1"
```

## Usage

### Random IDs

```rust
use short_id::short_id;

let id = short_id();
println!("Random ID: {}", id);
// Example: "X7K9mP2nQwE-Tg"
```

### Time-Ordered IDs

For IDs that include temporal information:

```rust
use short_id::short_id_ordered;

let id = short_id_ordered();
println!("Time-ordered ID: {}", id);
// Example: "aRZdyB6nsOeBDw"

// IDs generated at different times will differ
let id1 = short_id_ordered();
std::thread::sleep(std::time::Duration::from_secs(1));
let id2 = short_id_ordered();
assert_ne!(id1, id2);
```

## API

### `short_id()`

Generates a random 10-byte ID encoded with base64url (14 characters).

- **Length**: Always 14 characters
- **Characters**: `A-Z`, `a-z`, `0-9`, `-`, `_` (URL-safe)
- **Uniqueness**: Cryptographically random, collision probability is negligible
- **Availability**: Works in both `std` and `no_std` (with `alloc`) environments

### `short_id_ordered()`

Generates an ID with:
- First 4 bytes: Unix timestamp (u32, big-endian)
- Next 6 bytes: random bytes

The result is base64url-encoded (14 characters).

- **Availability**: Requires the `std` feature (enabled by default)
- **Uniqueness**: Timestamp + random bytes ensure uniqueness even within the same second
- **Ordering caveat**: While IDs include temporal information, lexicographic sorting is not guaranteed due to base64url encoding characteristics

## `no_std` Support

This crate supports `no_std` environments with `alloc`. To use in a `no_std` environment:

```toml
[dependencies]
short-id = { version = "0.1", default-features = false }
```

**Note:** The `short_id_ordered()` function requires the `std` feature (enabled by default) as it depends on `std::time::SystemTime`. In `no_std` mode, only `short_id()` is available.

## Implementation Details

- Random IDs use 10 cryptographically secure random bytes from `OsRng`
- Time-ordered IDs use 4 bytes for Unix timestamp + 6 random bytes
- All IDs are encoded with base64url (URL_SAFE_NO_PAD)
- Result is always exactly 14 characters (no padding)

## License

MIT
