use crate::MatrixNxM;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::borrow::Borrow;

//---Реализуем операции += и -= для всех типов, которые можно преобразовать к &MatrixNxM<N, M>

impl<const N: usize, const M: usize, RHS> AddAssign<RHS> for MatrixNxM<N, M>
where RHS: Borrow<MatrixNxM<N, M>>
{
    #[inline]
    fn add_assign(&mut self, rhs: RHS) {
        for i in 0..N {
            self[i] += rhs.borrow()[i]
        }
    }
}

impl<const N: usize, const M: usize, RHS> SubAssign<RHS> for MatrixNxM<N, M>
where RHS: Borrow<MatrixNxM<N, M>>
{
    #[inline]
    fn sub_assign(&mut self, rhs: RHS) {
        for i in 0..N {
            self[i] -= rhs.borrow()[i]
        }
    }
}

//---Generic методы---

impl<const N: usize, const M: usize> MatrixNxM<N, M> {
    fn generic_alloc_add(a: impl Borrow<Self>, b: impl Borrow<Self>) -> Self {
        let a = a.borrow();
        let b = b.borrow();
        Self::from_fn(|i| a[i] + b[i])
    }
}

impl<const N: usize, const M: usize> MatrixNxM<N, M> {
    fn generic_alloc_sub(a: impl Borrow<Self>, b: impl Borrow<Self>) -> Self {
        let a = a.borrow();
        let b = b.borrow();
        Self::from_fn(|i| a[i] - b[i])
    }
}

impl<const N: usize, const M: usize> MatrixNxM<N, M> {
    fn reversed_sub_assign(a: impl Borrow<Self>, b: &mut MatrixNxM<N, M>) {
        let a = a.borrow();
        for i in 0..N {
            b[i] = a[i] - b[i]
        }
    }
}

impl<const N: usize, const M: usize> MatrixNxM<N, M> {
    fn reversed_add_assign(a: impl Borrow<Self>, b: &mut MatrixNxM<N, M>) {
        let a = a.borrow();
        for i in 0..N {
            b[i] = a[i] + b[i]
        }
    }
}

//---Макросы---

macro_rules! impl_op {
    (left, $trate:ident, $trate_method:ident, $assign_op:ident, $lhs:ty, $rhs:ty) => {
        impl<const N: usize, const M: usize> $trate<$rhs> for $lhs {
            type Output = MatrixNxM<N, M>;
            fn $trate_method(mut self, rhs: $rhs) -> Self::Output {
                MatrixNxM::<N, M>::$assign_op(&mut self, rhs); self
            }
        }
    };

    (right, $trate:ident, $trate_method:ident, $assign_op:ident, $lhs:ty, $rhs:ty) => {
        impl<const N: usize, const M: usize> $trate<$rhs> for $lhs {
            type Output = MatrixNxM<N, M>;
            fn $trate_method(self, mut rhs: $rhs) -> Self::Output {
                MatrixNxM::<N, M>::$assign_op(self, &mut rhs); rhs
            }
        }
    };

    ($trate:ident, $trate_method:ident, $generic_method:ident, $lhs:ty, $rhs:ty) => {
        impl<const N: usize, const M: usize> $trate<$rhs> for $lhs {
            type Output = MatrixNxM<N, M>;
            fn $trate_method(self, rhs: $rhs) -> Self::Output {
                MatrixNxM::<N, M>::$generic_method(self, rhs)
            }
        }
    };
}

//---Имплементация через макросы---

impl_op!(Add, add, generic_alloc_add, &MatrixNxM<N, M>, &MatrixNxM<N, M>);
impl_op!(Add, add, generic_alloc_add, &mut MatrixNxM<N, M>, &MatrixNxM<N, M>);
impl_op!(Add, add, generic_alloc_add, &MatrixNxM<N, M>, &mut MatrixNxM<N, M>);
impl_op!(Add, add, generic_alloc_add, &mut MatrixNxM<N, M>, &mut MatrixNxM<N, M>);

impl_op!(Sub, sub, generic_alloc_sub, &MatrixNxM<N, M>, &MatrixNxM<N, M>);
impl_op!(Sub, sub, generic_alloc_sub, &mut MatrixNxM<N, M>, &MatrixNxM<N, M>);
impl_op!(Sub, sub, generic_alloc_sub, &MatrixNxM<N, M>, &mut MatrixNxM<N, M>);
impl_op!(Sub, sub, generic_alloc_sub, &mut MatrixNxM<N, M>, &mut MatrixNxM<N, M>);

impl_op!(left, Add, add, add_assign, MatrixNxM<N, M>, MatrixNxM<N, M>);
impl_op!(left, Add, add, add_assign, MatrixNxM<N, M>, &MatrixNxM<N, M>);
impl_op!(left, Add, add, add_assign, MatrixNxM<N, M>, &mut MatrixNxM<N, M>);
impl_op!(right, Add, add, reversed_add_assign, &MatrixNxM<N, M>, MatrixNxM<N, M>);
impl_op!(right, Add, add, reversed_add_assign, &mut MatrixNxM<N, M>, MatrixNxM<N, M>);

impl_op!(left, Sub, sub, sub_assign, MatrixNxM<N, M>, MatrixNxM<N, M>);
impl_op!(left, Sub, sub, sub_assign, MatrixNxM<N, M>, &MatrixNxM<N, M>);
impl_op!(left, Sub, sub, sub_assign, MatrixNxM<N, M>, &mut MatrixNxM<N, M>);
impl_op!(right, Sub, sub, reversed_sub_assign, &MatrixNxM<N, M>, MatrixNxM<N, M>);
impl_op!(right, Sub, sub, reversed_sub_assign, &mut MatrixNxM<N, M>, MatrixNxM<N, M>);