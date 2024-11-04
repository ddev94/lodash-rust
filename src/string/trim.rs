/// Trims whitespace from both ends of a string
pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

/// Trims specified characters from both ends of a string
pub fn trim_chars(s: &str, chars_to_trim: &str) -> String {
    if s.is_empty() || chars_to_trim.is_empty() {
        return s.to_string();
    }

    let chars_set: std::collections::HashSet<char> = chars_to_trim.chars().collect();

    // Find start index (first char that's not in chars_to_trim)
    let start = s
        .chars()
        .position(|c| !chars_set.contains(&c))
        .unwrap_or(s.len());

    // Find end index (last char that's not in chars_to_trim)
    let end = s
        .chars()
        .rev()
        .position(|c| !chars_set.contains(&c))
        .map(|pos| s.len() - pos)
        .unwrap_or(start);

    if start >= end {
        String::new()
    } else {
        s[start..end].to_string()
    }
}

/// Trims whitespace from the end of a string
pub fn trim_end(s: &str) -> String {
    s.trim_end().to_string()
}

/// Trims specified characters from the end of a string
pub fn trim_end_chars(s: &str, chars_to_trim: &str) -> String {
    if s.is_empty() || chars_to_trim.is_empty() {
        return s.to_string();
    }

    let chars_set: std::collections::HashSet<char> = chars_to_trim.chars().collect();

    // Find last index where character is not in chars_to_trim
    let end = s
        .chars()
        .rev()
        .position(|c| !chars_set.contains(&c))
        .map(|pos| s.len() - pos)
        .unwrap_or(0);

    if end == 0 {
        String::new()
    } else {
        s[..end].to_string()
    }
}

/// Trims whitespace from the start of a string
pub fn trim_start(s: &str) -> String {
    s.trim_start().to_string()
}

/// Trims specified characters from the start of a string
pub fn trim_start_chars(s: &str, chars_to_trim: &str) -> String {
    if s.is_empty() || chars_to_trim.is_empty() {
        return s.to_string();
    }

    let chars_set: std::collections::HashSet<char> = chars_to_trim.chars().collect();

    // Find first index where character is not in chars_to_trim
    let start = s
        .chars()
        .position(|c| !chars_set.contains(&c))
        .unwrap_or(s.len());

    if start >= s.len() {
        String::new()
    } else {
        s[start..].to_string()
    }
}

#[test]
fn test_trim() {
    assert_eq!(trim("  abc  "), "abc");
    assert_eq!(trim("\t\nabc\r\n"), "abc");
    assert_eq!(trim("abc"), "abc");
    assert_eq!(trim(""), "");
    assert_eq!(trim("   "), "");
}

#[test]
fn test_trim_chars() {
    assert_eq!(trim_chars("-_-abc-_-", "_-"), "abc");
    assert_eq!(trim_chars("***abc***", "*"), "abc");
    assert_eq!(trim_chars("abc", "_-"), "abc");
    assert_eq!(trim_chars("", "_-"), "");
    assert_eq!(trim_chars("---", "-"), "");
    assert_eq!(trim_chars("-_-abc-_-def-_-", "_-"), "abc-_-def");
    assert_eq!(trim_chars("__abc__def__", "_"), "abc__def");
    assert_eq!(trim_chars("-_-", "_-"), "");
    assert_eq!(trim_chars("abc-_-def", "_-"), "abc-_-def");
}

#[test]
fn test_trim_end() {
    assert_eq!(trim_end("  abc  "), "  abc");
    assert_eq!(trim_end("\t\nabc\r\n"), "\t\nabc");
    assert_eq!(trim_end("abc"), "abc");
    assert_eq!(trim_end(""), "");
    assert_eq!(trim_end("   "), "");
    assert_eq!(trim_end("abc   \n\t"), "abc");
}

