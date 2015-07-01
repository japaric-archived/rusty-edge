use std::ops::Index;

impl<T> Index<usize> for ::Row<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.0[i]
    }
}
