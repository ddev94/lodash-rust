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

#[test]
fn remove_if_array_test() {
    assert_eq!(remove_if(&vec![1, 2, 3], |&n| n % 2 == 0), [1, 3]);
    assert_eq!(
        remove_if(&vec![1, 2, 3, 4, 5, 6], |&n| n == 6),
        [1, 2, 3, 4, 5]
    );
}
