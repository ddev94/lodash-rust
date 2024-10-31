pub trait IsArray {
    fn is_array(&self) -> bool;
}

// Implement IsArray for Vec
impl<T> IsArray for Vec<T> {
    fn is_array(&self) -> bool {
        true
    }
}

// Implement IsArray for slices
impl<T> IsArray for &[T] {
    fn is_array(&self) -> bool {
        true
    }
}

// Provide a default implementation that returns false for all other types
impl IsArray for &str {
    fn is_array(&self) -> bool {
        false
    }
}

impl IsArray for () {
    fn is_array(&self) -> bool {
        false
    }
}

impl<F> IsArray for F 
where 
    F: Fn() 
{
    fn is_array(&self) -> bool {
        false
    }
}

pub fn is_array<T: IsArray>(input: &T) -> bool {
    input.is_array()
}
