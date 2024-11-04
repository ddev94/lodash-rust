pub fn camel_case(s: &str) -> String {
    // Split by spaces, dashes, and underscores, while removing empty segments
    let segments: Vec<&str> = s
        .split(|c: char| c == ' ' || c == '-' || c == '_')
        .filter(|&segment| !segment.is_empty())
        .collect();

    // Transform to camel case
    let mut camel_cased = String::new();

    for (i, segment) in segments.iter().enumerate() {
        if i == 0 {
            // First segment should be all lowercase
            camel_cased.push_str(&segment.to_lowercase());
        } else {
            // Subsequent segments should be capitalized
            let capitalized = format!(
                "{}{}",
                segment.chars().next().unwrap().to_uppercase(),
                &segment[1..]
            );
            camel_cased.push_str(&capitalized);
        }
    }

    camel_cased
}

#[test]
fn camel_case_test() {
    assert_eq!(camel_case("Geeks for Geeks"), "geeksForGeeks");
    assert_eq!(camel_case("Geeks-for-Geeks"), "geeksForGeeks");
    assert_eq!(camel_case("Geeks_for_Geeks"), "geeksForGeeks");
}
