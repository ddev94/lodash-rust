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
