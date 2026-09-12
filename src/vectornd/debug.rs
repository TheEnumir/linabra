use crate::VectorND;

impl<const N: usize> std::fmt::Debug for VectorND<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Вектор {}D: ", N)?;
        std::fmt::Debug::fmt(&self.arr, f)?;
        Ok(())
    }
}