use crate::VectorND;
use std::ops::{Div, DivAssign};
use std::borrow::Borrow;

impl<const N: usize> VectorND<N> {
    #[inline]
    fn generic_alloc_div(a: impl Borrow<Self>, b: f64) -> Self {
        let a = a.borrow();
        let factor = 1.0/b;
        Self::from_fn(|i| a[i] * factor)
    }
}

impl<const N: usize> DivAssign<f64> for VectorND<N> {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        let factor = 1.0/rhs;
        for s in self.iter_mut() {
            *s *= factor;
        }
    }
}

macro_rules! impl_div {
    ($lhs:ty, f64, $logic_closure:expr) => {
        impl<const N: usize> Div<f64> for $lhs {
            type Output = VectorND<N>;
            #[inline]
            fn div(self, rhs: f64) -> Self::Output {
                $logic_closure(self, rhs)
            }
        }
    };

    ($lhs:ty, f64) => {
        impl_div!($lhs, f64, VectorND::generic_alloc_div);
    };
}

impl_div!(VectorND<N>, f64, |mut lhs, rhs| { lhs /= rhs; lhs });
impl_div!(&VectorND<N>, f64);
impl_div!(&mut VectorND<N>, f64);