use crate::MatrixNxM;
use std::iter::Product;

impl<const N: usize> Product for MatrixNxM<N, N> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, m| acc * m).unwrap_or_else(MatrixNxM::identity)
    }
}

impl<'a, const N: usize> Product<&'a MatrixNxM<N, N>> for MatrixNxM<N, N> {
    #[inline]
    fn product<I: Iterator<Item = &'a MatrixNxM<N, N>>>(mut iter: I) -> Self {
        let mut prod = iter.next().map_or_else(MatrixNxM::identity, |m| m.clone());
        for m in iter {
            prod *= m;
        }
        prod
    }
}