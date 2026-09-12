use crate::VectorND;

mod add;
mod sub;
mod neg;
mod mul;
mod div;
mod index;
mod sum;
mod iter;

impl<const N: usize> VectorND<N> {
    /// Скалярное произведение двух векторов.
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.iter()
            .zip(other.arr.iter())
            .map(|(s, o)| s * o)
            .sum()
    }
    /// Квадрат длины вектора. Быстрее чем [`VectorND::length`].
    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }
    /// Возвращает длину вектор.
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    /// Возвращает новый сонаправеллный вектор единичной длины.
    #[inline]
    pub fn norm(&self) -> Self {
        self / self.length()
    }

}

impl VectorND<3> {
    /// Возвращает векторное произведение. Специфично для 3-х мерного вектора.
    #[inline]
    pub fn cross_product(self, other: Self) -> Self {
        Self::new([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }
}