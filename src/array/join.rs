pub fn join<T: ToString>(vec: &[T], separator: &str) -> String {
    vec.iter()
       .map(|item| item.to_string())
       .collect::<Vec<String>>()
       .join(separator)
}