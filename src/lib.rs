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
//! - **Length**: Always exactly 14 characters (default)
//! - **URL-safe**: Only `A-Z`, `a-z`, `0-9`, `-`, `_` (no special characters)
//! - **Cryptographically secure**: Uses `OsRng` for random bytes
//! - **No configuration needed**: Just call the function
//!
//! # Advanced: Custom Length IDs
//!
//! For advanced use cases, you can control the ID length by specifying the number of random bytes:
//!
//! ```
//! use short_id::{short_id_with_bytes, short_id_ordered_with_bytes};
//!
//! // Generate a shorter 8-character ID (6 bytes)
//! let short = short_id_with_bytes(6).unwrap();
//! assert_eq!(short.len(), 8);
//!
//! // Generate a longer 22-character ID (16 bytes)
//! let long = short_id_with_bytes(16).unwrap();
//! assert_eq!(long.len(), 22);
//!
//! // Time-ordered IDs also support custom lengths
//! let ordered = short_id_ordered_with_bytes(12).unwrap();
//! ```
//!
//! **When to use custom lengths:**
//!
//! - **Fewer bytes (e.g., 4-6)**: Use for low-volume applications where you need very short IDs
//!   and collision risk is acceptable. Keep in mind that 6 bytes provides only ~48 bits of entropy.
//!
//! - **Default (10 bytes)**: Recommended for most applications. Provides ~80 bits of entropy
//!   with 14-character IDs. The [`short_id()`] and [`short_id_ordered()`] functions use this.
//!
//! - **More bytes (e.g., 16-32)**: Use for high-volume applications or when you need extra
//!   safety margin. 16 bytes provides ~128 bits of entropy.
//!
//! **Important:** Using fewer bytes significantly increases collision probability. For most users,
//! the default [`short_id()`] and [`short_id_ordered()`] functions are recommended.
//!
//! # Features
//!
//! - **`std`** (enabled by default): Enables [`short_id_ordered()`] and [`short_id_ordered_with_bytes()`]
//!   which need `std::time::SystemTime`
//!
//! For `no_std` environments with `alloc`:
//!
//! ```toml
//! [dependencies]
//! short-id = { version = "0.4", default-features = false }
//! ```
//!
//! In `no_std` mode, only [`short_id()`] and [`short_id_with_bytes()`] are available.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(not(feature = "std"))]
use alloc::vec;

#[cfg(feature = "std")]
use std::vec;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore};
use thiserror::Error;

/// Maximum number of random bytes allowed for custom-length ID generation.
///
/// This limit prevents excessive memory allocation and ensures reasonable ID sizes.
const MAX_BYTES: usize = 32;

/// Errors returned by the custom-length ID generation functions.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ShortIdError {
    /// `num_bytes` was 0.
    #[error("num_bytes must be greater than 0")]
    ZeroBytes,
    /// `num_bytes` exceeded [`MAX_BYTES`].
    #[error("num_bytes must not exceed {max} (got {requested})")]
    TooManyBytes { requested: usize, max: usize },
    /// `num_bytes` was less than 8, which is required for the timestamp prefix in ordered IDs.
    #[error("num_bytes must be at least 8 for ordered IDs (got {requested})")]
    TooFewBytesForOrdered { requested: usize },
    /// The string is not a valid `ShortId` (wrong length or characters outside the URL-safe
    /// base64 alphabet).
    #[error("string is not a valid ShortId (wrong length or invalid characters)")]
    InvalidString,
}

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

/// Internal helper: generates a random ID with the specified number of bytes.
///
/// Callers must validate `num_bytes` before calling this function.
fn generate_random_id(num_bytes: usize) -> String {
    let mut bytes = vec![0u8; num_bytes];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
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
    generate_random_id(10)
}

/// Internal helper: generates a time-ordered ID with the specified number of bytes.
///
/// Uses 8 bytes for timestamp and fills the remaining bytes with random data.
/// Callers must validate `num_bytes` before calling this function.
#[cfg(feature = "std")]
fn generate_ordered_id(num_bytes: usize) -> String {
    // as_micros() returns u128; cast to u64 is safe — u64 microseconds
    // overflow in ~584,000 years (year ~586,912 AD).
    let timestamp_us = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_micros() as u64;

    let mut bytes = vec![0u8; num_bytes];
    bytes[0..8].copy_from_slice(&timestamp_us.to_be_bytes());
    OsRng.fill_bytes(&mut bytes[8..]);

    URL_SAFE_NO_PAD.encode(&bytes)
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
    generate_ordered_id(10)
}

