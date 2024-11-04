pub fn sort<T: Ord + Clone>(arr: &Vec<T>) -> Vec<T> {
    let mut sorted = arr.clone(); // Clone the original array to avoid mutability
    sorted.sort(); // Sort the cloned vector in place
    sorted // Return the sorted vector
}

#[test]
fn sort_array_test() {
    let numbers = vec![1, 3, 4, 2, 5];
    assert_eq!(sort(&numbers), [1, 2, 3, 4, 5]);
}
