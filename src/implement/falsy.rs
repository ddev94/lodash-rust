#![allow(dead_code)]

pub trait Falsy {
    fn is_falsy(&self) -> bool;
}

impl Falsy for i32 {
    fn is_falsy(&self) -> bool {
        *self == 0
    }
}

impl Falsy for bool {
    fn is_falsy(&self) -> bool {
        !*self
    }
}

impl Falsy for &str {
    fn is_falsy(&self) -> bool {
        self.is_empty()
    }
}