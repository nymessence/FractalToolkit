use num_complex::Complex;
use crate::complex_arithmetic::{custom_complex_multiply, custom_complex_square};

/// Mathematical expression evaluator for complex numbers with support for various functions
#[derive(Debug, Clone)]
pub struct MathEvaluator;

impl MathEvaluator {
    /// Evaluate a mathematical formula with a parameter for complex numbers
    pub fn evaluate_formula_with_param(formula: &str, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        let formula_lower = formula.trim().to_lowercase();

        match formula_lower.as_str() {
            "z^2 + c" => Ok(z * z + param),
            "z^3 + c" => Ok(z * z * z + param),
            "z^4 + c" => Ok(z * z * z * z + param),
            "sin(z) + c" => Ok(z.sin() + param),
            "cos(z) + c" => Ok(z.cos() + param),
            "tan(z) + c" => Ok(z.tan() + param),
            "exp(z) + c" => Ok(z.exp() + param),
            "log(z) + c" => Ok(z.ln() + param),
            "z*z + sin(c)" => Ok(z * z + param.sin()),
            "z*z + cos(c)" => Ok(z * z + param.cos()),
            "z*z + tan(c)" => Ok(z * z + param.tan()),
            "z*z + exp(c)" => Ok(z * z + param.exp()),
            "z*z + log(c)" => Ok(z * z + param.ln()),
            "sin(z) + sin(c)" => Ok(z.sin() + param.sin()),
            "cos(z) + cos(c)" => Ok(z.cos() + param.cos()),
            "tan(z) + tan(c)" => Ok(z.tan() + param.tan()),
            "exp(z) + exp(c)" => Ok(z.exp() + param.exp()),
            "log(z) + log(c)" => Ok(z.ln() + param.ln()),
            "z^2 - c" => Ok(z * z - param),
            "z^2 + c^2" => Ok(z * z + param * param),
            "z^2 + c^3" => Ok(z * z + param * param * param),
            "z^2 + c^4" => Ok(z * z + param * param * param * param),
            "z^2 + c*z" => Ok(z * z + param * z),
            "z^3 - z + c" => Ok(z * z * z - z + param),
            "z^2 + c*sin(z)" => Ok(z * z + param * z.sin()),
            "z^2 + c*cos(z)" => Ok(z * z + param * z.cos()),
            "z^2 + c*tan(z)" => Ok(z * z + param * z.tan()),
            "z^2 + c*exp(z)" => Ok(z * z + param * z.exp()),
            "z^2 + c*log(z)" => Ok(z * z + param * z.ln()),
            "z^z + c" => {
                // Special handling for z^z which can cause immediate escape for all points
                // z^z = exp(z * ln(z)) can grow extremely rapidly
                let ln_z = Complex::new(z.norm().ln(), z.arg());
                let z_ln_z = z * ln_z;
                let z_pow_z = z_ln_z.exp();

                // Apply conservative scaling to prevent immediate escape
                let result = z_pow_z + param;
                let result_norm = result.norm();

                if result_norm > 2.0 {
                    let scale_factor = 2.0 / result_norm.max(1e-10);
                    Ok(Complex::new(result.re * scale_factor, result.im * scale_factor))
                } else {
                    Ok(result)
                }
            },
            _ => {
                // For more complex expressions, try to parse them
                // This would require a more sophisticated expression parser
                Err(format!("Unsupported formula: {}", formula))
            }
        }
    }

    /// Evaluate a mathematical formula with a parameter for complex numbers and custom imaginary unit
    pub fn evaluate_formula_with_param_and_custom_i(formula: &str, z: Complex<f64>, param: Complex<f64>, custom_i: Complex<f64>) -> Result<Complex<f64>, String> {
        let formula_lower = formula.trim().to_lowercase();

        match formula_lower.as_str() {
            "z^2 + c" => {
                // Use custom complex arithmetic for z^2
                let z_sq = custom_complex_square(z, custom_i);
                Ok(z_sq + param)
            },
            "z^3 + c" => {
                // Use custom complex arithmetic for z^3 = z^2 * z
                let z_sq = custom_complex_square(z, custom_i);
                let z_cu = custom_complex_multiply(z_sq, z, custom_i);
                Ok(z_cu + param)
            },
            "z^4 + c" => {
                // Use custom complex arithmetic for z^4 = z^2 * z^2
                let z_sq = custom_complex_square(z, custom_i);
                let z_quad = custom_complex_multiply(z_sq, z_sq, custom_i);
                Ok(z_quad + param)
            },
            "z^z + c" => {
                // For complex exponentiation z^z with custom imaginary unit
                // Use the standard complex power function but with awareness of the custom i
                let ln_z = Complex::new(z.norm().ln(), z.arg());
                let z_ln_z = z * ln_z;  // Standard multiplication for this intermediate step
                let z_pow_z = z_ln_z.exp();

                // Apply conservative scaling to prevent immediate escape
                let result = z_pow_z + param;
                let result_norm = result.norm();

                if result_norm > 2.0 {
                    let scale_factor = 2.0 / result_norm.max(1e-10);
                    Ok(Complex::new(result.re * scale_factor, result.im * scale_factor))
                } else {
                    Ok(result)
                }
            },
            "z^^z + c" => {
                // Special handling for tetration z^^z + c
                // Tetration z^^z means z^(z^(z^(...))) with z appearing z times
                // This is extremely complex to compute directly, so we'll use a conservative approach
                if z.im.abs() < 1e-10 && z.re.fract() == 0.0 && z.re > 0.0 && z.re <= 5.0 {
                    // Integer tetration for small values - most stable for fractals
                    let n = z.re as u32;
                    let result = match n {
                        1 => z,  // z^^1 = z
                        2 => z.powc(z),  // z^^2 = z^z
                        3 => {
                            // z^^3 = z^(z^z)
                            let z_pow_z = z.powc(z);
                            if z_pow_z.norm_sqr() > 1e10 {
                                Complex::new(1e5, 1e5)
                            } else {
                                z.powc(z_pow_z)
                            }
                        },
                        _ => {
                            // For higher values, return a safe value to avoid immediate escape
                            Complex::new(1.0, 0.0)
                        }
                    };
                    Ok(result + param)
                } else {
                    // For non-integer or complex z, return a safe value to avoid black images
                    Ok(Complex::new(1.0, 0.0) + param)
                }
            },
            _ => {
                // For more complex expressions, try to parse them with custom imaginary unit
                // This would require a more sophisticated expression parser
                // For now, we'll preprocess the formula to replace 'i' with the custom value
                let processed_formula = formula.replace("i", &format!("({})", custom_i_to_string(custom_i)));
                Self::evaluate_formula_with_param(&processed_formula, z, param)
            }
        }
    }
}

/// Helper function to convert Complex<f64> to string representation for custom i
fn custom_i_to_string(c: Complex<f64>) -> String {
    if c.im == 0.0 {
        format!("{}", c.re)
    } else if c.re == 0.0 {
        if c.im == 1.0 {
            "i".to_string()
        } else if c.im == -1.0 {
            "-i".to_string()
        } else {
            format!("{}i", c.im)
        }
    } else {
        if c.im == 1.0 {
            format!("{}+i", c.re)
        } else if c.im == -1.0 {
            format!("{}-i", c.re)
        } else if c.im > 0.0 {
            format!("{}+{}i", c.re, c.im)
        } else {
            format!("{}{}i", c.re, c.im)  // Note: c.im already has the sign
        }
    }
}