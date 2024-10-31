pub fn sort<T: Ord + Clone>(arr: &Vec<T>) -> Vec<T> {
    let mut sorted = arr.clone(); // Clone the original array to avoid mutability
    sorted.sort(); // Sort the cloned vector in place
    sorted // Return the sorted vector
}