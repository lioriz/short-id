use short_id::short_id;
#[cfg(feature = "std")]
use short_id::short_id_ordered;

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
