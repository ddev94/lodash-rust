use std::hash::Hash;

pub fn xor<T: Eq + Hash + Clone>(arrays: &[&[T]]) -> Vec<T> {
    let mut unique_counts = std::collections::HashMap::new();

    // Count occurrences of each element across all input arrays
    for array in arrays {
        for item in *array {
            *unique_counts.entry(item.clone()).or_insert(0) += 1;
        }
    }

    // Collect elements that occur exactly once
    unique_counts
        .into_iter()
        .filter_map(|(item, count)| if count == 1 { Some(item) } else { None })
        .collect()
}