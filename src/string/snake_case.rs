pub fn snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_non_alphanumeric = true; // Track if the last character was non-alphanumeric

    for c in s.chars() {
        if c.is_alphanumeric() {
            // Convert the character to lowercase
            let lower_c = c.to_lowercase().next().unwrap();
            // Only add an underscore if the last added was non-alphanumeric
            if prev_was_non_alphanumeric && !result.is_empty() {
                result.push('_');
            }
            result.push(lower_c);
            prev_was_non_alphanumeric = false; // Set to false since we just added a character
        } else {
            // Mark as non-alphanumeric
            prev_was_non_alphanumeric = true;
        }
    }

    // Trim leading and trailing underscores
    result.trim_matches('_').to_string()
}