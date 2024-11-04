use std::collections::HashMap;

pub fn create_html_entities_map() -> HashMap<String, String> {
    let mut entities = HashMap::new();
    entities.insert("&amp;".to_string(), "&".to_string());
    entities.insert("&lt;".to_string(), "<".to_string());
    entities.insert("&gt;".to_string(), ">".to_string());
    entities.insert("&quot;".to_string(), "\"".to_string());
    entities.insert("&#39;".to_string(), "'".to_string());
    entities.insert("&apos;".to_string(), "'".to_string());
    entities
}

pub fn unescape(input: &str) -> String {
    let entities = create_html_entities_map();
    let mut result = input.to_string();

    for (entity, character) in entities.iter() {
        result = result.replace(entity, character);
    }

    result
}

#[test]
fn test_unescape() {
    let test_cases = vec![
        // Basic case (original example)
        ("fred, barney, &amp; pebbles", "fred, barney, & pebbles"),
        // Multiple entities
        (
            "&lt;div&gt;Hello &amp; welcome&lt;/div&gt;",
            "<div>Hello & welcome</div>",
        ),
        // No entities
        ("Hello World", "Hello World"),
        // Quotes and apostrophes
        ("&quot;Hello&quot; &apos;World&apos;", "\"Hello\" 'World'"),
        // Empty string
        ("", ""),
    ];

    for (input, expected) in test_cases {
        assert_eq!(unescape(input), expected, "Failed on input: {}", input);
    }
}
