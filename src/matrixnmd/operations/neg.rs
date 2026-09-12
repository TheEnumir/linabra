use crate::MatrixNxM;
use std::ops::Neg;

impl<const N: usize, const M: usize> Neg for MatrixNxM<N, M> {
    type Output = Self;
    #[inline]
    fn neg(mut self) -> Self::Output {
        for column in self.iter_mut() {
            *column *= -1.0;
        }
        self
    }
}

impl<const N: usize, const M: usize> Neg for &MatrixNxM<N, M> {
    type Output = MatrixNxM<N, M>;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::from_fn(|i| -self[i])
    }
}