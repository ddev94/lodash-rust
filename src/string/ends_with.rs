pub fn ends_with(s: &str, suffix: &str, position: Option<usize>) -> bool {
    // If position is specified, adjust the string accordingly
    let end = match position {
        Some(pos) if pos <= s.len() => pos,
        _ => s.len(), // Use the full length if no position is specified or if it's out of bounds
    };

    // Get the slice of the string up to the specified position
    let substring = &s[..end];

    // Check if the substring ends with the given suffix
    substring.ends_with(suffix)
}

#[test]
fn ends_with_test() {
    assert_eq!(ends_with("abc", "c", None), true);
    assert_eq!(ends_with("abc", "b", Some(2)), true);
    assert_eq!(ends_with("abc", "a", None), false);
}
