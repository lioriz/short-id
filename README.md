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
short-id = "0.3"
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
std::thread::sleep(std::time::Duration::from_micros(2));
let id2 = short_id_ordered();

// IDs sort chronologically (microsecond precision)
assert!(id1 < id2);
```

**Note:** The timestamp has **microsecond precision**, so IDs generated just a few microseconds apart will have different timestamps and sort correctly. IDs generated within the same microsecond will still be unique due to the random component.

This is useful for:
- Log entries that should sort by time
- Event IDs in chronological order  
- Resource IDs where temporal order matters

### Convenience Macros

```rust
use short_id::{id, ordered_id};

let random = id!();          // Same as short_id()
let ordered = ordered_id!(); // Same as short_id_ordered()
```

### Typed Wrapper

```rust
use short_id::ShortId;

let id = ShortId::random();   // Returns ShortId newtype
let id = ShortId::ordered();  // Time-ordered variant

// Convert to/from String
let s: String = id.into();
let id: ShortId = s.into();
```

## Advanced: Custom Length IDs

For advanced use cases, you can control the ID length by specifying the number of random bytes:

```rust
use short_id::{short_id_with_bytes, short_id_ordered_with_bytes};

// Generate a shorter 8-character ID (6 bytes)
let short = short_id_with_bytes(6);
assert_eq!(short.len(), 8);

// Generate a longer 22-character ID (16 bytes)
let long = short_id_with_bytes(16);
assert_eq!(long.len(), 22);

// Time-ordered IDs also support custom lengths
let ordered = short_id_ordered_with_bytes(12);
```

**When to use custom lengths:**

- **Fewer bytes (e.g., 4-6)**: Use for low-volume applications where you need very short IDs and collision risk is acceptable. Keep in mind that 6 bytes provides only ~48 bits of entropy (~1 in 10^14 collision probability).

- **Default (10 bytes)**: Recommended for most applications. Provides ~80 bits of entropy with 14-character IDs. The `short_id()` and `short_id_ordered()` functions use this (~1 in 10^24 collision probability).

- **More bytes (e.g., 16-32)**: Use for high-volume applications or when you need extra safety margin. 16 bytes provides ~128 bits of entropy.

**Important:** Using fewer bytes significantly increases collision probability. For most users, the default `short_id()` and `short_id_ordered()` functions are recommended.

## API Reference

**Functions:**
- `short_id() -> String` - Generate a random 14-character ID (recommended)
- `short_id_ordered() -> String` - Generate a time-ordered 14-character ID (requires `std`)
- `short_id_with_bytes(num_bytes: usize) -> String` - Advanced: custom length random ID
- `short_id_ordered_with_bytes(num_bytes: usize) -> String` - Advanced: custom length time-ordered ID (requires `std`)

**Macros:**
- `id!()` - Shorthand for `short_id()`
- `ordered_id!()` - Shorthand for `short_id_ordered()`

**Type:**
- `ShortId` - Newtype wrapper with methods:
  - `ShortId::random() -> Self`
  - `ShortId::ordered() -> Self` (requires `std`)
  - `as_str(&self) -> &str`
  - `into_string(self) -> String`
  - Implements: `Display`, `AsRef<str>`, `From<String>`, `From<ShortId> for String`

Default IDs are:
- Exactly 14 characters
- URL-safe: `A-Z`, `a-z`, `0-9`, `-`, `_`
- Cryptographically secure (using `OsRng`)
- Base64url encoded (no padding)
- ~80 bits of entropy (10 random bytes)

## `no_std` Support

This crate works in `no_std` environments with `alloc`:

```toml
[dependencies]
short-id = { version = "0.3", default-features = false }
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
