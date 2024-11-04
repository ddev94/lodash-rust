#[derive(Debug, PartialEq, Clone)]
pub enum ConcatItem<T> {
    Single(T),
    Array(Vec<T>),
}

// Function to concat arrays and values with one level of flattening
pub fn concat<T: Clone>(array: &[T], items: Vec<ConcatItem<T>>) -> Vec<T> {
    let mut result = array.to_vec();

    for item in items {
        match item {
            ConcatItem::Single(value) => result.push(value),
            ConcatItem::Array(values) => result.extend(values),
        }
    }

    result
}

#[test]
fn test_concat() {
    let array = vec![1];
    let other = concat(
        &array,
        vec![
            ConcatItem::Single(2),
            ConcatItem::Array(vec![3]),
            ConcatItem::Array(vec![4]),
        ],
    );

    assert_eq!(other, vec![1, 2, 3, 4]);
    assert_eq!(array, vec![1]); // Original array unchanged
}

#[test]
fn test_concat_empty() {
    let array: Vec<i32> = vec![];
    let other = concat(
        &array,
        vec![ConcatItem::Single(1), ConcatItem::Array(vec![2, 3])],
    );

    assert_eq!(other, vec![1, 2, 3]);
}

#[test]
fn test_concat_strings() {
    let array = vec!["hello".to_string()];
    let other = concat(
        &array,
        vec![
            ConcatItem::Single("world".to_string()),
            ConcatItem::Array(vec!["!".to_string()]),
        ],
    );

    assert_eq!(other, vec!["hello", "world", "!"]);
}

// Test with nested Vec<Vec<T>>
#[test]
fn test_concat_nested() {
    let array = vec![vec![1]];
    let other = concat(
        &array,
        vec![
            ConcatItem::Single(vec![2]),
            ConcatItem::Array(vec![vec![3]]),
        ],
    );

    assert_eq!(other, vec![vec![1], vec![2], vec![3]]);
}
