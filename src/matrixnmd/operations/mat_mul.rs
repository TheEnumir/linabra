use crate::{MatrixNxM};
use std::ops::{Mul, MulAssign};

//Образующие
impl<const N: usize, const M: usize, const K: usize> Mul<&MatrixNxM<M, K>> for &MatrixNxM<N, M> {
    type Output = MatrixNxM<N, K>;
    #[inline]
    fn mul(self, rhs: &MatrixNxM<M, K>) -> Self::Output {
        Self::Output::from_fn( |i| self[i] * rhs )
    }
}

//Образуемые
impl<const N: usize> MulAssign<&MatrixNxM<N, N>> for MatrixNxM<N, N> {
    #[inline]
    fn mul_assign(&mut self, rhs: &MatrixNxM<N, N>) {
        *self = self.mul(rhs)
    }
}

impl<const N: usize> MulAssign for MatrixNxM<N, N> {
    #[inline]
    fn mul_assign(&mut self, rhs: MatrixNxM<N, N>) {
        *self *= &rhs
    }
}

impl<const N: usize, const M: usize, const K: usize> Mul<MatrixNxM<M, K>> for &MatrixNxM<N, M> {
    type Output = MatrixNxM<N, K>;
    #[inline]
    fn mul(self, rhs: MatrixNxM<M, K>) -> Self::Output {
        self * &rhs
    }
}

impl<const N: usize, const M: usize, const K: usize> Mul<&MatrixNxM<M, K>> for MatrixNxM<N, M> {
    type Output = MatrixNxM<N, K>;
    #[inline]
    fn mul(self, rhs: &MatrixNxM<M, K>) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize, const M: usize, const K: usize> Mul<MatrixNxM<M, K>> for MatrixNxM<N, M> {
    type Output = MatrixNxM<N, K>;
    #[inline]
    fn mul(self, rhs: MatrixNxM<M, K>) -> Self::Output {
        &self * &rhs
    }
}