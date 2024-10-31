#![allow(dead_code)]

use crate::implement::Falsy;

pub fn compact<T: Falsy + Clone>(arr: &Vec<T>) -> Vec<T> {
    arr.iter()
        .filter(|item| !item.is_falsy())
        .cloned()
        .collect()
}