use crate::{MatrixNxM, VectorND};

///- matrixNxM![]
#[macro_export]
macro_rules! matrixNxM {
    () => {
        $crate::MatrixNxM::<0, 0>::new([])
    };

    (id; $n:literal x $m:literal) => {{
        const _: () = { if $n != $m { panic!("identity существует только для квадратных матриц!") } };
        $crate::MatrixNxM::<$n, $m>::identity()
    }};

    ($elem:expr; $n:literal x $m:literal) => {
        $crate::MatrixNxM::<$n, $m>::from_elem($elem)
    };

    ($($($value:expr),+ $(,)?);+ $(;)?) => {
        $crate::MatrixNxM::new_row_ordered([$(
            [$($value),*]
        ),+])
    };
}

impl<const N: usize, const M: usize> MatrixNxM<N, M> {
    #[inline]
    pub fn new(columns: [[f64; M]; N]) -> Self {
        let columns: [VectorND<M>; N] = std::array::from_fn(|i| {
            VectorND::new(columns[i])
        });
        Self { columns }
    }
    #[inline]
    pub fn new_row_ordered(rows: [[f64; N]; M]) -> Self {
        MatrixNxM::<M, N>::new(rows).transpose()
    }
    #[inline]
    pub fn zero() -> Self {
        Self { columns: std::array::from_fn(|_| VectorND::<M>::zero()) }
    }

    #[inline]
    pub fn from_fn(f: impl FnMut(usize)->VectorND<M>) -> Self {
        Self { columns: std::array::from_fn(f) }
    }

    #[inline]
    pub fn from_elem(el: f64) -> Self {
        Self::new([[el; M]; N])
    }
}

impl<const N: usize> MatrixNxM<N, N> {
    #[inline]
    pub fn identity() -> Self {
        Self::new(
            std::array::from_fn(|i| {
                std::array::from_fn(|j| { if i == j {1.0} else {0.0} })
            })
        )
    }
}

