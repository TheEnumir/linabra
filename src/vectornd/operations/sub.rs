use crate::VectorND;
use std::ops::{Sub, SubAssign};
use std::borrow::Borrow;

impl<const N: usize> VectorND<N> {
    #[inline]
    fn generic_alloc_sub(a: impl Borrow<VectorND<N>>, b: impl Borrow<VectorND<N>>) -> VectorND<N> {
        let a = a.borrow();
        let b = b.borrow();
        VectorND::from_fn(|i| a[i] - b[i])
    }

    #[inline]
    fn generic_sub_assign(&mut self, other: impl Borrow<VectorND<N>>) {
        for (s, o) in self.iter_mut().zip(other.borrow()) {
            *s -= o
        }
    }

    #[inline]
    fn generic_reverse_sub_assign(&mut self, other: impl Borrow<VectorND<N>>) {
        for (s, o) in self.iter_mut().zip(other.borrow()) {
            *s = *o - *s
        }
    }
}

macro_rules! impl_sub_assign {
    ($rhs:ty) => {
        impl<const N: usize> SubAssign<$rhs> for VectorND<N> {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs) {
                self.generic_sub_assign(rhs);
            }
        }
    };
}

impl_sub_assign!(VectorND<N>);
impl_sub_assign!(&VectorND<N>);
impl_sub_assign!(&mut VectorND<N>);

macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty, $logic_closure:expr) => {
        impl<const N: usize> Sub<$rhs> for $lhs {
            type Output = VectorND<N>;
            #[inline]
            fn sub(self, rhs: $rhs) -> Self::Output {
                $logic_closure(self, rhs)
            }
        }
    };
    ($lhs:ty, $rhs:ty) => {
        impl_sub!($lhs, $rhs, |lhs, rhs| VectorND::generic_alloc_sub(lhs, rhs));
    };
}

impl_sub!(VectorND<N>, VectorND<N>, |mut lhs, rhs| { lhs -= rhs; lhs });
impl_sub!(VectorND<N>, &VectorND<N>, |mut lhs, rhs| { lhs -= rhs; lhs });
impl_sub!(&VectorND<N>, VectorND<N>, |lhs, mut rhs: VectorND<N>| { rhs.generic_reverse_sub_assign(lhs); rhs });
impl_sub!(VectorND<N>, &mut VectorND<N>, |mut lhs, rhs| { lhs -= rhs; lhs });
impl_sub!(&mut VectorND<N>, VectorND<N>, |lhs, mut rhs: VectorND<N>| { rhs.generic_reverse_sub_assign(lhs); rhs });

impl_sub!(&VectorND<N>, &VectorND<N>);
impl_sub!(&mut VectorND<N>, &VectorND<N>);
impl_sub!(&VectorND<N>, &mut VectorND<N>);
impl_sub!(&mut VectorND<N>, &mut VectorND<N>);