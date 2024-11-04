use std::collections::HashSet;

pub fn difference<T: Eq + std::hash::Hash>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Clone,
{
    let set: HashSet<_> = arr2.iter().collect();
    arr1.iter()
        .filter(|item| !set.contains(item))
        .cloned()
        .collect()
}

#[test]
fn difference_array_test() {
    let arr1 = vec![1, 2];
    let arr2 = vec![1, 2];
    assert_eq!(difference(&arr1, &arr2), []);

    let arr3 = vec![1, 2, 3];
    let arr4 = vec![2, 3];
    assert_eq!(difference(&arr3, &arr4), [1])
}
