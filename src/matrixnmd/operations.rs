use crate::{MatrixNxM, VectorND};

mod vec_mul;
mod mat_mul;
mod add_sub;
mod neg;
mod index;
mod iter;
mod sum;
mod product;

impl<const N: usize, const M: usize> MatrixNxM<N, M> {
    #[inline]
    pub fn transpose(self) -> MatrixNxM<M, N> {
        let columns: [VectorND<N>; M] = std::array::from_fn(|m| {
            std::array::from_fn(|n| self[n][m]).into()
        });
        MatrixNxM::<M, N> { columns }
    }
}