use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn xor<T: Eq + Hash + Clone>(arrays: &[&[T]]) -> Vec<T> {
    let mut unique_counts = HashMap::new();

    // Remove duplicates within each array by converting to a HashSet
    for array in arrays {
        let unique_items: HashSet<_> = array.iter().cloned().collect();
        for item in unique_items {
            *unique_counts.entry(item).or_insert(0) += 1;
        }
    }

    // Collect elements that occur exactly once, preserving the order in arrays
    let mut result = Vec::new();
    for array in arrays {
        for item in *array {
            if let Some(1) = unique_counts.get(item) {
                if !result.contains(item) {
                    result.push(item.clone());
                }
            }
        }
    }

    result
}

#[test]
fn xor_array_test() {
    let arr1 = vec![3, 1];
    let arr2 = vec![1, 2];
    assert_eq!(xor(&[&arr1, &arr2]), [3, 2]);
    assert_eq!(xor(&[&arr2, &arr1]), [2, 3]);
}
