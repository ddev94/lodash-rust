pub fn reverse<T>(array: &Vec<T>) -> Vec<T>
where
    T: Clone,
{
    let mut reversed = Vec::with_capacity(array.len());

    // Iterate from the end to the beginning and push elements to the new vector
    for item in array.iter().rev() {
        reversed.push(item.clone());
    }

    reversed
}

#[test]
fn reverse_array_test() {
    assert_eq!(reverse(&vec![1, 2, 3]), [3, 2, 1]);
}
