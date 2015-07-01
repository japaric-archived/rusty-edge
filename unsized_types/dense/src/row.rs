use std::mem;
use std::ops::Index;

impl<T> Index<usize> for ::Row<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        unsafe {
            &mem::transmute::<_, &[T]>(self)[i]
        }
    }
}
