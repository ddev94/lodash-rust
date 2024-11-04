pub fn starts_with(s: &str, prefix: &str, position: Option<usize>) -> bool {
    // Determine the starting position
    let start = position.unwrap_or(0);
    
    // Check if the starting position is valid
    if start > s.len() {
        return false;
    }

    // Get the substring starting from the specified position
    let substring = &s[start..];

    // Check if the substring starts with the prefix
    substring.starts_with(prefix)
}

#[test]
    fn starts_with_test() {
        assert_eq!(starts_with("abc", "a", None), true);
        assert_eq!(starts_with("abc", "b", None), false);
        assert_eq!(starts_with("abc", "b", Some(1)), true);
        assert_eq!(starts_with("abc", "ab", Some(0)), true);
        assert_eq!(starts_with("abc", "abc", Some(0)), true);
        assert_eq!(starts_with("abc", "c", Some(2)), true);
        assert_eq!(starts_with("abc", "a", Some(1)), false);
        assert_eq!(starts_with("abc", "", Some(1)), true); // An empty string is always a prefix
    }