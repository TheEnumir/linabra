use crate::VectorND;
use std::{borrow::Borrow, ops::{Mul, MulAssign}};

impl<const N: usize> VectorND<N> {
    #[inline]
    fn generic_alloc_mul(a: impl Borrow<Self>, b: f64) -> Self {
        let a = a.borrow();
        Self::from_fn(|i| a[i] * b)
    }
}

impl<const N: usize> MulAssign<f64> for VectorND<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        for s in self.iter_mut() {
            *s *= rhs;
        }
    }
}

macro_rules! impl_mul {
    ($lhs:ty, $rhs:ty, $logic_closure:expr) => {
        impl<const N: usize> Mul<$rhs> for $lhs {
            type Output = VectorND<N>;
            #[inline]
            fn mul(self, rhs: $rhs) -> Self::Output {
                $logic_closure(self, rhs)
            }
        }
    };

    ($lhs:ty, f64) => {
        impl_mul!($lhs, f64, VectorND::generic_alloc_mul);
    };

    (f64, $rhs:ty) => {
        impl_mul!(f64, $rhs, |lhs, rhs| VectorND::generic_alloc_mul(rhs, lhs));
    };
}

impl_mul!(VectorND<N>, f64, |mut lhs, rhs| { lhs *= rhs; lhs });
impl_mul!(&VectorND<N>, f64);
impl_mul!(&mut VectorND<N>, f64);

impl_mul!(f64, VectorND<N>, |lhs, mut rhs| { rhs *= lhs; rhs });
impl_mul!(f64, &VectorND<N>);
impl_mul!(f64, &mut VectorND<N>);