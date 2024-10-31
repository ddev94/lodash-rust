#![allow(dead_code)]

pub fn chunk<T: Clone>(arr: &Vec<T>, mut chunk_size: usize) -> Vec<Vec<T>> {
    let mut chunks = Vec::new();
    let mut i = 0;

    if chunk_size <= 0 {
        chunk_size = 1
    }

    while i < arr.len() {
        let end = (i + chunk_size).min(arr.len());
        chunks.push(arr[i..end].to_vec());
        i += chunk_size;
    }

    chunks
}