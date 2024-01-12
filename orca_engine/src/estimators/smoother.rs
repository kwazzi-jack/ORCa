use num_complex::Complex;

#[inline]
pub fn scalar_real_smoother<T>(
    mp: T, Pp: T, mf: T, Pf: T, 
    ms: T, Ps: T, F: T) -> (T, T) 
where 
    T: num_traits::Float,
{
    let G = Pf * F / Pp;
    (mf + G * (ms - mp), Pf + G.powi(2) * (Ps - Pp))
} 

#[inline]
pub fn scalar_complex_predictor<T>(
    mp: Complex<T>, Pp: Complex<T>, mf: Complex<T>, Pf: Complex<T>, 
    ms: Complex<T>, Ps: Complex<T>, F: Complex<T>) -> (Complex<T>, Complex<T>) 
where 
    T: num_traits::Float, 
{
    let G = Pf * F.conj() / Pp;
    let G_norm_sqr = Complex::new(G.norm_sqr(), T::zero());
    (mf + G * (ms - mp), Pf + G_norm_sqr * (Ps - Pp))
}