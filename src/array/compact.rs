#![allow(dead_code)]

use crate::implement::Falsey;

pub fn compact<T: Falsey + Clone>(slice: &[T]) -> Vec<T> {
    slice.iter().filter(|x| !x.is_falsey()).cloned().collect()
}

#[test]
fn test_compact_numbers() {
    let input = vec![0, 1, 2, 0, 3];
    assert_eq!(compact(&input), vec![1, 2, 3]);
}

#[test]
fn test_compact_bool() {
    let input = vec![true, false, true];
    assert_eq!(compact(&input), vec![true, true]);
}

#[test]
fn test_compact_mixed() {
    // To test mixed types, we need to convert everything to the same type
    // Here we'll use strings to demonstrate the concept
    let input = vec![
        "".to_string(),
        "hello".to_string(),
        "".to_string(),
        "world".to_string(),
    ];
    assert_eq!(
        compact(&input),
        vec!["hello".to_string(), "world".to_string()]
    );
}
