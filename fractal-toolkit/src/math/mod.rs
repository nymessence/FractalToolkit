use num_complex::Complex;

/// Mathematical utilities for fractal computations
/// Contains helper functions for complex number operations

/// Safely compute the power of a complex number with bounds checking
pub fn safe_complex_power(base: Complex<f64>, exp: Complex<f64>) -> Complex<f64> {
    // Check for potential overflow conditions
    if base.norm_sqr() > 1e10 || exp.norm_sqr() > 1e10 {
        // Return a bounded value to prevent overflow
        return Complex::new(
            base.re.clamp(-1e5, 1e5),
            base.im.clamp(-1e5, 1e5)
        );
    }
    
    // Perform the actual computation
    base.powc(exp)
}

/// Compute the norm with overflow protection
pub fn safe_norm(z: Complex<f64>) -> f64 {
    let norm_sqr = z.norm_sqr();
    if norm_sqr > 1e20 {
        1e10  // Return a large but finite value
    } else {
        norm_sqr.sqrt()
    }
}

/// Check if a complex number is finite (not NaN or infinite)
pub fn is_complex_finite(z: Complex<f64>) -> bool {
    z.re.is_finite() && z.im.is_finite()
}

/// Clamp a complex number to reasonable bounds
pub fn clamp_complex(z: Complex<f64>, max_abs: f64) -> Complex<f64> {
    Complex::new(
        z.re.clamp(-max_abs, max_abs),
        z.im.clamp(-max_abs, max_abs)
    )
}

/// Compute the argument (angle) of a complex number safely
pub fn safe_arg(z: Complex<f64>) -> f64 {
    if z.norm_sqr() < 1e-20 {
        // For very small numbers, return 0 to avoid division by zero
        0.0
    } else {
        z.arg()
    }
}

/// Compute the square root of a complex number with branch cut considerations
pub fn safe_complex_sqrt(z: Complex<f64>) -> Complex<f64> {
    // Use the principal branch of the square root
    z.sqrt()
}

/// Compute the natural logarithm of a complex number safely
pub fn safe_complex_ln(z: Complex<f64>) -> Complex<f64> {
    if z.norm_sqr() < 1e-20 {
        // For very small numbers, return a large negative real part
        Complex::new(f64::NEG_INFINITY, 0.0)
    } else {
        z.ln()
    }
}

/// Compute the exponential of a complex number with overflow protection
pub fn safe_complex_exp(z: Complex<f64>) -> Complex<f64> {
    if z.re > 700.0 {  // Prevent overflow in exp function
        // Return a large value in the direction of the complex number
        let magnitude = 1e10;
        Complex::new(
            magnitude * (z.re / z.norm()).max(0.1),  // Ensure positive growth
            magnitude * (z.im / z.norm()).max(-1.0).min(1.0)  // Bounded imaginary part
        )
    } else {
        z.exp()
    }
}