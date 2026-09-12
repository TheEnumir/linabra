use crate::VectorND;

/// -vectorND![]
/// 
/// Макрос для удобного создания нового вектора.
/// 
/// # Examples
/// ```
/// # use linabra::*;
/// 
/// assert_eq!(VectorND::new([2.0, 3.0, -10.0]), vectorND![2.0, 3.0, -10.0]);
/// assert_eq!(VectorND::new([1.0, 1.0, 1.0]), vectorND![1.0; 3]);
/// assert_eq!(VectorND::new([]), vectorND![]);
/// 
/// ```
#[macro_export]
macro_rules! vectorND {
    () => {
        $crate::VectorND::<0>::new([])
    };
    
    ($elem:expr; $n:literal) => {
        $crate::VectorND::<$n>::from_elem($elem)
    };

    ($( $value:expr ),+ $(,)?) => {
        $crate::VectorND::from([$($value),+])
    };
}

/// Конструкторы для [`VectorND`].
impl<const N: usize> VectorND<N> {
    /// Базовый конструктор
    #[inline]
    pub const fn new(arr: [f64; N]) -> Self {
        Self { arr }
    }

    /// Конструктор с заполнением нулями.
    #[inline]
    pub const fn zero() -> Self {
        Self { arr: [0.0; N] }
    }

    /// Создание [`VectorND`] из замыкания, принимающего индекс элемента и возвращающего f64.
    /// Аналогичен функции [`std::array::from_fn`] для массивов.
    #[inline]
    pub fn from_fn(f: impl FnMut(usize) -> f64) -> VectorND<N> {
        Self::new(std::array::from_fn(f))
    }
    
    /// Создает новый вектора, заполненный одним значением.
    #[inline]
    pub fn from_elem(el: f64) -> Self {
        Self { arr: [el; N] }
    }
}

/// Реализация трейта Default.
/// Возвращает вектор, заполненный нулями.
impl<const N: usize> Default for VectorND<N> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

/// Создает вектор из массива f64.
impl<const N: usize> From<[f64; N]> for VectorND<N> {
    #[inline]
    fn from(value: [f64; N]) -> Self {
        Self::new(value)
    }
}

/// Создание из 3-х мерного вектора 4-х мерный.
/// Заполняет координату w единицей.
impl From<VectorND<3>> for VectorND<4> {
    fn from(value: VectorND<3>) -> Self {
        Self::new([value[0], value[1], value[2], 1.0])
    }
}

/// Создание из 4-х мерного вектора 3-х мерный.
/// Делит каждую координату итогового вектора на координату w исходного.
impl From<VectorND<4>> for VectorND<3> {
    fn from(value: VectorND<4>) -> Self {
        let res = Self::new([value[0], value[1], value[2]]);
        let w = value[3];
        if w != 0.0 { res / w } else { res }
    }
}