use crate::VectorND;
use core::slice;
use std::iter::IntoIterator;

impl<const N: usize> IntoIterator for VectorND<N> {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, N>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a VectorND<N> {
    type Item = &'a f64;
    type IntoIter = std::slice::Iter<'a, f64>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.arr.iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut VectorND<N> {
    type Item = &'a mut f64;
    type IntoIter = std::slice::IterMut<'a, f64>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.arr.iter_mut()
    }
}

impl<'a, const N: usize> VectorND<N> {
    pub fn iter(&'a self) -> slice::Iter<'a, f64> {
        self.into_iter()
    }

    pub fn iter_mut(&'a mut self) -> slice::IterMut<'a, f64> {
        self.into_iter()
    }
}