pub fn find_index<T, F>(arr: &Vec<T>, f: F) -> i8
where
    F: Fn(&T) -> bool,
{
    let mut opt: Option<usize> = None;
    for (index, item) in arr.iter().enumerate() {
        if f(item) {
            opt = Some(index);
        }
    }

    match opt {
        Some(i) => {
            return i as i8;
        }
        None => {
            return -1;
        }
    }
}

#[test]
fn find_integer_index_test() {
    let arr = vec![1, 2, 3, 4];
    assert_eq!(find_index(&arr, |&x| x == 2), 1);
    assert_eq!(find_index(&arr, |&x| x == 5), -1);
}

#[test]
fn find_string_index_test() {
    let arr = vec!["Hello", "Lodash", "Rust"];
    assert_eq!(find_index(&arr, |&x| x == "Rust"), 2);
    assert_eq!(find_index(&arr, |&x| x == "Out"), -1);
}

#[test]
fn find_char_index_test() {
    let arr = vec!['r', 'u', 's', 't'];
    assert_eq!(find_index(&arr, |&x| x == 't'), 3);
    assert_eq!(find_index(&arr, |&x| x == 'z'), -1);
}
