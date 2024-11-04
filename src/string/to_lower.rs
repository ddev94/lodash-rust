pub fn to_lower(s: &str) -> String {
    s.to_lowercase() // Convert the string to lowercase
}

#[test]
fn to_lower_test() {
    assert_eq!(to_lower("--Foo-Bar--"), "--foo-bar--");
    assert_eq!(to_lower("fooBar"), "foobar");
    assert_eq!(to_lower("__FOO_BAR__"), "__foo_bar__");
    assert_eq!(to_lower("HELLO WORLD!"), "hello world!");
    assert_eq!(to_lower("Rust Is Awesome!"), "rust is awesome!");
}
