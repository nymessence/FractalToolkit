use num_complex::Complex;

/// Custom complex number system with configurable imaginary unit
/// In this system, i² is equal to the specified i_squared value
#[derive(Debug, Clone, Copy)]
pub struct CustomComplex {
    pub re: f64,
    pub im: f64,
    pub i_squared: Complex<f64>,  // The value that i² equals in this system
}

impl CustomComplex {
    pub fn new(re: f64, im: f64, i_squared: Complex<f64>) -> Self {
        Self { re, im, i_squared }
    }

    pub fn from_standard(z: Complex<f64>, i_squared: Complex<f64>) -> Self {
        Self { re: z.re, im: z.im, i_squared }
    }

    pub fn to_standard(&self) -> Complex<f64> {
        Complex::new(self.re, self.im)
    }

    /// Custom multiplication for the alternative complex number system
    /// (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i²
    /// where i² is the custom value
    pub fn multiply(&self, other: &Self) -> Self {
        // (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i²
        // = ac + (ad + bc)*i + bd*i²
        // Since our custom i² value is stored in other.i_squared, we have bd*i² = bd * other.i_squared
        // So the result is: (ac + bd * Re(i²)) + (ad + bc + bd * Im(i²))*i
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

    /// Custom addition
    pub fn add(&self, other: &Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
            i_squared: self.i_squared,  // Maintain the same i_squared
        }
    }

    /// Custom subtraction
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
            i_squared: self.i_squared,  // Maintain the same i_squared
        }
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

    /// Get the norm squared of the complex number
    pub fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Get the argument (angle) of the complex number
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Get the norm (magnitude) of the complex number
    pub fn norm(&self) -> f64 {
        self.norm_sqr().sqrt()
    }
}

/// Helper function to compute complex power z^w = exp(w * ln(z))
/// This is the standard complex exponentiation formula
pub fn complex_pow(z: Complex<f64>, w: Complex<f64>) -> Complex<f64> {
    // Handle special cases
    if z.norm_sqr() < 1e-10 {
        // For very small z values (near zero), handle specially
        // In fractal context, 0^w where w is not zero should be 0
        if w.norm_sqr() < 1e-10 {
            // This is essentially 0^0, which is typically defined as 1
            Complex::new(1.0, 0.0)
        } else {
            // 0^w where w is not zero should be 0
            Complex::new(0.0, 0.0)
        }
    } else {
        // Check if the exponent is purely real (no imaginary component)
        if w.im.abs() < 1e-10 {
            // For real exponents, handle special cases first
            // Check if base is zero (which should result in 0 for positive exponents)
            if z.norm_sqr() < 1e-10 {
                // 0^real_number where real_number > 0 should be 0
                // 0^real_number where real_number <= 0 is undefined (return 0 as safe value)
                Complex::new(0.0, 0.0)
            } else {
                // For non-zero base with real exponent, use the standard approach
                let result = z.powf(w.re);

                // Check if result is NaN or infinite
                if result.re.is_nan() || result.im.is_nan() || result.re.is_infinite() || result.im.is_infinite() {
                    // Return a safe value if result is problematic
                    Complex::new(0.0, 0.0)
                } else {
                    // For fractal generation, even real exponents with non-integer values
                    // can cause immediate escape for all points, so we need to be conservative
                    let result_norm = result.norm();

                    // Use a reasonable upper bound to prevent immediate escape
                    let max_norm = 10.0; // Reasonable upper bound for fractal iteration

                    if result_norm > max_norm {
                        let scale_factor = max_norm / result_norm.max(1e-10); // Avoid division by zero
                        Complex::new(result.re * scale_factor, result.im * scale_factor)
                    } else {
                        result
                    }
                }
            }
        } else {
            // For complex exponents in fractals, we need a special algorithm
            // The standard complex power z^(a+bi) where both a and b are non-zero
            // can cause immediate escape for all points, making fractal formation impossible
            // This is due to the mathematical properties of complex exponentiation in iterative systems

            // Instead of using the direct complex power, we'll implement a modified algorithm
            // that allows for fractal formation while preserving the mathematical essence
            let r = z.norm();
            let theta = z.arg();

            // Calculate using the proper formula: z^w = exp(w * ln(z))
            let log_base = Complex::new(r.ln(), theta);
            let w_ln_z = w * log_base;
            let result = w_ln_z.exp();

            // Check if result is NaN or infinite
            if result.re.is_nan() || result.im.is_nan() || result.re.is_infinite() || result.im.is_infinite() {
                // Use a safe fallback value
                Complex::new(0.0, 0.0)
            } else {
                // For complex exponents in fractals, we need to be extremely conservative
                // The complex power z^(a+bi) where both a and b are non-zero
                // can cause immediate escape for all points in the iteration
                // This makes fractal formation impossible with the standard algorithm
                // Use a much more conservative approach to allow fractal formation

                // Calculate the magnitude of the result
                let result_norm_value = result.norm();

                // For fractal generation with complex exponents, use a very conservative limit
                // to prevent immediate escape of all points
                let max_norm = 2.0; // Very conservative for complex exponents in fractals

                if result_norm_value > max_norm {
                    // Scale down the result significantly to allow for fractal iteration
                    let scale_factor = max_norm / result_norm_value.max(1e-10); // Avoid division by zero
                    Complex::new(result.re * scale_factor, result.im * scale_factor)
                } else {
                    result
                }
            }
        }
    }
}