/// **Advanced:** Generates a random, URL-safe short ID with a custom number of bytes.
///
/// This is an advanced API that allows you to control the ID length by specifying
/// the number of random bytes to use. The ID is encoded using URL-safe base64 without
/// padding, so the resulting string length will be approximately `(num_bytes * 4) / 3`.
///
/// **For most users, [`short_id()`] is the recommended API.**
///
/// # Parameters
///
/// - `num_bytes`: Number of random bytes to generate (1 to 32 inclusive)
///
/// # Errors
///
/// Returns [`ShortIdError::ZeroBytes`] if `num_bytes` is 0, or
/// [`ShortIdError::TooManyBytes`] if `num_bytes` exceeds 32.
///
/// # Security Note
///
/// **Using fewer bytes reduces entropy and increases collision probability.**
/// - 10 bytes (default): ~80 bits of entropy, collision probability ~1 in 10^24
/// - 6 bytes: ~48 bits of entropy, collision probability ~1 in 10^14
/// - 4 bytes: ~32 bits of entropy, collision probability ~1 in 4 billion
///
/// Choose an appropriate size based on your uniqueness requirements and expected scale.
///
/// # Examples
///
/// Generate a standard 14-character ID (equivalent to `short_id()`):
///
/// ```
/// use short_id::short_id_with_bytes;
///
/// let id = short_id_with_bytes(10).unwrap();
/// assert_eq!(id.len(), 14);
/// ```
///
/// Generate a shorter 8-character ID with less entropy:
///
/// ```
/// use short_id::short_id_with_bytes;
///
/// let short_id = short_id_with_bytes(6).unwrap();
/// assert_eq!(short_id.len(), 8);
/// // Suitable for small-scale applications with fewer expected IDs
/// ```
///
/// Generate a longer ID with more entropy:
///
/// ```
/// use short_id::short_id_with_bytes;
///
/// let long_id = short_id_with_bytes(16).unwrap();
/// assert_eq!(long_id.len(), 22);
/// // Extra safety margin for high-volume applications
/// ```
///
/// All IDs are URL-safe regardless of size:
///
/// ```
/// use short_id::short_id_with_bytes;
///
/// let id = short_id_with_bytes(6).unwrap();
/// let url = format!("https://example.com/resource/{}", id);
/// // No encoding needed - safe to use directly
/// ```
pub fn short_id_with_bytes(num_bytes: usize) -> Result<String, ShortIdError> {
    if num_bytes == 0 {
        return Err(ShortIdError::ZeroBytes);
    }
    if num_bytes > MAX_BYTES {
        return Err(ShortIdError::TooManyBytes { requested: num_bytes, max: MAX_BYTES });
    }
    Ok(generate_random_id(num_bytes))
}

