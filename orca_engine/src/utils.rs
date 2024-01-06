extern crate rand_distr;
extern crate nalgebra as na;

use rand::Rng;
use rand_distr::StandardNormal;
use na::{DMatrix, DVector, SVD};
use log::{info, debug, error};

type KernelPointer = Box<dyn Fn(f64, f64, f64, f64) -> f64>;

pub fn squared_exponential_kernel(x: f64, y: f64, std: f64, scale: f64) -> f64 {
    std.powi(2) * (-0.5 * ((x - y) / scale).powi(2)).exp()
}

pub enum Kernels {
    SqrExp,
}

impl Kernels {
    pub fn kernel(&self) -> KernelPointer {
        match self {
            Kernels::SqrExp => Box::new(squared_exponential_kernel),
        }
    }
}

pub struct GaussianProcess {
    mean: f64,
    std: f64,
    scale: f64,
    kernel: KernelPointer,
    length: usize,
}

impl GaussianProcess {
    pub fn new(mean: f64, std: f64, scale: f64,
        kernel: KernelPointer, length: usize) -> Self {
        Self {
            mean,
            std,
            scale,
            kernel,
            length,
        }
    }

    pub fn generate(&self) -> DVector<f64> {
        // Calculate covariance matrix
        let covariance_matrix = self.calculate_covariance_matrix();

        // Perform SVD
        let svd = SVD::new(covariance_matrix, true, true);
        let u = svd.u.expect("Failed to compute U matrix");
        let singular_values = svd.singular_values;
        let v_t = svd.v_t.expect("Failed to compute V^T matrix");

        // Convert singular values to a diagonal matrix
        let sigma_matrix = DMatrix::from_diagonal(&singular_values.map(|x| x.sqrt()));

        // Generate standard normal samples
        let mut rng = rand::thread_rng();
        let z: DVector<f64> = DVector::from_iterator(self.length, (0..self.length).map(|_| rng.sample(StandardNormal)));
        debug!("Axis samples drawn.");

        // Transform using SVD components
        let samples = u * sigma_matrix * v_t.transpose() * z;
        debug!("Derived GP from SVD and samples.");

        // Add the mean
        info!("Generated Gaussian Process data.");
        samples.map(|v| v + self.mean)
    }

    fn calculate_covariance_matrix(&self) -> DMatrix<f64> {
        let mut matrix = DMatrix::zeros(self.length, self.length);
        for i in 0..self.length {
            for j in 0..self.length {                
                // Use the kernel function to compute the covariance
                matrix[(i, j)] = squared_exponential_kernel(i as f64, j as f64, self.std, self.scale);
            }
        }
        matrix
    }
}

fn condition_number(matrix: &DMatrix<f64>) -> f64 {
    let svd = matrix.clone().svd(true, true);
    let singular_values = svd.singular_values;

    let max_singular = singular_values.max();
    let min_singular = singular_values.min();

    max_singular / min_singular
}