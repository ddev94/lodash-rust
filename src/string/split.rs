pub fn split(s: &str, delimiter: &str, limit: Option<usize>) -> Vec<String> {
    // Split the string using the delimiter
    let parts: Vec<&str> = s.split(delimiter).collect();

    // If a limit is provided, only take that many parts
    match limit {
        Some(l) => parts.into_iter().take(l).map(String::from).collect(),
        None => parts.into_iter().map(String::from).collect(),
    }
}

#[test]
fn split_test() {
    assert_eq!(split("Foo~Bar", "~", None), vec!["Foo", "Bar"]);
    assert_eq!(split("Foo~Bar~The", "~", Some(2)), vec!["Foo", "Bar"]);
}
