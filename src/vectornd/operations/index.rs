use crate::VectorND;
use std::ops::{Index, IndexMut};

impl<const N: usize> Index<usize> for VectorND<N> {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl<const N: usize> IndexMut<usize> for VectorND<N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arr[index]
    }
}