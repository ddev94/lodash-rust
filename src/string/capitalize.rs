pub fn capitalize(s: &str) -> String {
    // Check if the string is empty
    if s.is_empty() {
        return String::new();
    }

    // Convert the first character to uppercase and the rest to lowercase
    let first_char = s.chars().next().unwrap().to_uppercase();
    let rest = &s[1..].to_lowercase();

    // Combine them into a new string
    format!("{}{}", first_char, rest)
}

#[test]
fn capitalize_test() {
    assert_eq!(capitalize("Geeks for Geeks"), "Geeks for geeks");
    assert_eq!(capitalize("GeeksforGeeks"), "Geeksforgeeks");
}
