#[allow(non_snake_case)]
mod calibrator;
mod utils;

use log::debug;
use utils::{GaussianProcess, Kernels};
use rand_distr::{Normal, Distribution};
use hdf5::File;

struct Axis {
    mu: f64,
    sigma: f64,
}

fn measurement_operator(input: f64) -> f64 {
    // Example: Damped harmonic oscillator
    0.5 * ((2.0 * std::f64::consts::PI * input).cos() * (-input / 8.0).exp())
}

fn simulate_data(input_data: &Vec<f64>, meas_axis: Axis) -> (Vec<f64>, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let meas_dist = Normal::new(meas_axis.mu, meas_axis.sigma).unwrap();

    let output_data: Vec<f64> = input_data
                                .iter()
                                .map(|&x| measurement_operator(x))
                                .collect();
                            
    let noisy_data: Vec<f64> = output_data
                               .iter()
                               .map(|&x| x + meas_dist.sample(&mut rng))
                               .collect();

    (output_data, noisy_data)
}

fn main() -> hdf5::Result<()> {
    // Initialise `log4rs` from config file
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    // Gaussian Process Parameters
    const N_TIME: usize = 1000; 
    let kernel = Kernels::SqrExp.kernel();
    let gp = GaussianProcess::new(1.0, 1.0, 1.0, kernel, N_TIME);
    debug!("Gaussian Process (GP) instantiated.");

    // Generate data using Gaussian Process
    let input_data: Vec<f64> = gp.generate().iter().cloned().collect();
    debug!("Input data created from GP.");

    // Measurable Signal Axis
    let meas_axis = Axis { mu: 0.0, sigma: 0.5 };

    // Simulate data using the measurement operator and noise
    let (output_data, noisy_data) = simulate_data(&input_data, meas_axis);
    debug!("Output data created from input data.");

    // Create a new HDF5 file
    let file = File::create("data.h5").unwrap();
    let group = file.create_group("data").unwrap();

    // Create datasets
    group.new_dataset::<f64>()
        .create("input_data", input_data.len())
        .unwrap()
        .write(&input_data)
        .unwrap();
    group.new_dataset::<f64>()
        .create("output_data", output_data.len())
        .unwrap()
        .write(&output_data)
        .unwrap();
    group.new_dataset::<f64>()
        .create("noisy_data", noisy_data.len())
        .unwrap()
        .write(&noisy_data)
        .unwrap();

    Ok(())
}   

