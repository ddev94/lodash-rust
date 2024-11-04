// Define a trait for determining if a value should be considered "falsey"
pub trait Falsey {
    fn is_falsey(&self) -> bool;
}

// Implement Falsey for common types
impl Falsey for i32 {
    fn is_falsey(&self) -> bool {
        *self == 0
    }
}

impl Falsey for f64 {
    fn is_falsey(&self) -> bool {
        *self == 0.0 || self.is_nan()
    }
}

impl Falsey for bool {
    fn is_falsey(&self) -> bool {
        !*self
    }
}

impl Falsey for String {
    fn is_falsey(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Falsey for Option<T> {
    fn is_falsey(&self) -> bool {
        self.is_none()
    }
}
