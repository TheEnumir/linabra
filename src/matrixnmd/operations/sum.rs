use crate::MatrixNxM;
use std::iter::Sum;
use std::borrow::Borrow;

#[inline]
fn generic_sum<I, B, const N: usize, const M: usize>(mut iter: I) -> MatrixNxM<N, M>
where
    I: Iterator<Item = B>,
    B: Borrow<MatrixNxM<N, M>>
{
    let mut s = iter.next().map_or_else(MatrixNxM::zero, |m| m.borrow().clone());
    for m in iter {
        s += m.borrow();
    }
    s
}

impl<const N: usize, const M: usize> Sum for MatrixNxM<N, M> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        generic_sum(iter)
    }
}

impl<'a, const N: usize, const M: usize> Sum<&'a MatrixNxM<N, M>> for MatrixNxM<N, M> {
    #[inline]
    fn sum<I: Iterator<Item = &'a MatrixNxM<N, M>>>(iter: I) -> Self {
        generic_sum(iter)
    }
}