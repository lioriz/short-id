# short-id

[![Crates.io](https://img.shields.io/crates/v/short-id.svg)](https://crates.io/crates/short-id)
[![Documentation](https://docs.rs/short-id/badge.svg)](https://docs.rs/short-id)

A tiny Rust library for generating short, URL-safe IDs.

## Usage

```rust
use short_id::{short_id, short_id_ordered};

fn main() {
    // Generate a random 10-byte ID (14 characters after encoding)
    let id = short_id();
    println!("Random ID: {}", id);
    // Example: "X7K9mP2nQwE-Tg"

    // Generate a time-ordered ID (4-byte timestamp + 6 random bytes)
    let ordered_id = short_id_ordered();
    println!("Time-ordered ID: {}", ordered_id);
    // Example: "aRZdyB6nsOeBDw"
}
```

## Why?

- **Shorter than UUIDs**: 14 characters vs 36 for UUIDs
- **URL-safe**: Uses base64url encoding (no `+`, `/`, or `=` padding)
- **Minimal API**: Just two functions, no configuration needed
- **Secure**: Uses `OsRng` for cryptographically secure random bytes
- **Time-ordered option**: Include timestamp prefix for temporal context

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
short-id = "0.1"
```

## API

### `short_id()`

Generates a random 10-byte ID encoded with base64url (14 characters).

### `short_id_ordered()`

Generates an ID with:
- First 4 bytes: Unix timestamp (u32, big-endian)
- Next 6 bytes: random bytes

The result is base64url-encoded (14 characters). Note that while the timestamp prefix provides temporal information, lexicographic sorting is not guaranteed due to base64url encoding characteristics.

## License

MIT
