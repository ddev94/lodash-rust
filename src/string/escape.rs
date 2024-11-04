pub fn escape(s: &str) -> String {
    let mut escaped_string = String::new();

    for c in s.chars() {
        match c {
            '&' => escaped_string.push_str("&amp;"),
            '<' => escaped_string.push_str("&lt;"),
            '>' => escaped_string.push_str("&gt;"),
            '"' => escaped_string.push_str("&quot;"),
            '\'' => escaped_string.push_str("&apos;"),
            _ => escaped_string.push(c), // Push the character itself if no match
        }
    }

    escaped_string
}

#[test]
fn escape_with_test() {
    assert_eq!(
        escape("fred, barney, & pebbles"),
        "fred, barney, &amp; pebbles"
    );
    assert_eq!(
        escape("fred, barney, <script>aaa</script> pebbles"),
        "fred, barney, &lt;script&gt;aaa&lt;/script&gt; pebbles"
    );
}
