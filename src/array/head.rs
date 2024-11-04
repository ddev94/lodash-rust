pub fn head<T: Clone>(vec: &[T]) -> T {
    // This will panic if the vector is empty
    vec.first().expect("Slice is empty").clone()
}

#[test]
fn head_array_test() {
    assert_eq!(head(&vec![1, 2, 3]), 1);
    assert_eq!(head(&vec!["Hello", "Rust"]), "Hello");
    assert_eq!(head(&vec![vec![1], vec![2]]), vec![1]);
}
