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
    let start = s.chars()
        .position(|c| !chars_set.contains(&c))
        .unwrap_or(s.len());

    // Find end index (last char that's not in chars_to_trim)
    let end = s.chars()
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
    let end = s.chars()
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
    let start = s.chars()
        .position(|c| !chars_set.contains(&c))
        .unwrap_or(s.len());

    if start >= s.len() {
        String::new()
    } else {
        s[start..].to_string()
    }
}