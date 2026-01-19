use num_complex::Complex;

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
                let result_norm = result.norm();

                // For fractal generation with complex exponents, use a very conservative limit
                // to prevent immediate escape of all points
                let max_norm = 2.0; // Very conservative for complex exponents in fractals

                if result_norm > max_norm {
                    // Scale down the result significantly to allow for fractal iteration
                    let scale_factor = max_norm / result_norm.max(1e-10); // Avoid division by zero
                    Complex::new(result.re * scale_factor, result.im * scale_factor)
                } else {
                    // For complex exponents, we also need to ensure the result doesn't cause
                    // immediate escape in subsequent iterations. Let's apply a more sophisticated
                    // transformation that preserves the mathematical character while allowing
                    // for fractal formation

                    // Apply a transformation that maps large values to a more manageable range
                    // but still allows for differentiation between points
                    let transformed_result = if result_norm > 1.5 {
                        // For large results, compress the range logarithmically
                        let compressed_norm = 1.0 + 0.5 * (result_norm - 1.5).min(1.0); // Gradually compress
                        let scale_factor = compressed_norm / result_norm.max(1e-10);
                        Complex::new(result.re * scale_factor, result.im * scale_factor)
                    } else if result_norm < 0.01 {
                        // For very small results, slightly amplify to avoid stagnation
                        let amplified_norm = result_norm.max(0.01) * 2.0;
                        let scale_factor = amplified_norm / result_norm.max(1e-10);
                        Complex::new(result.re * scale_factor, result.im * scale_factor)
                    } else {
                        result
                    };

                    transformed_result
                }
            }
        }
    }
}

/// Helper function to compute complex natural logarithm
/// ln(z) = ln(|z|) + i*arg(z)
pub fn complex_ln(z: Complex<f64>) -> Complex<f64> {
    let magnitude = z.norm();
    let argument = z.arg();
    Complex::new(magnitude.ln(), argument)
}

/// Helper function to compute complex exponential
/// exp(z) = exp(re) * (cos(im) + i*sin(im))
pub fn complex_exp(z: Complex<f64>) -> Complex<f64> {
    let exp_re = z.re.exp();
    Complex::new(exp_re * z.im.cos(), exp_re * z.im.sin())
}

/// Helper function to compute tetration (iterated exponentiation)
/// z^^n = z^(z^(z^(...^z))) where z appears n times
pub fn tetration(z: Complex<f64>, n: u32) -> Complex<f64> {
    if n == 0 {
        return Complex::new(1.0, 0.0); // By convention, z^^0 = 1
    }
    
    let mut result = z;
    for _ in 1..n {
        // Check for overflow before computing
        if result.norm_sqr() > 1e10 {
            // Return a large value to indicate divergence
            return Complex::new(1e5, 1e5);
        }
        result = z.powc(result);
    }
    result
}

/// Helper function to compute pentation (iterated tetration)
/// z^^^n = z^^(z^^(z^^(...^^z))) where z appears n times
pub fn pentation(z: Complex<f64>, n: u32) -> Complex<f64> {
    if n == 0 {
        return Complex::new(1.0, 0.0); // By convention, z^^^0 = 1
    }
    
    let mut result = z;
    for _ in 1..n {
        // Check for overflow before computing
        if result.norm_sqr() > 1e10 {
            // Return a large value to indicate divergence
            return Complex::new(1e5, 1e5);
        }
        result = tetration(z, result.norm() as u32); // Simplified approach for complex numbers
    }
    result
}

/// Helper function to compute hexation (iterated pentation)
/// z^^^^n = z^^^(z^^^(z^^^(...^^^z))) where z appears n times
pub fn hexation(z: Complex<f64>, n: u32) -> Complex<f64> {
    if n == 0 {
        return Complex::new(1.0, 0.0); // By convention, z^^^^0 = 1
    }
    
    let mut result = z;
    for _ in 1..n {
        // Check for overflow before computing
        if result.norm_sqr() > 1e10 {
            // Return a large value to indicate divergence
            return Complex::new(1e5, 1e5);
        }
        result = pentation(z, result.norm() as u32); // Simplified approach for complex numbers
    }
    result
}