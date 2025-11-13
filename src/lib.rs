//! A tiny crate for generating short, URL-safe, random or time-ordered IDs.
//!
//! This crate provides two simple functions:
//! - [`short_id()`] generates a random 10-byte ID
//! - [`short_id_ordered()`] generates a time-ordered ID with a 4-byte timestamp prefix
//!
//! Both functions return base64url-encoded strings that are safe to use in URLs.
//!
//! # Examples
//!
//! ```
//! use short_id::{short_id, short_id_ordered};
//!
//! // Generate a random ID
//! let id = short_id();
//! println!("Random ID: {}", id);
//!
//! // Generate a time-ordered ID
//! let ordered_id = short_id_ordered();
//! println!("Time-ordered ID: {}", ordered_id);
//! ```

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore};

/// Generates a short, random, URL-safe ID.
///
/// This function generates 10 random bytes using a cryptographically secure random
/// number generator and encodes them using base64url encoding without padding.
///
/// The resulting string is 14 characters long and contains only URL-safe characters
/// (A-Z, a-z, 0-9, -, _).
///
/// # Examples
///
/// ```
/// use short_id::short_id;
///
/// let id = short_id();
/// assert_eq!(id.len(), 14);
/// ```
pub fn short_id() -> String {
    let mut bytes = [0u8; 10];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Generates a time-ordered, URL-safe ID.
///
/// This function generates an ID with a timestamp prefix:
/// - First 4 bytes: current Unix timestamp as a big-endian u32
/// - Next 6 bytes: random bytes from a cryptographically secure RNG
///
/// The 10 bytes are then encoded using base64url encoding without padding.
///
/// Note: While the timestamp prefix provides temporal information, base64url encoding
/// does not guarantee perfect lexicographic sorting (UTF-8 character order differs from
/// base64 value order). IDs generated seconds apart will typically differ in their
/// first characters, but lexicographic ordering is not guaranteed.
///
/// # Examples
///
/// ```
/// use short_id::short_id_ordered;
///
/// let id1 = short_id_ordered();
/// std::thread::sleep(std::time::Duration::from_secs(1));
/// let id2 = short_id_ordered();
///
/// // IDs will be different (timestamp + random)
/// assert_ne!(id1, id2);
/// ```
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
    fn test_short_id_ordered_length() {
        let id = short_id_ordered();
        assert_eq!(id.len(), 14);
    }

    #[test]
    fn test_short_id_ordered_unique() {
        let id1 = short_id_ordered();
        let id2 = short_id_ordered();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_short_id_ordered_chronological() {
        let id1 = short_id_ordered();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let id2 = short_id_ordered();

        assert!(id1 < id2, "id1: {}, id2: {}", id1, id2);
    }

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
