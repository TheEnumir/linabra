use crate::VectorND;
use std::ops::{Add, AddAssign};
use std::borrow::Borrow;

impl<const N: usize> VectorND<N> {
    #[inline]
    fn generic_alloc_add(a: impl Borrow<VectorND<N>>, b: impl Borrow<VectorND<N>>) -> VectorND<N> {
        let a = a.borrow();
        let b = b.borrow();
        VectorND::from_fn(|i| a[i] + b[i])
    }

    #[inline]
    fn generic_add_assign(&mut self, other: impl Borrow<VectorND<N>>) {
        for (s, o) in self.iter_mut().zip(other.borrow()) {
            *s += o
        }
    }
}

macro_rules! impl_add_assign {
    ($rhs:ty) => {
        impl<const N: usize> AddAssign<$rhs> for VectorND<N> {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                self.generic_add_assign(rhs);
            }
        }
    };
}

impl_add_assign!(VectorND<N>);
impl_add_assign!(&VectorND<N>);
impl_add_assign!(&mut VectorND<N>);

macro_rules! impl_add {
    ($lhs:ty, $rhs:ty, $logic_closure:expr) => {
        impl<const N: usize> Add<$rhs> for $lhs {
            type Output = VectorND<N>;
            #[inline]
            fn add(self, rhs: $rhs) -> Self::Output {
                $logic_closure(self, rhs)
            }
        }
    };
    ($lhs:ty, $rhs:ty) => {
        impl_add!($lhs, $rhs, |lhs, rhs| VectorND::generic_alloc_add(lhs, rhs));
    };
}

impl_add!(VectorND<N>, VectorND<N>, |mut lhs, rhs| { lhs += rhs; lhs });
impl_add!(VectorND<N>, &VectorND<N>, |mut lhs, rhs| { lhs += rhs; lhs });
impl_add!(&VectorND<N>, VectorND<N>, |lhs, mut rhs| { rhs += lhs; rhs });
impl_add!(VectorND<N>, &mut VectorND<N>, |mut lhs, rhs| { lhs += rhs; lhs });
impl_add!(&mut VectorND<N>, VectorND<N>, |lhs, mut rhs| { rhs += lhs; rhs });

impl_add!(&VectorND<N>, &VectorND<N>);
impl_add!(&mut VectorND<N>, &VectorND<N>);
impl_add!(&VectorND<N>, &mut VectorND<N>);
impl_add!(&mut VectorND<N>, &mut VectorND<N>);