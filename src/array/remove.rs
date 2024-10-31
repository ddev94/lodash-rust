pub fn remove_if<T, F>(array: &Vec<T>, predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let mut kept = Vec::new();
    // let mut removed = Vec::new();

    for item in array {
        if predicate(item) {
            // removed.push(item.clone());
        } else {
            kept.push(item.clone());
        }
    }

    kept
}