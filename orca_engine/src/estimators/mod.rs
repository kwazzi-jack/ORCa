use num_complex::Complex;

#[allow(non_snake_case)]
pub mod predictor;

#[allow(non_snake_case)]
pub mod filter;

#[allow(non_snake_case)]
pub mod smoother;

struct ScalarKalmanFilter<T> {
    dtype: T,
    size: usize,
}

impl<T> ScalarKalmanFilter<T>
where 
    T: num_traits::Float, 
{
    pub fn new(dtype: T, size: usize) -> Self {
        todo!();
    }
}