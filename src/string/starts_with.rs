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