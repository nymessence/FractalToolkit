//! # Parameters Module
//!
//! This module contains the FractalParams struct and related functionality
//! for configuring fractal generation parameters.

use num_complex::Complex;
use serde::{Deserialize, Serialize};

/// Parameters for fractal generation with support for custom imaginary units
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalParams {
    /// Coordinate bounds: [x_min, x_max, y_min, y_max]
    pub bounds: [f64; 4],
    /// Output dimensions: [width, height]
    pub dimensions: [u32; 2],
    /// Maximum iterations per pixel
    pub max_iterations: u32,
    /// Starting point for iteration
    pub spawn: Complex<f64>,
    /// Color palette definition
    pub color_palette: Vec<(String, f64)>, // (hex_color, position)
    /// Escape threshold (bailout value)
    pub bailout: f64,
    /// Mathematical formula to use
    pub formula: String,
    /// Custom value for the imaginary unit (i is the square root of this value)
    pub i_sqrt_value: Complex<f64>,
    /// Output filename
    pub output: String,
    /// Enable orbit debugging
    pub debug_orbits: bool,
}

impl FractalParams {
    /// Create default parameters for fractal generation
    pub fn default() -> Self {
        Self {
            bounds: [-2.0, 2.0, -2.0, 2.0],
            dimensions: [512, 512],
            max_iterations: 100,
            spawn: Complex::new(0.0, 0.0),
            color_palette: vec![
                ("#000000".to_string(), 0.0),   // Black
                ("#FF0000".to_string(), 0.33),  // Red
                ("#00FF00".to_string(), 0.66),  // Green
                ("#FFFFFF".to_string(), 1.0),   // White
            ],
            bailout: 4.0,
            formula: "z^2 + c".to_string(),
            i_sqrt_value: Complex::new(-1.0, 0.0), // Standard complex: i² = -1
            output: "output.png".to_string(),
            debug_orbits: false,
        }
    }

    /// Calculate the scale factors based on bounds and dimensions
    pub fn scale_factors(&self) -> (f64, f64) {
        let width_scale = (self.bounds[1] - self.bounds[0]) / self.dimensions[0] as f64;
        let height_scale = (self.bounds[3] - self.bounds[2]) / self.dimensions[1] as f64;
        (width_scale, height_scale)
    }

    /// Convert pixel coordinates to complex plane coordinates
    pub fn pixel_to_complex(&self, x: u32, y: u32) -> Complex<f64> {
        let (width_scale, height_scale) = self.scale_factors();
        let real = self.bounds[0] + (x as f64) * width_scale;
        let imag = self.bounds[2] + (y as f64) * height_scale;
        Complex::new(real, imag)
    }
}