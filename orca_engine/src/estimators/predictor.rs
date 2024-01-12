use nalgebra::{Matrix, Const, DimName, Scalar, SMatrix, DMatrix, DVector};
use crate::core::{Float, Complex};

pub fn predict_scalar_real<T>(mf: T, Pf: T, F: T, Q: T) -> (T, T) 
where 
    T: Float,
{
    (
        F * mf, 
        F.powi(2) * Pf + Q
    )
}

pub fn predict_scalar_complex<T>(mf: Complex<T>, Pf: Complex<T>, F: Complex<T>, Q: Complex<T>) -> (Complex<T>, Complex<T>) 
where 
    T: Float,
{
    let F_norm_sqr = Complex::new(F.norm_sqr(), T::zero());
    (
        F * mf, 
        F_norm_sqr * Pf + Q
    )
}

pub fn predict_matrix_real<T>(mf: &DVector<T>, Pf: &DMatrix<T>, F: &DMatrix<T>, Q: &DMatrix<T>) -> (DVector<T>, DMatrix<T>) 
where 
    T: Float + Scalar,
{
    (
        F * mf, 
        F * Pf * F.transpose() + Q
    )
}
