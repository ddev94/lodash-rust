pub fn drop<T>(arr: &[T], n: usize) -> Vec<T>
where
    T: Clone,
{
    // If n is 0, return all elements
    if n == 0 {
        return arr.to_vec();
    }
    
    // Return the elements starting from index n
    arr.iter().skip(n).cloned().collect()
}

pub fn drop_right<T>(arr: &[T], n: usize) -> Vec<T>
where
    T: Clone,
{
    // If n is 0, return all elements
    if n == 0 {
        return arr.to_vec();
    }

    // Calculate the number of elements to take from the beginning
    let drop_count = if n >= arr.len() { arr.len() } else { arr.len() - n };
    
    // Return the elements up to drop_count
    arr.iter().take(drop_count).cloned().collect()
}