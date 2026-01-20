use num_complex::Complex;

/// Parameters for fractal generation
#[derive(Debug, Clone)]
pub struct FractalParams {
    pub bounds: [f64; 4],           // [x_min, x_max, y_min, y_max]
    pub max_iterations: u32,
    pub spawn: [f64; 2],            // For Julia sets
    pub bailout: f64,
    pub formula: String,
    pub i_sqrt_value: Complex<f64>, // Custom imaginary unit (i = sqrt of this value), defaults to 0+1i
}

impl FractalParams {
    pub fn new(bounds: [f64; 4], max_iterations: u32, spawn: [f64; 2], bailout: f64, formula: String) -> Self {
        Self {
            bounds,
            max_iterations,
            spawn,
            bailout,
            formula,
            i_sqrt_value: Complex::new(0.0, 1.0), // Default to standard imaginary unit
        }
    }
}