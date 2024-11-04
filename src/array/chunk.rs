#![allow(dead_code)]

pub fn chunk<T: Clone>(slice: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return Vec::new();
    }

    let chunks = (slice.len() as f64 / size as f64).ceil() as usize;
    let mut result = Vec::with_capacity(chunks);

    for chunk_index in 0..chunks {
        let start = chunk_index * size;
        let end = (start + size).min(slice.len());
        result.push(slice[start..end].to_vec());
    }

    result
}

#[test]
fn test_chunk() {
    // Test with size 2
    assert_eq!(
        chunk(&['a', 'b', 'c', 'd'], 2),
        vec![vec!['a', 'b'], vec!['c', 'd']]
    );

    // Test with size 3
    assert_eq!(
        chunk(&['a', 'b', 'c', 'd'], 3),
        vec![vec!['a', 'b', 'c'], vec!['d']]
    );

    // Additional test cases
    assert_eq!(
        chunk(&[1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );

    // Test empty slice
    assert_eq!(chunk::<i32>(&[], 2), Vec::<Vec<i32>>::new());

    // Test with size 0
    assert_eq!(chunk(&[1, 2, 3], 0), Vec::<Vec<i32>>::new());

    // Test with size 1
    assert_eq!(chunk(&[1, 2, 3], 1), vec![vec![1], vec![2], vec![3]]);

    // Test with size larger than array
    assert_eq!(chunk(&[1, 2, 3], 5), vec![vec![1, 2, 3]]);
}
