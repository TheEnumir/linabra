use crate::VectorND;
use std::iter::Sum;
use std::borrow::Borrow;

#[inline]
fn generic_sum<I, K, const N: usize>(mut iter: I) -> VectorND<N>
where
    I: Iterator<Item = K>,
    K: Borrow<VectorND<N>>,
{
    let mut s = iter.next().map_or_else(VectorND::zero, |v| v.borrow().clone());
    for v in iter {
        s += v.borrow();
    }
    s
}

impl<const N: usize> Sum for VectorND<N> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        generic_sum(iter)
    }
}

impl<'a, const N: usize> Sum<&'a VectorND<N>> for VectorND<N> {
    #[inline]
    fn sum<I: Iterator<Item = &'a VectorND<N>>>(iter: I) -> Self {
        generic_sum(iter)
    }
}