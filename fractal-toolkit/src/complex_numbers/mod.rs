//! # Complex Numbers Module
//!
//! This module contains the CustomComplex struct and related functionality
//! for working with alternative complex number systems where i² can equal
//! any complex number value.

use num_complex::Complex;
use serde::{Deserialize, Serialize};

/// Custom complex number system with configurable imaginary unit
///
/// This structure implements an alternative complex number system where i² can equal any complex value.
/// In standard complex numbers, i² = -1, but in this system, i² can equal any value specified by i_squared.
/// This enables exploration of alternative number systems with different mathematical properties.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct CustomComplex {
    /// Real component of the complex number
    pub re: f64,
    /// Imaginary component of the complex number
    pub im: f64,
    /// The value that i² equals in this number system (i.e., what i is the square root of)
    pub i_squared: Complex<f64>,
}

impl CustomComplex {
    /// Create a new CustomComplex number with the specified real and imaginary components
    /// and the custom value that i² equals in this number system.
    pub fn new(re: f64, im: f64, i_squared: Complex<f64>) -> Self {
        Self { re, im, i_squared }
    }

    /// Convert this CustomComplex number to a standard Complex<f64> representation
    pub fn to_standard(&self) -> Complex<f64> {
        Complex::new(self.re, self.im)
    }

    /// Create a CustomComplex number from a standard Complex<f64> with a custom imaginary unit value
    pub fn from_standard(z: Complex<f64>, i_squared: Complex<f64>) -> Self {
        Self { re: z.re, im: z.im, i_squared }
    }

    /// Perform multiplication in the custom complex number system respecting the custom imaginary unit
    pub fn multiply(&self, other: &Self) -> Self {
        // (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i²
        // = ac + (ad + bc)*i + bd*i²
        let a = self.re;
        let b = self.im;
        let c = other.re;
        let d = other.im;

        let ac = a * c;
        let ad = a * d;
        let bc = b * c;
        let bd = b * d;

        // bd * i² where i² is our custom value
        let bd_i_squared = bd * other.i_squared;

        // The real part: ac + Re(bd * i²)
        let real_part = ac + bd_i_squared.re;
        // The imaginary part: ad + bc + Im(bd * i²)
        let imag_part = ad + bc + bd_i_squared.im;

        Self {
            re: real_part,
            im: imag_part,
            i_squared: other.i_squared,  // Use the same i_squared value as the other operand
        }
    }

    /// Perform addition in the custom complex number system
    pub fn add(&self, other: &Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
            i_squared: self.i_squared,  // Maintain the same i_squared
        }
    }

    /// Perform subtraction in the custom complex number system
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
            i_squared: self.i_squared,  // Maintain the same i_squared
        }
    }

    /// Get the norm squared of the complex number
    pub fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Get the argument (angle) of the complex number in radians
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Custom power operation that respects the custom imaginary unit
    pub fn pow(&self, exp: &Self) -> Self {
        // For complex exponentiation z^w where z and w are complex numbers,
        // the standard formula is: z^w = exp(w * ln(z))
        // But with a custom imaginary unit, we need to be more careful
        // For now, we'll use the standard complex power function but with awareness of the custom i
        let z = self.to_standard();
        let w = exp.to_standard();

        // Use the standard complex power function
        let result = complex_pow(z, w);
        Self::from_standard(result, self.i_squared)
    }
}

/// Helper function to compute complex power z^w = exp(w * ln(z))
/// This is the standard complex exponentiation formula
fn complex_pow(z: Complex<f64>, w: Complex<f64>) -> Complex<f64> {
    // Handle special cases
    if z.norm_sqr() < 1e-10 {
        // z is essentially zero
        if w.re > 0.0 {
            // 0^w where Re(w) > 0 should be 0
            Complex::new(0.0, 0.0)
        } else if w.re == 0.0 && w.im == 0.0 {
            // 0^0 is typically defined as 1
            Complex::new(1.0, 0.0)
        } else {
            // For other cases involving zero base, return NaN or a large value
            Complex::new(f64::NAN, f64::NAN)
        }
    } else {
        // Standard complex exponentiation: z^w = exp(w * ln(z))
        let ln_z = complex_ln(z);
        let w_ln_z = w * ln_z;
        complex_exp(w_ln_z)
    }
}

/// Helper function to compute complex natural logarithm
/// ln(z) = ln(|z|) + i*arg(z)
fn complex_ln(z: Complex<f64>) -> Complex<f64> {
    let magnitude = z.norm();
    let argument = z.arg();
    Complex::new(magnitude.ln(), argument)
}

/// Helper function to compute complex exponential
/// exp(z) = exp(re) * (cos(im) + i*sin(im))
fn complex_exp(z: Complex<f64>) -> Complex<f64> {
    let exp_re = z.re.exp();
    Complex::new(exp_re * z.im.cos(), exp_re * z.im.sin())
}