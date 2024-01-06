// calibrator.rs
 // Ease on variable names

use log::debug;
use nalgebra::{DVector, DMatrix};

// Estimator Trait
trait Estimator {}

// Main estimators
struct Predictor;
struct Filter;
struct Smoother;

// Prediction Step
impl Predictor {
    fn compute(mf: f64, Pf: f64, F: f64, Q: f64) -> (f64, f64){
        return (
            F * mf,
            F * Pf * F + Q,
        )
    }
}

// Update Step
impl Filter {
    fn compute(y: f64, mp: f64, Pp: f64, H: f64, R: f64) -> (f64, f64){
        let e = y - H * mp;
        let S = H * Pp * H + R;
        let K = Pp * H / S;

        return (
            mp + K * e,
            (1.0f64 - K * H) * Pp,
        )
    }
}

// Smoothing Step
impl Smoother {
    fn compute( mp: f64, Pp: f64, mf: f64, Pf: f64, ms: f64, Ps: f64, F: f64) -> (f64, f64){
        let G = Pf * F / Pp;
        return (
            mf + G * (ms - mp),
            Pf + G * (Ps - Pp) * G,
        )
    }
}

pub struct Calibrator {
    n_time: usize,
    mp_values: Vec<f64>,
    Pp_values: Vec<f64>,
    mf_values: Vec<f64>,
    Pf_values: Vec<f64>,
    ms_values: Vec<f64>,
    Ps_values: Vec<f64>,
    y_values: Vec<f64>,
    F: f64,
    Q: f64,
    H: f64,
    R: f64,
}

impl Calibrator {
    pub fn new(y_values: Vec<f64>, F: f64, Q: f64, H: f64, R: f64) -> Self {
        let n_time = y_values.len();
        Calibrator {
            n_time: n_time,
            mp_values: Vec::with_capacity(n_time),
            Pp_values: Vec::with_capacity(n_time),
            mf_values: Vec::with_capacity(n_time),
            Pf_values: Vec::with_capacity(n_time),
            ms_values: Vec::with_capacity(n_time),
            Ps_values: Vec::with_capacity(n_time),
            y_values: y_values,
            F: F,
            Q: Q,
            H: H,
            R: R,
        }
    }

    pub fn forward(&mut self, mf0: f64, Pf0: f64) {
        // Initial Prediction Step
        let (mp, Pp) = Predictor::compute(mf0, Pf0, self.F, self.Q);
        self.mp_values.push(mp);
        self.Pp_values.push(Pp);

        // Initial Filter Step
        let y1 = self.y_values[0];
        let (mf, Pf) = Filter::compute(y1, mp, Pp, self.H, self.R);
        self.mf_values.push(mf);
        self.Pf_values.push(Pf);

        // Run Predictor and Filter from 1 to n_time
        for i in 1..self.n_time {
            // Initial Prediction Step
            let (mp, Pp) = Predictor::compute(mf, Pf, self.F, self.Q);
            self.mp_values.push(mp);
            self.Pp_values.push(Pp);

            // Update Step
            let y = self.y_values[i];
            let (mf, Pf) = Filter::compute(y, mp, Pp, self.H, self.R);
            self.mf_values.push(mf);
            self.Pf_values.push(Pf);
        }
    }

    pub fn backward(&mut self) {
        // Initial estimates
        let ms = self.mf_values[self.n_time - 1];
        let Ps = self.Pf_values[self.n_time - 1];
        self.ms_values.push(ms);
        self.Ps_values.push(Ps);

        for i in 1..self.n_time {
            // Retrieve estimates
            let idx = self.n_time - i - 1;

            let mf = self.mf_values[idx];
            let Pf = self.Pf_values[idx];
            let mp = self.mp_values[idx + 1];
            let Pp = self.Pp_values[idx + 1];

            // Smoothing Step
            let G = Pf * self.F / Pp;
            let (ms, Ps) = Smoother::compute(mp, Pp, mf, Pf, ms, Ps, self.F);
            self.ms_values.push(ms);
            self.Ps_values.push(Ps);
        }
    }

    pub fn values(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        (
            self.mp_values.clone(),
            self.Pp_values.clone(),
            self.mf_values.clone(),
            self.Pf_values.clone(),
            self.ms_values.clone(),
            self.Ps_values.clone(),
        )
    }
}