/// **Advanced:** Generates a time-ordered, URL-safe short ID with a custom number of bytes.
///
/// This is an advanced API that allows you to control the ID length by specifying
/// the number of bytes to use. The first 8 bytes always contain a microsecond-precision
/// timestamp, and the remaining bytes are filled with cryptographically secure random data.
///
/// **For most users, [`short_id_ordered()`] is the recommended API.**
///
/// **This function requires the `std` feature** (enabled by default).
///
/// # Parameters
///
/// - `num_bytes`: Total number of bytes for the ID (8 to 32 inclusive, must be at least 8 for the timestamp)
///
/// # Errors
///
/// Returns [`ShortIdError::TooFewBytesForOrdered`] if `num_bytes` is less than 8, or
/// [`ShortIdError::TooManyBytes`] if `num_bytes` exceeds 32.
///
/// # Security Note
///
/// **Using fewer random bytes (beyond the 8-byte timestamp) reduces uniqueness within the same microsecond.**
/// - 10 bytes (default): 8 bytes timestamp + 2 bytes random (~16 bits randomness per microsecond)
/// - 8 bytes: timestamp only, no randomness (IDs in the same microsecond will collide!)
/// - 16 bytes: 8 bytes timestamp + 8 bytes random (~64 bits randomness per microsecond)
///
/// # Examples
///
/// Generate a standard time-ordered ID (equivalent to `short_id_ordered()`):
///
/// ```
/// use short_id::short_id_ordered_with_bytes;
///
/// let id = short_id_ordered_with_bytes(10).unwrap();
/// assert_eq!(id.len(), 14);
/// ```
///
/// IDs from different times contain different timestamps:
///
/// ```
/// use short_id::short_id_ordered_with_bytes;
///
/// let id1 = short_id_ordered_with_bytes(10).unwrap();
/// std::thread::sleep(std::time::Duration::from_millis(10));
/// let id2 = short_id_ordered_with_bytes(10).unwrap();
///
/// // IDs from different times are different
/// assert_ne!(id1, id2);
/// ```
///
/// Shorter time-ordered IDs with minimal randomness:
///
/// ```
/// use short_id::short_id_ordered_with_bytes;
///
/// let id = short_id_ordered_with_bytes(8).unwrap();
/// assert_eq!(id.len(), 11);
/// // Warning: No random component! Only suitable if you never generate
/// // multiple IDs within the same microsecond.
/// ```
///
/// Longer time-ordered IDs with extra randomness:
///
/// ```
/// use short_id::short_id_ordered_with_bytes;
///
/// let id = short_id_ordered_with_bytes(16).unwrap();
/// assert_eq!(id.len(), 22);
/// // 8 bytes random component provides excellent uniqueness
/// // even when generating millions of IDs per second
/// ```
#[cfg(feature = "std")]
pub fn short_id_ordered_with_bytes(num_bytes: usize) -> Result<String, ShortIdError> {
    if num_bytes < 8 {
        return Err(ShortIdError::TooFewBytesForOrdered { requested: num_bytes });
    }
    if num_bytes > MAX_BYTES {
        return Err(ShortIdError::TooManyBytes { requested: num_bytes, max: MAX_BYTES });
    }
    Ok(generate_ordered_id(num_bytes))
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
    fn test_short_id_ordered_url_safe() {
        for _ in 0..100 {
            let id = short_id_ordered();
            assert!(!id.contains('+'));
            assert!(!id.contains('/'));
            assert!(!id.contains('='));
        }
    }

    // Tests for short_id_with_bytes

    #[test]
    fn test_short_id_with_bytes_standard() {
        let id = short_id_with_bytes(10).unwrap();
        assert_eq!(id.len(), 14);
    }

    #[test]
    fn test_short_id_with_bytes_shorter() {
        let id = short_id_with_bytes(6).unwrap();
        assert_eq!(id.len(), 8);
    }

    #[test]
    fn test_short_id_with_bytes_longer() {
        let id = short_id_with_bytes(16).unwrap();
        assert_eq!(id.len(), 22);
    }

    #[test]
    fn test_short_id_with_bytes_url_safe() {
        for num_bytes in [6, 10, 16, 32] {
            let id = short_id_with_bytes(num_bytes).unwrap();
            assert!(!id.contains('+'));
            assert!(!id.contains('/'));
            assert!(!id.contains('='));
        }
    }

    #[test]
    fn test_short_id_with_bytes_unique() {
        for num_bytes in [6, 10, 16] {
            let id1 = short_id_with_bytes(num_bytes).unwrap();
            let id2 = short_id_with_bytes(num_bytes).unwrap();
            assert_ne!(id1, id2);
        }
    }

    #[test]
    fn test_short_id_with_bytes_zero_errors() {
        assert_eq!(short_id_with_bytes(0), Err(ShortIdError::ZeroBytes));
    }

    #[test]
    fn test_short_id_with_bytes_too_large_errors() {
        assert_eq!(
            short_id_with_bytes(33),
            Err(ShortIdError::TooManyBytes { requested: 33, max: MAX_BYTES })
        );
    }

    // Tests for short_id_ordered_with_bytes

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_with_bytes_standard() {
        let id = short_id_ordered_with_bytes(10).unwrap();
        assert_eq!(id.len(), 14);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_with_bytes_minimal() {
        let id = short_id_ordered_with_bytes(8).unwrap();
        assert_eq!(id.len(), 11);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_with_bytes_longer() {
        let id = short_id_ordered_with_bytes(16).unwrap();
        assert_eq!(id.len(), 22);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_with_bytes_url_safe() {
        for num_bytes in [8, 10, 16, 32] {
            let id = short_id_ordered_with_bytes(num_bytes).unwrap();
            assert!(!id.contains('+'));
            assert!(!id.contains('/'));
            assert!(!id.contains('='));
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_with_bytes_unique() {
        for num_bytes in [10, 16] {
            let id1 = short_id_ordered_with_bytes(num_bytes).unwrap();
            let id2 = short_id_ordered_with_bytes(num_bytes).unwrap();
            assert_ne!(id1, id2);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_with_bytes_too_small_errors() {
        assert_eq!(
            short_id_ordered_with_bytes(7),
            Err(ShortIdError::TooFewBytesForOrdered { requested: 7 })
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_short_id_ordered_with_bytes_too_large_errors() {
        assert_eq!(
            short_id_ordered_with_bytes(33),
            Err(ShortIdError::TooManyBytes { requested: 33, max: MAX_BYTES })
        );
    }
}

/// A newtype wrapper around a short ID string.
///
/// Provides a typed interface for working with short IDs, with methods for
/// generation and conversion. The inner string is always a valid 14-character
/// URL-safe base64 identifier (characters `A-Z`, `a-z`, `0-9`, `-`, `_`).
///
/// # Ordering
///
/// `ShortId` implements [`Ord`] and [`PartialOrd`] via lexicographic string comparison.
/// This ordering is meaningful only for IDs created with [`ShortId::ordered()`], where
/// the timestamp prefix ensures creation-time order. For random IDs from
/// [`ShortId::random()`], the ordering is arbitrary.
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
/// let s: String = id.into();
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
    /// IDs generated with this method sort in creation-time order when compared with [`Ord`].
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

impl From<ShortId> for String {
    fn from(id: ShortId) -> Self {
        id.0
    }
}

/// Attempts to wrap a `String` as a [`ShortId`].
///
/// Validates that the string is exactly 14 characters long and contains only
/// URL-safe base64 characters (`A-Z`, `a-z`, `0-9`, `-`, `_`).
///
/// # Errors
///
/// Returns [`ShortIdError::InvalidString`] if validation fails.
///
/// # Examples
///
/// ```
/// use short_id::{ShortId, ShortIdError};
///
/// let id = ShortId::random();
/// let s: String = id.clone().into();
///
/// // A valid 14-char URL-safe string round-trips correctly
/// let recovered: ShortId = s.try_into().unwrap();
/// assert_eq!(id, recovered);
///
/// // An invalid string is rejected
/// let result = ShortId::try_from("not-valid!!".to_string());
/// assert_eq!(result, Err(ShortIdError::InvalidString));
/// ```
impl TryFrom<String> for ShortId {
    type Error = ShortIdError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() != 14 || !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err(ShortIdError::InvalidString);
        }
        Ok(ShortId(s))
    }
}

#[cfg(test)]
mod shortid_tests {
    use super::*;

    #[test]
    fn test_shortid_random_length() {
        assert_eq!(ShortId::random().as_str().len(), 14);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_shortid_ordered_length() {
        assert_eq!(ShortId::ordered().as_str().len(), 14);
    }

    #[test]
    fn test_shortid_into_string() {
        let id = ShortId::random();
        let s: String = id.clone().into();
        assert_eq!(s.len(), 14);
        assert_eq!(s, id.as_str());
    }

    #[test]
    fn test_shortid_try_from_valid() {
        let id = ShortId::random();
        let s: String = id.clone().into();
        let recovered = ShortId::try_from(s).unwrap();
        assert_eq!(id, recovered);
    }

    #[test]
    fn test_shortid_try_from_wrong_length() {
        assert_eq!(
            ShortId::try_from("short".to_string()),
            Err(ShortIdError::InvalidString)
        );
        assert_eq!(
            ShortId::try_from("this_is_way_too_long_to_be_valid".to_string()),
            Err(ShortIdError::InvalidString)
        );
    }

    #[test]
    fn test_shortid_try_from_invalid_chars() {
        // 14 chars but contains invalid characters
        assert_eq!(
            ShortId::try_from("invalid!chars!!".to_string()),
            Err(ShortIdError::InvalidString)
        );
    }
}
