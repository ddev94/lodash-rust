pub fn pull<T: PartialEq + Clone>(vec: &[T], items: &[T]) -> Vec<T> {
    vec.iter()
        .filter(|x| !items.contains(*x))
        .cloned()
        .collect()
}