use crate::VectorND;
use std::ops::{Mul, MulAssign};

//Образующие
impl<const N: usize> Mul<f64> for &VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        VectorND::<N>::from_fn(|i| self[i] * rhs)
    }
}

impl<const N: usize> MulAssign<f64> for VectorND<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..N {
            self[i] *= rhs
        }
    }
}

//Образованные
impl<const N: usize> Mul<f64> for VectorND<N> {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs; self
    }
}

impl<const N: usize> Mul<VectorND<N>> for f64 {
    type Output = VectorND<N>;
    #[inline]
    fn mul(self, rhs: VectorND<N>) -> Self::Output {
        rhs * self
    }
}

impl<const N: usize> Mul<&VectorND<N>> for f64 {
    type Output = VectorND<N>;
    #[inline]
    fn mul(self, rhs: &VectorND<N>) -> Self::Output {
        rhs * self
    }
}