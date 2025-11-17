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
//! std::thread::sleep(std::time::Duration::from_millis(100));
//! let id2 = short_id_ordered();
//!
//! // IDs from different times are different
//! assert_ne!(id1, id2);
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

/// Convenience macro for generating a random short ID.
///
/// This macro simply calls [`short_id()`] and is provided for ergonomics.
///
/// # Examples
///
/// ```
/// use short_id::id;
///
/// let request_id = id!();
/// assert_eq!(request_id.len(), 14);
/// ```
#[macro_export]
macro_rules! id {
    () => {
        $crate::short_id()
    };
}

/// Convenience macro for generating a time-ordered short ID.
///
/// This macro simply calls [`short_id_ordered()`] and is provided for ergonomics.
/// Requires the `std` feature (enabled by default).
///
/// # Examples
///
/// ```
/// use short_id::ordered_id;
///
/// let log_id = ordered_id!();
/// assert_eq!(log_id.len(), 14);
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! ordered_id {
    () => {
        $crate::short_id_ordered()
    };
}

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
/// Creates a 14-character ID with microsecond-precision timestamp for excellent time
/// resolution when generating IDs in rapid succession. The ID consists of:
/// - First 8 bytes: Unix timestamp (microseconds since epoch) as big-endian u64
/// - Next 2 bytes: Cryptographically secure random bytes
///
/// With microsecond precision, IDs created within the same microsecond will differ
/// by their random component (65,536 possible values per microsecond).
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
/// IDs from different times differ:
///
/// ```
/// use short_id::short_id_ordered;
///
/// let id1 = short_id_ordered();
/// std::thread::sleep(std::time::Duration::from_millis(100));
/// let id2 = short_id_ordered();
///
/// // IDs generated at different times are different
/// assert_ne!(id1, id2);
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
    let timestamp_us = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_micros() as u64;

    let mut bytes = [0u8; 10];
    bytes[0..8].copy_from_slice(&timestamp_us.to_be_bytes());
    OsRng.fill_bytes(&mut bytes[8..10]);

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
    fn test_short_id_ordered_includes_timestamp() {
        // Generate IDs and verify they contain timestamp information
        // by checking they change over time
        let id1 = short_id_ordered();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let id2 = short_id_ordered();

        // IDs from different times should differ
        assert_ne!(id1, id2);
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

/// A newtype wrapper around a short ID string.
///
/// Provides a typed interface for working with short IDs, with methods for
/// generation and conversion. The inner string is always a valid 14-character
/// URL-safe identifier.
///
/// # Examples
///
/// ```
/// use short_id::ShortId;
///
/// // Generate a random ID
/// let id = ShortId::random();
/// assert_eq!(id.as_str().len(), 14);
///
/// // Convert to string
/// let s: String = id.into_string();
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ShortId(String);

impl ShortId {
    /// Creates a new random short ID.
    ///
    /// This is equivalent to calling [`short_id()`] but returns a typed [`ShortId`].
    ///
    /// # Examples
    ///
    /// ```
    /// use short_id::ShortId;
    ///
    /// let id = ShortId::random();
    /// assert_eq!(id.as_str().len(), 14);
    /// ```
    pub fn random() -> Self {
        ShortId(short_id())
    }

    /// Creates a new time-ordered short ID.
    ///
    /// This is equivalent to calling [`short_id_ordered()`] but returns a typed [`ShortId`].
    /// Requires the `std` feature (enabled by default).
    ///
    /// # Examples
    ///
    /// ```
    /// use short_id::ShortId;
    ///
    /// let id = ShortId::ordered();
    /// assert_eq!(id.as_str().len(), 14);
    /// ```
    #[cfg(feature = "std")]
    pub fn ordered() -> Self {
        ShortId(short_id_ordered())
    }

    /// Returns the ID as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use short_id::ShortId;
    ///
    /// let id = ShortId::random();
    /// let s: &str = id.as_str();
    /// assert_eq!(s.len(), 14);
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `ShortId` and returns the inner `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use short_id::ShortId;
    ///
    /// let id = ShortId::random();
    /// let s: String = id.into_string();
    /// assert_eq!(s.len(), 14);
    /// ```
    pub fn into_string(self) -> String {
        self.0
    }
}

impl core::fmt::Display for ShortId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ShortId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for ShortId {
    fn from(s: String) -> Self {
        ShortId(s)
    }
}

impl From<ShortId> for String {
    fn from(id: ShortId) -> Self {
        id.0
    }
}
