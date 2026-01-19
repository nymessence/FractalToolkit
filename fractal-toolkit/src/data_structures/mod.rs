use num_complex::Complex;
use serde::{Deserialize, Serialize};

/// Parameters for standard Mandelbrot and Julia sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalParams {
    pub bounds: [f64; 4],           // [x_min, x_max, y_min, y_max]
    pub max_iterations: u32,
    pub spawn: Complex<f64>,        // For Julia sets
    pub bailout: f64,
    pub formula: String,
    pub i_sqrt_value: Complex<f64>, // Custom imaginary unit (i = sqrt of this value)
}

impl FractalParams {
    pub fn new(bounds: [f64; 4], max_iterations: u32, spawn: [f64; 2], bailout: f64, formula: String) -> Self {
        Self {
            bounds,
            max_iterations,
            spawn: Complex::new(spawn[0], spawn[1]),
            bailout,
            formula,
            i_sqrt_value: Complex::new(0.0, 1.0), // Default to standard i = sqrt(-1)
        }
    }
}

/// Parameters for Buddhabrot rendering with RGB channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddhabrotParams {
    pub bounds: [f64; 4],           // [x_min, x_max, y_min, y_max]
    pub width: u32,
    pub height: u32,
    pub min_iterations: u32,        // Minimum iterations for points to be considered
    pub max_iterations: u32,        // Maximum iterations to check
    pub samples: u64,               // Number of random samples to take
    pub bailout: f64,
    pub formula: String,
    pub channels: BuddhabrotChannels, // RGB channel configurations
    pub i_sqrt_value: Complex<f64>, // Custom imaginary unit (i = sqrt of this value)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddhabrotChannel {
    pub min_iter: u32,
    pub max_iter: u32,
    pub samples: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddhabrotChannels {
    pub red: BuddhabrotChannel,
    pub green: BuddhabrotChannel,
    pub blue: BuddhabrotChannel,
}

impl BuddhabrotParams {
    pub fn new(
        bounds: [f64; 4],
        width: u32,
        height: u32,
        min_iterations: u32,
        max_iterations: u32,
        samples: u64,
        bailout: f64,
        formula: String,
        channels: BuddhabrotChannels,
    ) -> Self {
        Self {
            bounds,
            width,
            height,
            min_iterations,
            max_iterations,
            samples,
            bailout,
            formula,
            channels,
            i_sqrt_value: Complex::new(0.0, 1.0), // Default to standard i = sqrt(-1)
        }
    }
}

/// Parameters for Buddhabrot Julia sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddhabrotJuliaParams {
    pub bounds: [f64; 4],           // [x_min, x_max, y_min, y_max]
    pub width: u32,
    pub height: u32,
    pub min_iterations: u32,        // Minimum iterations for points to be considered
    pub max_iterations: u32,        // Maximum iterations to check
    pub samples: u64,               // Number of random samples to take
    pub bailout: f64,
    pub spawn: Complex<f64>,        // Constant c value for Julia set
    pub formula: String,
    pub channels: BuddhabrotChannels, // RGB channel configurations
    pub i_sqrt_value: Complex<f64>, // Custom imaginary unit (i = sqrt of this value)
}

impl BuddhabrotJuliaParams {
    pub fn new(
        bounds: [f64; 4],
        width: u32,
        height: u32,
        min_iterations: u32,
        max_iterations: u32,
        samples: u64,
        bailout: f64,
        spawn: [f64; 2],
        formula: String,
        channels: BuddhabrotChannels,
    ) -> Self {
        Self {
            bounds,
            width,
            height,
            min_iterations,
            max_iterations,
            samples,
            bailout,
            spawn: Complex::new(spawn[0], spawn[1]),
            formula,
            channels,
            i_sqrt_value: Complex::new(0.0, 1.0), // Default to standard i = sqrt(-1)
        }
    }
}

/// Parameters for domain color plots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainColorParams {
    pub bounds: [f64; 4],  // [x_min, x_max, y_min, y_max]
    pub width: u32,
    pub height: u32,
    pub formula: String,
    pub i_sqrt_value: Complex<f64>, // Custom imaginary unit (i = sqrt of this value)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorStop {
    pub color: String,  // Hex color code like "#FF0000"
    pub position: f64,  // Position in the gradient (0.0 to 1.0)
}