pub fn repeat(s: &str, count: usize) -> String {
    // Use the repeat method on the string slice
    s.repeat(count)
}

#[test]
fn repeat_test() {
    assert_eq!(repeat("*", 3), "***");
    assert_eq!(repeat("abc", 2), "abcabc");
    assert_eq!(repeat("abc", 0), "");
}
