use crate::{MatrixNxM, VectorND};
use std::iter::IntoIterator;

impl<const N: usize, const M: usize> IntoIterator for MatrixNxM<N, M> {
    type Item = VectorND<M>;
    type IntoIter = std::array::IntoIter<VectorND<M>, N>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.columns.into_iter()
    }
}

impl<'a, const N: usize, const M: usize> IntoIterator for &'a MatrixNxM<N, M> {
    type Item = &'a VectorND<M>;
    type IntoIter = std::slice::Iter<'a, VectorND<M>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.columns.iter()
    }
}

impl<'a, const N: usize, const M: usize> IntoIterator for &'a mut MatrixNxM<N, M> {
    type Item = &'a mut VectorND<M>;
    type IntoIter = std::slice::IterMut<'a, VectorND<M>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.columns.iter_mut()
    }
}

impl<const N: usize, const M: usize> MatrixNxM<N, M> {
    #[inline]
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, VectorND<M>> {
        self.into_iter()
    }

    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, VectorND<M>> {
        self.into_iter()
    }
}