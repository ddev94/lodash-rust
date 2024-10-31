pub fn last<T: Clone>(slice: &[T]) -> T {
    // This will panic if the slice is empty
    slice.last().expect("Slice is empty").clone()
}