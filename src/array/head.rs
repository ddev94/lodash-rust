pub fn head<T: Clone>(vec: &[T]) -> T {
    // This will panic if the vector is empty
    vec.first().expect("Slice is empty").clone()
}