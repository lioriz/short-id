//! A tiny crate for generating short, URL-safe, unique identifiers.
//!
//! Unlike full UUIDs (which are 36 characters and include hyphens), `short-id` gives you
//! compact 14-character strings that are easy to copy, paste, and use in URLs.
//!
//! # Goals
//!
//! 1. **Make it very easy to generate short random IDs** for things like request IDs,
//!    user-facing tokens, test data, and log correlation.
//!
//! 2. **Provide an optional "ordered" variant** where IDs include a timestamp prefix,
//!    so when you sort them as strings they roughly follow creation time.
//!
//! This crate is intentionally minimal - no configuration, no custom alphabets, no complex API.
//!
//! # Quick Start
//!
//! ```
//! use short_id::short_id;
//!
//! // Generate a random ID
//! let id = short_id();
//! println!("Request ID: {}", id);
//! // Example output: "X7K9mP2nQwE-Tg"
//! ```
//!
//! For time-ordered IDs:
//!
//! ```
//! use short_id::short_id_ordered;
//!
//! let id1 = short_id_ordered();
//! std::thread::sleep(std::time::Duration::from_secs(1));
//! let id2 = short_id_ordered();
//!
//! // IDs roughly sort by creation time
//! assert!(id1 < id2);
//! ```
//!
//! # Use Cases
//!
//! - Request IDs for logging and tracing
//! - User-facing tokens and session IDs
//! - Test data generation
//! - Short URLs and resource identifiers
//! - Any place you want something shorter and simpler than UUIDs
//!
//! # Characteristics
//!
//! - **Length**: Always exactly 14 characters
//! - **URL-safe**: Only `A-Z`, `a-z`, `0-9`, `-`, `_` (no special characters)
//! - **Cryptographically secure**: Uses `OsRng` for random bytes
//! - **No configuration needed**: Just call the function
//!
//! # Features
//!
//! - **`std`** (enabled by default): Enables [`short_id_ordered()`] which needs `std::time::SystemTime`
//!
//! For `no_std` environments with `alloc`:
//!
//! ```toml
//! [dependencies]
//! short-id = { version = "0.1", default-features = false }
//! ```
//!
//! In `no_std` mode, only [`short_id()`] is available.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore};

/// Generates a random, URL-safe short ID.
///
/// Creates a 14-character ID from 10 cryptographically secure random bytes,
/// encoded with base64url (no padding).
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
/// ```
///
/// Use for request IDs:
///
/// ```
/// use short_id::short_id;
///
/// fn handle_request() -> String {
///     let request_id = short_id();
///     println!("Processing request {}", request_id);
///     request_id
/// }
///
/// let id = handle_request();
/// assert_eq!(id.len(), 14);
/// ```
///
/// Generate multiple unique IDs:
///
/// ```
/// use short_id::short_id;
///
/// let ids: Vec<String> = (0..10).map(|_| short_id()).collect();
///
/// // All IDs are unique
/// for i in 0..ids.len() {
///     for j in i+1..ids.len() {
///         assert_ne!(ids[i], ids[j]);
///     }
/// }
/// ```
///
/// IDs are URL-safe:
///
/// ```
/// use short_id::short_id;
///
/// let id = short_id();
/// let url = format!("https://example.com/resource/{}", id);
/// // No encoding needed - safe to use directly
/// ```
pub fn short_id() -> String {
    let mut bytes = [0u8; 10];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Generates a time-ordered, URL-safe short ID.
///
/// Creates a 14-character ID that includes the current Unix timestamp, making IDs
/// roughly sortable by creation time. The ID consists of:
/// - First 4 bytes: Unix timestamp (seconds) as big-endian u32
/// - Next 6 bytes: Cryptographically secure random bytes
///
/// When you sort these IDs as strings, they will roughly follow creation time order.
///
/// **This function requires the `std` feature** (enabled by default).
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
/// ```
///
/// IDs sort by creation time:
///
/// ```
/// use short_id::short_id_ordered;
///
/// let id1 = short_id_ordered();
/// std::thread::sleep(std::time::Duration::from_secs(1));
/// let id2 = short_id_ordered();
///
/// // IDs from different times sort chronologically
/// assert!(id1 < id2);
/// ```
///
/// Even within the same second, IDs are unique:
///
/// ```
/// use short_id::short_id_ordered;
///
/// let ids: Vec<String> = (0..10).map(|_| short_id_ordered()).collect();
///
/// // All unique due to random component
/// for i in 0..ids.len() {
///     for j in i+1..ids.len() {
///         assert_ne!(ids[i], ids[j]);
///     }
/// }
/// ```
///
/// Use for log entries:
///
/// ```
/// use short_id::short_id_ordered;
///
/// struct LogEntry {
///     id: String,
///     message: String,
/// }
///
/// impl LogEntry {
///     fn new(message: String) -> Self {
///         LogEntry {
///             id: short_id_ordered(),
///             message,
///         }
///     }
/// }
///
/// let log = LogEntry::new("Started processing".to_string());
/// assert_eq!(log.id.len(), 14);
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

    #[test]
    fn test_many_unique_ids() {
        // Generate many IDs and ensure all are unique
        #[cfg(feature = "std")]
        {
            let ids: Vec<String> = (0..1000).map(|_| short_id()).collect();
            let unique_count = ids.iter().collect::<std::collections::HashSet<_>>().len();
            assert_eq!(unique_count, 1000);
        }

        #[cfg(not(feature = "std"))]
        {
            // In no_std, just verify a few IDs are unique
            let id1 = short_id();
            let id2 = short_id();
            let id3 = short_id();
            assert_ne!(id1, id2);
            assert_ne!(id2, id3);
            assert_ne!(id1, id3);
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
    fn test_short_id_ordered_sorts_by_time() {
        let id1 = short_id_ordered();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let id2 = short_id_ordered();

        // IDs from different times sort chronologically
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
