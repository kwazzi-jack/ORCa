use nalgebra::{DVector, DMatrix};

// Estimator Trait
trait Estimator {
    // Initialise estimator
    fn new(&self);

    // Run estimator
    fn compute(&self);
}

// Main estimators
struct Predictor {
    f: f64,
    q: f64,
}

struct Filter {
    h: f64,
    r: f64,
}

struct Smoother {
    f: f64,
    q: f64,
}

// Prediction Step
impl Estimator for Predictor {
    fn new(&self, f: f64, q: f64) -> Self {
        Predictor {
            f: f,
            q: q,
        }
    }

    fn compute(&self) -> (f64, f64){
        todo!();
    }
}

// Update Step
impl Estimator for Filter {
    fn new(&self) -> Self {
        todo!();
    }

    fn compute(&self) -> (f64, f64){
        todo!();
    }
}

// Smoothing Step
impl Estimator for Smoother {
    fn new(&self) -> Self {
        todo!();
    }

    fn compute(&self) -> (f64, f64){
        todo!();
    }
}


pub struct Calibrator {
    mf0: f64,
    pf0: f64,
    mp: Vec<f64>,
    mf: Vec<f64>,
    ms: Vec<f64>,
    pp: Vec<f64>,
    pf: Vec<f64>,
    ps: Vec<f64>,
    f: f64,
    h: f64,
    q: f64,
    r: f64,
    predictor: Predictor,
    filter: Filter,
    smoother: Smoother,
}

impl Calibrator {
    fn new(&self, mf0: f64, pf0: f64, f: f64, 
            h: f64, sigma_q: f64, sigma_r: f64) {
        
        // Storage
        self.mp = Vec::new();
        self.mf = Vec::new();
        self.ms = Vec::new();
        self.pp = Vec::new();
        self.pf = Vec::new();
        self.ps = Vec::new();

        // Initial
        self.mf0 = mf0;
        self.pf0 = pf0;

        // State Functions
        self.f = f;
        self.h = h;

        // Noise Covariance
        self.q = q.powi(2);
        self.r = sigma_r.powi(2);

        // Estimators
        self.predictor = Predictor::new();
        self.filter = Filter::new();
        self.smoother = Smoother::new();
    }

    fn forward(&self) {
        todo!();
    }

    fn backward(&self) {
        todo!();
    }
}