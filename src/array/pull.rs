pub fn pull<T: PartialEq + Clone>(vec: &[T], items: &[T]) -> Vec<T> {
    vec.iter()
        .filter(|x| !items.contains(*x))
        .cloned()
        .collect()
}

#[test]
fn pull_array_test() {
    assert_eq!(pull(&vec![1, 2, 3], &vec![1, 4, 5]), [2, 3]);
    assert_eq!(pull(&vec![1, 2, 3], &vec![2, 3]), [1]);
}