#[test]
fn test_trim_end_chars() {
    assert_eq!(trim_end_chars("-_-abc-_-", "_-"), "-_-abc");
    assert_eq!(trim_end_chars("***abc***", "*"), "***abc");
    assert_eq!(trim_end_chars("abc", "_-"), "abc");
    assert_eq!(trim_end_chars("", "_-"), "");
    assert_eq!(trim_end_chars("---", "-"), "");
    assert_eq!(trim_end_chars("-_-abc-_-def-_-", "_-"), "-_-abc-_-def");
    assert_eq!(trim_end_chars("__abc__def__", "_"), "__abc__def");
    assert_eq!(trim_end_chars("-_-", "_-"), "");
    assert_eq!(trim_end_chars("abc-_-def", "_-"), "abc-_-def");
}

#[test]
fn test_edge_cases() {
    // Mixed characters
    assert_eq!(trim_end_chars("---***abc***---", "-*"), "---***abc");

    // Repeated characters
    assert_eq!(trim_end_chars("aaaaabcaaaa", "a"), "aaaaabc");

    // Single character
    assert_eq!(trim_end_chars("a", "a"), "");

    // All characters to be trimmed
    assert_eq!(trim_end_chars("abcdef", "abcdef"), "");

    // No characters to be trimmed
    assert_eq!(trim_end_chars("abc", "xyz"), "abc");

    // Mixed whitespace and custom characters
    assert_eq!(trim_end_chars("  abc  ***", "*"), "  abc  ");
}

#[test]
fn test_trim_start() {
    assert_eq!(trim_start("  abc  "), "abc  ");
    assert_eq!(trim_start("\t\nabc\r\n"), "abc\r\n");
    assert_eq!(trim_start("abc"), "abc");
    assert_eq!(trim_start(""), "");
    assert_eq!(trim_start("   "), "");
    assert_eq!(trim_start("   \n\tabc"), "abc");
}

#[test]
fn test_trim_start_chars() {
    assert_eq!(trim_start_chars("-_-abc-_-", "_-"), "abc-_-");
    assert_eq!(trim_start_chars("***abc***", "*"), "abc***");
    assert_eq!(trim_start_chars("abc", "_-"), "abc");
    assert_eq!(trim_start_chars("", "_-"), "");
    assert_eq!(trim_start_chars("---", "-"), "");
    assert_eq!(trim_start_chars("-_-abc-_-def-_-", "_-"), "abc-_-def-_-");
    assert_eq!(trim_start_chars("__abc__def__", "_"), "abc__def__");
    assert_eq!(trim_start_chars("-_-", "_-"), "");
    assert_eq!(trim_start_chars("abc-_-def", "_-"), "abc-_-def");
}

#[test]
fn test_trim_start_edge_cases() {
    // Mixed characters
    assert_eq!(trim_start_chars("---***abc***---", "-*"), "abc***---");

    // Repeated characters
    assert_eq!(trim_start_chars("aaaaabcaaaa", "a"), "bcaaaa");

    // Single character
    assert_eq!(trim_start_chars("a", "a"), "");

    // All characters to be trimmed
    assert_eq!(trim_start_chars("abcdef", "abcdef"), "");

    // No characters to be trimmed
    assert_eq!(trim_start_chars("abc", "xyz"), "abc");

    // Mixed whitespace and custom characters
    assert_eq!(trim_start_chars("***  abc  ", "*"), "  abc  ");
}

#[test]
fn test_trim_start_complex_cases() {
    // Multiple character types
    assert_eq!(trim_start_chars("-_*-abc-_*", "_-*"), "abc-_*");

    // Alternating characters
    assert_eq!(trim_start_chars("-_-_-_abc", "_-"), "abc");

    // Single character in trim set
    assert_eq!(trim_start_chars("---abc---", "-"), "abc---");

    // Empty trim set
    assert_eq!(trim_start_chars("---abc---", ""), "---abc---");

    // Trim set larger than string
    assert_eq!(trim_start_chars("abc", "abcdef"), "");
}
