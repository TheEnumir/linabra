use std::ops::{Sub, SubAssign};
use crate::VectorND;

//Образующие
impl<const N: usize> Sub for &VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::from_fn(|i| self[i] - rhs[i])
    }
}

impl<const N: usize> SubAssign<&VectorND<N>> for VectorND<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: &VectorND<N>) {
        for i in 0..N {
            self[i] -= rhs[i]
        }
    }
}

//Образованные
impl<const N: usize> Sub<VectorND<N>> for &VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn sub(self, mut rhs: VectorND<N>) -> Self::Output {
        rhs -= self; -rhs
    }
}

impl<const N: usize> Sub<&VectorND<N>> for VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn sub(mut self, rhs: &VectorND<N>) -> Self::Output {
        self -= rhs; self
    }
}

impl<const N: usize> SubAssign for VectorND<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs);
    }
}

impl<const N: usize> Sub for VectorND<N> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs; self
    }
}