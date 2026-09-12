use crate::VectorND;
use std::ops::{Div, DivAssign};

//Образующие
impl<const N: usize> Div<f64> for &VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self.clone() * (1.0 / rhs)
    }
}

impl<const N: usize> DivAssign<f64> for VectorND<N> {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

//Образованные
impl<const N: usize> Div<f64> for VectorND<N> {
    type Output = Self;
    #[inline]
    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs; self
    }
}