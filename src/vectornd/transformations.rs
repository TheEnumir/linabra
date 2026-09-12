use crate::VectorND;

/// Преобразование из 3-х мерного вектора в 4-х мерный.
/// Заполняет координату w единицей.
impl VectorND<3> {
    #[inline]
    pub fn into_v4(self) -> VectorND<4> {
        self.into()
    }
}

/// Преобразование из 4-х мерного вектора в 3-х мерный.
/// Делит каждую координату итогового вектора на координату w исходного.
impl VectorND<4> {
    #[inline]
    pub fn into_v3(self) -> VectorND<3> {
        self.into()
    }
}

impl<const N: usize> From<VectorND<N>> for [f64; N] {
    #[inline]
    fn from(value: VectorND<N>) -> Self {
        value.arr
    }
}

impl<const N: usize> From<VectorND<N>> for [f32; N] {
    #[inline]
    fn from(value: VectorND<N>) -> Self {
        std::array::from_fn(|i| value[i] as f32)
    }
}