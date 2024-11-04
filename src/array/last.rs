pub fn last<T: Clone>(slice: &[T]) -> T {
    // This will panic if the slice is empty
    slice.last().expect("Slice is empty").clone()
}

#[test]
fn last_array_test() {
    assert_eq!(last(&vec![1, 2, 3]), 3);
}
