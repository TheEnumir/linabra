use crate::MatrixNxM;

impl<const N: usize, const M: usize> std::fmt::Debug for MatrixNxM<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let transposed = self.transpose();

        writeln!(f, "Матрица {}x{}: [", N, M)?;

        for v in transposed.columns.into_iter() {
            writeln!(f, "   {:?}", <[f64; N]>::from(v))?;
        }

        write!(f, "]")?;

        Ok(())
    }
}