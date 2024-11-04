pub fn lower_case(s: &str) -> String {
    // Convert the string to lowercase and collect characters
    let mut result = String::new();
    let mut prev_was_space = true; // Track if the previous character was a space

    for c in s.chars() {
        // Check if the character is alphanumeric
        if c.is_alphanumeric() {
            // Add the lowercase character to the result
            result.push(c.to_lowercase().next().unwrap());
            prev_was_space = false; // Set to false since we just added a character
        } else {
            // If the character is not alphanumeric and the last added wasn't a space, add a space
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true; // Update the state to indicate we added a space
            }
        }
    }

    // Trim leading and trailing spaces
    result.trim().to_string()
}

#[test]
fn lower_case_test() {
    assert_eq!(lower_case("--Foo-Bar--"), "foo bar");
    assert_eq!(lower_case("fooBar"), "foobar");
    assert_eq!(lower_case("__FOO_BAR__"), "foo bar");
}
