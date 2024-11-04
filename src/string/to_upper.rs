pub fn to_upper(s: &str) -> String {
    s.to_uppercase() // Convert the string to uppercase
}

#[test]
fn to_upper_test() {
    assert_eq!(to_upper("--foo-bar--"), "--FOO-BAR--");
    assert_eq!(to_upper("fooBar"), "FOOBAR");
    assert_eq!(to_upper("__foo_bar__"), "__FOO_BAR__");
    assert_eq!(to_upper("hello world!"), "HELLO WORLD!");
    assert_eq!(to_upper("Rust is awesome!"), "RUST IS AWESOME!");
}
