pub fn find<T, F>(array: &[T], predicate: F) -> T
where
    T: Clone + Default,
    F: Fn(&T) -> bool,
{
    array.iter().find(|&item| predicate(item)).cloned().unwrap_or_default()
}
