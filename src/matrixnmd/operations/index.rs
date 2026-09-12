use crate::{MatrixNxM, VectorND};
use std::ops::{Index, IndexMut};

impl<const N: usize, const M: usize> Index<usize> for MatrixNxM<N, M> {
    type Output = VectorND<M>;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.columns[index]
    }
}

impl<const N: usize, const M: usize> IndexMut<usize> for MatrixNxM<N, M> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.columns[index]
    }
}