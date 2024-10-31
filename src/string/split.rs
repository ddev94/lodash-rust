pub fn split(s: &str, delimiter: &str, limit: Option<usize>) -> Vec<String> {
    // Split the string using the delimiter
    let parts: Vec<&str> = s.split(delimiter).collect();

    // If a limit is provided, only take that many parts
    match limit {
        Some(l) => parts.into_iter().take(l).map(String::from).collect(),
        None => parts.into_iter().map(String::from).collect(),
    }
}