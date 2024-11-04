pub fn slice<T: Clone>(array: &[T], start: Option<isize>, end: Option<isize>) -> Vec<T> {
    let len = array.len() as isize;

    let start_index = match start {
        Some(s) if s < 0 => (len + s).max(0), // Handle negative start
        Some(s) => s.min(len).max(0),         // Handle positive start
        None => 0,
    };

    let end_index = match end {
        Some(e) if e < 0 => (len + e).max(0), // Handle negative end
        Some(e) => e.min(len),                // Handle positive end
        None => len,                          // No end specified means to the end of the array
    };

    // Slice the array
    array[start_index as usize..end_index as usize].to_vec()
}

#[test]
fn slice_array_test() {
    let animals = vec!["ant", "bison", "camel", "duck", "elephant"];
    assert_eq!(
        slice(&animals, Some(2), None),
        ["camel", "duck", "elephant"]
    );
    assert_eq!(slice(&animals, Some(2), Some(4)), ["camel", "duck"]);
}
