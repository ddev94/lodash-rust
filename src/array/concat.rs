#![allow(dead_code)]

pub fn concat<T: Clone>(vec1: &Vec<T>, vec2: &Vec<T>) -> Vec<T> {
    let mut result = vec1.clone();
    result.extend(vec2.iter().cloned());
    result
}
