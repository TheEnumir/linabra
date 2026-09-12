use std::ops::{Add, AddAssign};
use crate::VectorND;

//Образующие
impl<const N: usize> Add for &VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::from_fn(|i| self[i] + rhs[i])
    }
}

impl<const N: usize> AddAssign<&VectorND<N>> for VectorND<N> {
    #[inline]
    fn add_assign(&mut self, rhs: &VectorND<N>) {
        for i in 0..N {
            self[i] += rhs[i]
        }
    }
}

//Образованные
impl<const N: usize> Add<VectorND<N>> for &VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn add(self, mut rhs: VectorND<N>) -> Self::Output {
        rhs += self; rhs
    }
}

impl<const N: usize> Add<&VectorND<N>> for VectorND<N> {
    type Output = VectorND<N>;
    #[inline]
    fn add(mut self, rhs: &VectorND<N>) -> Self::Output {
        self += rhs; self
    }
}

impl<const N: usize> AddAssign for VectorND<N> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
    }
}

impl<const N: usize> Add for VectorND<N> {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs; self
    }
}