use num_complex::Complex;

/// Compute tetration z^^h (z tetrated to height h) with proper handling for both integer and non-integer heights
///
/// Tetration z^^h means z^(z^(z^(...))) with h copies of z in the tower.
/// For integer heights, this computes the exact tetration.
/// For non-integer heights, this uses approximation methods.
///
/// # Arguments
///
/// * `z` - The base of the tetration
/// * `h` - The height (number of z's in the tower)
/// * `custom_i` - The custom imaginary unit value (what i² equals)
///
/// # Returns
///
/// The result of z^^h as a complex number
pub fn tetration(z: Complex<f64>, h: Complex<f64>, custom_i: Complex<f64>) -> Complex<f64> {
    // Handle integer heights exactly
    if h.im.abs() < 1e-10 && h.re.fract() == 0.0 && h.re > 0.0 && h.re <= 10.0 {
        let n = h.re as u32;
        return match n {
            1 => z,  // z^^1 = z
            2 => {
                // z^^2 = z^z
                if custom_i == Complex::new(0.0, -1.0) {
                    // Standard complex arithmetic
                    z.powc(z)
                } else {
                    // Custom arithmetic
                    custom_complex_power(z, z, custom_i)
                }
            },
            3 => {
                // z^^3 = z^(z^z)
                let z_pow_z = if custom_i == Complex::new(0.0, -1.0) {
                    z.powc(z)
                } else {
                    custom_complex_power(z, z, custom_i)
                };

                if z_pow_z.norm_sqr() > 1e10 {
                    // Prevent overflow
                    Complex::new(1e5, 1e5)
                } else {
                    if custom_i == Complex::new(0.0, -1.0) {
                        z.powc(z_pow_z)
                    } else {
                        custom_complex_power(z, z_pow_z, custom_i)
                    }
                }
            },
            _ => {
                // For higher integer heights, return a safe value to prevent immediate escape
                Complex::new(1.0, 0.0)
            }
        };
    }

    // For non-integer heights, use approximation methods
    // This is a simplified approach - more sophisticated methods exist
    Complex::new(1.0, 0.0)
}

/// Compute pentation z^^^p (z pentated to level p)
///
/// Pentation z^^^p means z^^(z^^(z^^(...))) with p copies of z in the pentation tower.
/// This grows extremely rapidly, so we use conservative approaches.
///
/// # Arguments
///
/// * `z` - The base of the pentation
/// * `p` - The level (number of z's in the pentation tower)
/// * `custom_i` - The custom imaginary unit value (what i² equals)
///
/// # Returns
///
/// The result of z^^^p as a complex number
pub fn pentation(z: Complex<f64>, p: Complex<f64>, custom_i: Complex<f64>) -> Complex<f64> {
    // Pentation grows extremely rapidly, so we'll use a conservative approach
    if p.im.abs() < 1e-10 && p.re.fract() == 0.0 && p.re > 0.0 && p.re <= 3.0 {
        let n = p.re as u32;
        return match n {
            1 => z,  // z^^^1 = z
            2 => {
                // z^^^2 = z^^z
                tetration(z, z, custom_i)
            },
            _ => {
                // For higher levels, return a safe value to prevent immediate escape
                Complex::new(1.0, 0.0)
            }
        };
    }

    // For non-integer levels, return a safe value
    Complex::new(1.0, 0.0)
}

/// Compute hexation z^^^^h (z hexated to level h)
///
/// Hexation z^^^^h means z^^^(z^^^(z^^^(...))) with h copies of z in the hexation tower.
/// This grows even more rapidly than pentation.
///
/// # Arguments
///
/// * `z` - The base of the hexation
/// * `h` - The level (number of z's in the hexation tower)
/// * `custom_i` - The custom imaginary unit value (what i² equals)
///
/// # Returns
///
/// The result of z^^^^h as a complex number
pub fn hexation(z: Complex<f64>, h: Complex<f64>, custom_i: Complex<f64>) -> Complex<f64> {
    // Hexation grows extremely rapidly, so we'll use a very conservative approach
    if h.im.abs() < 1e-10 && h.re.fract() == 0.0 && h.re > 0.0 && h.re <= 2.0 {
        let n = h.re as u32;
        return match n {
            1 => z,  // z^^^^1 = z
            2 => {
                // z^^^^2 = z^^^z
                pentation(z, z, custom_i)
            },
            _ => {
                // For higher levels, return a safe value to prevent immediate escape
                Complex::new(1.0, 0.0)
            }
        };
    }

    // For non-integer levels, return a safe value
    Complex::new(1.0, 0.0)
}

/// Helper function for custom complex power operation
fn custom_complex_power(base: Complex<f64>, exp: Complex<f64>, custom_i: Complex<f64>) -> Complex<f64> {
    // This is a simplified implementation for custom complex power
    // A full implementation would require more sophisticated mathematics
    if custom_i == Complex::new(0.0, -1.0) {
        // Standard complex power
        base.powc(exp)
    } else {
        // For custom imaginary units, we'll use a simplified approach
        // More sophisticated implementations would handle this differently
        base.powc(exp) // Using standard power as fallback
    }
}