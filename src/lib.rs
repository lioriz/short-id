//! A tiny crate for generating short, URL-safe, random or time-ordered IDs.
//!
//! # Overview
//!
//! `short-id` generates compact, URL-safe identifiers that are:
//! - **Shorter than UUIDs**: 14 characters vs 36 for UUIDs
//! - **URL-safe**: No special characters requiring encoding (`+`, `/`, `=`)
//! - **Cryptographically secure**: Uses `OsRng` for random bytes
//! - **Minimal**: Just two functions, no configuration needed
//! - **`no_std` compatible**: Works in embedded environments with `alloc`
//!
//! # Quick Start
//!
//! ```
//! use short_id::short_id;
//!
//! // Generate a random ID
//! let id = short_id();
//! println!("Generated ID: {}", id);
//! // Example output: "X7K9mP2nQwE-Tg"
//!
//! // IDs are always 14 characters long
//! assert_eq!(id.len(), 14);
//!
//! // IDs are URL-safe (no special characters)
//! assert!(!id.contains('+') && !id.contains('/') && !id.contains('='));
//! ```
//!
//! # Time-Ordered IDs
//!
//! For IDs that include temporal information (requires the `std` feature):
//!
//! ```
//! use short_id::short_id_ordered;
//!
//! let id1 = short_id_ordered();
//! std::thread::sleep(std::time::Duration::from_secs(1));
//! let id2 = short_id_ordered();
//!
//! // IDs generated at different times will differ
//! assert_ne!(id1, id2);
//! ```
//!
//! # Features
//!
//! - **`std`** (enabled by default): Enables [`short_id_ordered()`] which requires `std::time::SystemTime`
//!
//! To use in `no_std` environments with `alloc`:
//!
//! ```toml
//! [dependencies]
//! short-id = { version = "0.1", default-features = false }
//! ```
//!
//! In `no_std` mode, only [`short_id()`] is available.
//!
//! # Implementation Details
//!
//! - Random IDs use 10 cryptographically secure random bytes
//! - Time-ordered IDs use 4 bytes for Unix timestamp + 6 random bytes
//! - All IDs are encoded with base64url (URL_SAFE_NO_PAD)
//! - Result is always exactly 14 characters (no padding)

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore};

