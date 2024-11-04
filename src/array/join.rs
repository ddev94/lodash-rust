pub fn join<T: ToString>(vec: &[T], separator: &str) -> String {
    vec.iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}

#[test]
fn join_array_test() {
    assert_eq!(join(&vec![1, 2, 3], "~"), "1~2~3");
    assert_eq!(join(&vec![1, 2, 3], "+"), "1+2+3");
    assert_eq!(join(&vec![1, 2, 3], ""), "123");
}
