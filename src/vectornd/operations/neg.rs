use std::ops::Neg;
use crate::VectorND;

macro_rules! impl_neg {
    ($vec_type:ty, $logic_closure:expr) => {
        impl<const N: usize> Neg for $vec_type {
            type Output = VectorND<N>;
            #[inline]
            fn neg(self) -> Self::Output {
                $logic_closure(self)
            }
        }
    };
}

impl_neg!(VectorND<N>, |mut v| {v *= -1.0; v});
impl_neg!(&VectorND<N>, |v: &VectorND<N>| -v.clone());
impl_neg!(&mut VectorND<N>, |v: &mut VectorND<N>| -v.clone());