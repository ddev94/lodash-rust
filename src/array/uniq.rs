use std::collections::HashSet;
use std::hash::Hash;

pub fn uniq<T>(arr: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut unique_set = HashSet::new();
    let mut unique_vec = Vec::new();

    for item in arr {
        // Insert returns true if the item was not already present
        if unique_set.insert(item.clone()) {
            unique_vec.push(item.clone());
        }
    }

    unique_vec
}

#[test]
fn uniq_array_test() {
    let numbers = vec![1, 3, 3, 4, 4, 2, 5];
    assert_eq!(uniq(&numbers), [1, 3, 4, 2, 5]);
}
