use crate::{MatrixNxM, VectorND};
use std::ops::{Mul, MulAssign};

//Образующие
impl<const N: usize, const M: usize> Mul<&MatrixNxM<N, M>> for &VectorND<N> {
    type Output = VectorND<M>;
    #[inline]
    fn mul(self, rhs: &MatrixNxM<N, M>) -> Self::Output {
        (0..N).map(|i| rhs[i] * self[i]).sum()
    }
}

//Образованные
impl<const N: usize> MulAssign<&MatrixNxM<N, N>> for VectorND<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: &MatrixNxM<N, N>) {
        *self = (&*self) * rhs
    }
}

impl<const N: usize> MulAssign<MatrixNxM<N, N>> for VectorND<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: MatrixNxM<N, N>) {
        *self *= &rhs
    }
}

impl<const N: usize, const M: usize> Mul<MatrixNxM<N, M>> for VectorND<N> {
    type Output = VectorND<M>;
    #[inline]
    fn mul(self, rhs: MatrixNxM<N, M>) -> Self::Output {
        &self * &rhs
    }
}

impl<const N: usize, const M: usize> Mul<&MatrixNxM<N, M>> for VectorND<N> {
    type Output = VectorND<M>;
    #[inline]
    fn mul(self, rhs: &MatrixNxM<N, M>) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize, const M: usize> Mul<MatrixNxM<N, M>> for &VectorND<N> {
    type Output = VectorND<M>;
    #[inline]
    fn mul(self, rhs: MatrixNxM<N, M>) -> Self::Output {
        self * &rhs
    }
}