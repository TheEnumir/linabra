mod operations;
mod constructors;
mod debug;

use super::VectorND;

#[derive(Clone, Copy, PartialEq)]
pub struct MatrixNxM<const N: usize, const M: usize> {
    columns: [VectorND<M>; N]
}