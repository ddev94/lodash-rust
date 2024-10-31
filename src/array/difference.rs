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