/// Generates a short, random, URL-safe ID.
///
/// This function generates 10 cryptographically secure random bytes and encodes them
/// using base64url encoding without padding, resulting in a 14-character string.
///
/// # Characteristics
///
/// - **Length**: Always 14 characters
/// - **Characters**: `A-Z`, `a-z`, `0-9`, `-`, `_` (URL-safe)
/// - **Uniqueness**: Cryptographically random, collision probability is negligible
/// - **Availability**: Works in both `std` and `no_std` (with `alloc`) environments
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use short_id::short_id;
///
/// let id = short_id();
/// assert_eq!(id.len(), 14);
/// println!("Generated ID: {}", id);
/// ```
///
/// Generate multiple unique IDs:
///
/// ```
/// use short_id::short_id;
///
/// let id1 = short_id();
/// let id2 = short_id();
/// let id3 = short_id();
///
/// // Each ID is unique
/// assert_ne!(id1, id2);
/// assert_ne!(id2, id3);
/// assert_ne!(id1, id3);
/// ```
///
/// IDs are URL-safe:
///
/// ```
/// use short_id::short_id;
///
/// let id = short_id();
///
/// // Safe to use in URLs without encoding
/// let url = format!("https://example.com/resource/{}", id);
/// assert!(url.chars().all(|c| c.is_ascii_alphanumeric() || ":/.-_".contains(c)));
/// ```
///
/// Use in function signatures:
///
/// ```
/// use short_id::short_id;
///
/// fn create_session() -> String {
///     short_id()
/// }
///
/// let session_id = create_session();
/// assert_eq!(session_id.len(), 14);
/// ```
pub fn short_id() -> String {
    let mut bytes = [0u8; 10];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Generates a time-ordered, URL-safe ID.
///
/// This function generates an ID prefixed with the current Unix timestamp, making IDs
/// contain temporal information. The ID consists of:
/// - First 4 bytes: Current Unix timestamp (seconds since epoch) as big-endian u32
/// - Next 6 bytes: Cryptographically secure random bytes
///
/// The result is base64url-encoded to produce a 14-character string.
///
/// # Availability
///
/// **This function requires the `std` feature** (enabled by default) as it depends on
/// `std::time::SystemTime`.
///
/// # Characteristics
///
/// - **Length**: Always 14 characters
/// - **Characters**: `A-Z`, `a-z`, `0-9`, `-`, `_` (URL-safe)
/// - **Temporal information**: First 4 bytes encode the timestamp
/// - **Uniqueness**: Timestamp + random bytes ensure uniqueness
///
/// # Ordering Caveat
///
/// While IDs include a timestamp prefix, **lexicographic ordering is not guaranteed**
/// due to base64url encoding characteristics (UTF-8 character order differs from base64
/// value order). IDs generated seconds apart will typically differ in their prefix, but
/// don't rely on string comparison for sorting by time.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use short_id::short_id_ordered;
///
/// let id = short_id_ordered();
/// assert_eq!(id.len(), 14);
/// println!("Time-ordered ID: {}", id);
/// ```
///
/// IDs generated at different times differ:
///
/// ```
/// use short_id::short_id_ordered;
///
/// let id1 = short_id_ordered();
/// std::thread::sleep(std::time::Duration::from_millis(100));
/// let id2 = short_id_ordered();
///
/// // IDs are different due to timestamp and random components
/// assert_ne!(id1, id2);
/// ```
///
/// Even within the same second, IDs are unique:
///
/// ```
/// use short_id::short_id_ordered;
///
/// let id1 = short_id_ordered();
/// let id2 = short_id_ordered();
/// let id3 = short_id_ordered();
///
/// // Random component ensures uniqueness within the same second
/// assert_ne!(id1, id2);
/// assert_ne!(id2, id3);
/// ```
///
/// Use for timestamped resources:
///
/// ```
/// use short_id::short_id_ordered;
///
/// struct Message {
///     id: String,
///     content: String,
/// }
///
/// impl Message {
///     fn new(content: String) -> Self {
///         Message {
///             id: short_id_ordered(),
///             content,
///         }
///     }
/// }
///
/// let msg = Message::new("Hello, world!".to_string());
/// assert_eq!(msg.id.len(), 14);
/// ```
#[cfg(feature = "std")]
pub fn short_id_ordered() -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_secs() as u32;

    let mut bytes = [0u8; 10];
    bytes[0..4].copy_from_slice(&timestamp.to_be_bytes());
    OsRng.fill_bytes(&mut bytes[4..10]);

    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_id_length() {
        let id = short_id();
        assert_eq!(id.len(), 14);
    }

    #[test]
    fn test_short_id_unique() {
        let id1 = short_id();
        let id2 = short_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_short_id_url_safe() {
        for _ in 0..100 {
            let id = short_id();
            assert!(!id.contains('+'));
            assert!(!id.contains('/'));
            assert!(!id.contains('='));
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_length() {
        let id = short_id_ordered();
        assert_eq!(id.len(), 14);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_unique() {
        let id1 = short_id_ordered();
        let id2 = short_id_ordered();
        assert_ne!(id1, id2);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_chronological() {
        let id1 = short_id_ordered();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let id2 = short_id_ordered();

        assert!(id1 < id2, "id1: {}, id2: {}", id1, id2);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_url_safe() {
        for _ in 0..100 {
            let id = short_id_ordered();
            assert!(!id.contains('+'));
            assert!(!id.contains('/'));
            assert!(!id.contains('='));
        }
    }
}
