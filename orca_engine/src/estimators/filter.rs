use num_complex::Complex;

#[inline]
pub fn scalar_real_filter<T>(mp: T, Pp: T, y: T, H: T, R: T) -> (T, T) 
where 
    T: num_traits::Float,
{
    let e = y - H * mp;
    let S = H.powi(2) * Pp + R;
    let K = Pp * H / S;

    (mp + K * e, (T::one() - K * H) * Pp)
} 

#[inline]
pub fn scalar_complex_filter<T>(mp: Complex<T>, Pp: Complex<T>, y: Complex<T>, H: Complex<T>, R: Complex<T>) -> (Complex<T>, Complex<T>) 
where 
    T: num_traits::Float, 
{
    let H_norm_sqr = Complex::new(H.norm_sqr(), T::zero());
    let e = y - H * mp;
    let S = H_norm_sqr * Pp + R;
    let K = Pp * H.conj() / S;

    (mp + K * e, (Complex::new(T::one(), T::zero()) - K * H) * Pp)
}