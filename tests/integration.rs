use short_id::{id, short_id, ShortId};
#[cfg(feature = "std")]
use short_id::{ordered_id, short_id_ordered};

#[test]
fn test_short_id_returns_non_empty_string() {
    let id = short_id();
    assert!(
        !id.is_empty(),
        "short_id() should return a non-empty string"
    );
}

#[test]
fn test_short_id_returns_different_values() {
    let id1 = short_id();
    let id2 = short_id();
    assert_ne!(
        id1, id2,
        "short_id() should return different values for consecutive calls"
    );
}

#[test]
fn test_many_short_ids_are_unique() {
    // Generate 100 IDs in rapid succession
    let ids: Vec<String> = (0..100).map(|_| short_id()).collect();

    // All should be unique
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            assert_ne!(
                ids[i], ids[j],
                "Found duplicate IDs: {} == {}",
                ids[i], ids[j]
            );
        }
    }
}

#[test]
fn test_short_id_returns_valid_base64_url_safe() {
    let id = short_id();
    assert!(
        !id.contains('='),
        "short_id() should not contain '=' (URL_SAFE_NO_PAD)"
    );
    assert!(!id.contains('+'), "short_id() should not contain '+'");
    assert!(!id.contains('/'), "short_id() should not contain '/'");
}

#[cfg(feature = "std")]
#[test]
fn test_short_id_ordered_returns_different_values() {
    let id1 = short_id_ordered();
    let id2 = short_id_ordered();
    assert_ne!(
        id1, id2,
        "short_id_ordered() should return different values for consecutive calls"
    );
}

#[cfg(feature = "std")]
#[test]
fn test_short_id_ordered_returns_valid_base64_url_safe() {
    let ordered_id = short_id_ordered();
    assert!(
        !ordered_id.contains('='),
        "short_id_ordered() should not contain '=' (URL_SAFE_NO_PAD)"
    );
    assert!(
        !ordered_id.contains('+'),
        "short_id_ordered() should not contain '+'"
    );
    assert!(
        !ordered_id.contains('/'),
        "short_id_ordered() should not contain '/'"
    );
}

// Tests for id!() macro
#[test]
fn test_id_macro_returns_non_empty() {
    let id = id!();
    assert!(!id.is_empty(), "id!() should return a non-empty string");
}

#[test]
fn test_id_macro_returns_different_values() {
    let id1 = id!();
    let id2 = id!();
    assert_ne!(id1, id2, "id!() should return different values");
}

// Tests for ordered_id!() macro
#[cfg(feature = "std")]
#[test]
fn test_ordered_id_macro_returns_non_empty() {
    let id = ordered_id!();
    assert!(
        !id.is_empty(),
        "ordered_id!() should return a non-empty string"
    );
}

// Tests for ShortId::random()
#[test]
fn test_short_id_random_returns_non_empty() {
    let id = ShortId::random();
    assert_eq!(id.as_str().len(), 14, "ShortId::random() should be 14 chars");
}

#[test]
fn test_short_id_random_returns_different_values() {
    let id1 = ShortId::random();
    let id2 = ShortId::random();
    assert_ne!(id1, id2, "ShortId::random() should return different values");
}

// Tests for ShortId::ordered()
#[cfg(feature = "std")]
#[test]
fn test_short_id_ordered_returns_non_empty() {
    let id = ShortId::ordered();
    assert_eq!(
        id.as_str().len(),
        14,
        "ShortId::ordered() should be 14 chars"
    );
}

// Tests for ShortId trait implementations
#[test]
fn test_short_id_display() {
    let id = ShortId::random();
    let displayed = format!("{}", id);
    assert_eq!(displayed, id.as_str(), "Display should match as_str()");
}

#[test]
fn test_short_id_as_ref() {
    let id = ShortId::random();
    let s: &str = id.as_ref();
    assert_eq!(s, id.as_str(), "AsRef<str> should match as_str()");
}

#[test]
fn test_short_id_from_string() {
    let s = String::from("test_id_123456");
    let id: ShortId = s.clone().into();
    assert_eq!(id.as_str(), "test_id_123456");
}

#[test]
fn test_short_id_into_string() {
    let id = ShortId::random();
    let s: String = id.clone().into();
    assert_eq!(s, id.as_str());
}
