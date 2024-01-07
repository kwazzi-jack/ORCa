use crate::core::{Float32, Float64, Complex32, Complex64};

pub trait PredictorOps {
    type Output;

    fn predict(mf: Self, Pf: Self, F: Self, Q: Self) -> (Self::Output, Self::Output);
}

impl PredictorOps for Float32 {
    type Output = Float32;

    fn predict(mf: Self, Pf: Self, F: Self, Q: Self) -> (Self::Output, Self::Output) {
        (F * mf, F.powi(2) * Pf + Q)
    }
}

impl PredictorOps for Float64 {
    type Output = Float64;

    fn predict(mf: Self, Pf: Self, F: Self, Q: Self) -> (Self::Output, Self::Output) {
        (F * mf, F.powi(2) * Pf + Q)
    }
}

impl PredictorOps for Complex32 {
    type Output = Complex32;

    fn predict(mf: Self, Pf: Self, F: Self, Q: Self) -> (Self::Output, Self::Output) {
        let F_norm_sqr = Complex32::new(F.norm_sqr(), 0.0);
        (F * mf, F_norm_sqr * Pf + Q)
    }
}

impl PredictorOps for Complex64 {
    type Output = Complex64;

    fn predict(mf: Self, Pf: Self, F: Self, Q: Self) -> (Self::Output, Self::Output) {
        let F_norm_sqr = Complex64::new(F.norm_sqr(), 0.0);
        (F * mf, F_norm_sqr * Pf + Q)
    }
}