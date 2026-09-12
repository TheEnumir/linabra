mod constructors;
mod operations;
mod transformations;
mod debug;

/// Структура вектора-строки в N-мерном пространстве.
/// Для структуры реализованны трейты [`Clone`], [`Copy`] и основные арифметические операции.
/// 
/// Для создания нового вектора используйте макрос [`crate::vectorND!`].
/// # Examples
/// ```
/// #use linabra::*;
/// 
/// assert_eq!(VectorND::new([2.0, 1.0, 3.0]), vectorND[2.0, 1.0, 3.0]) //N автоматически выведется как 3.
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct 
VectorND<const N: usize> {
    arr: [f64; N]
}