pub fn fill<T: Clone, F: Into<T>>(arr: &[T], value: F) -> Vec<T> {
    let value = value.into(); // Convert the value into type T
    vec![value; arr.len()] // Create a new vector filled with the specified value
}

pub fn fill_range<T: Clone, F: Into<T>>(arr: &[T], value: F, start: usize, end: usize) -> Vec<T> {
    let len = arr.len();
    let end = end.min(len); // Ensure end is not out of bounds
    let start = start.min(end); // Ensure start is not greater than end

    // Create a new vector and fill it
    let mut new_arr = arr.to_vec(); // Clone the original array
    let value = value.into(); // Convert the value into type T
    for i in start..end {
        new_arr[i] = value.clone(); // Fill the specified range
    }

    new_arr
}

#[test]
fn fill_array_test() {
    assert_eq!(fill(&vec![1, 2, 3], 2), vec![2, 2, 2]);
    assert_eq!(fill(&vec![1, 2, 3], 0), vec![0, 0, 0]);
}
