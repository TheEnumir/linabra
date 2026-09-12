use std::ops::Neg;
use crate::VectorND;

impl<const N: usize> Neg for VectorND<N> {
    type Output = Self;
    #[inline]
    fn neg(mut self) -> Self::Output {
        self *= -1.0; self
    }
}

impl<const N: usize> Neg for &VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}