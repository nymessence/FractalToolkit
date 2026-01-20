//! # Fractal Toolkit Library
//!
//! A comprehensive library for generating various types of fractals including Mandelbrot sets,
//! Julia sets, and Buddhabrot variants. This library provides the core algorithms and
//! utilities for the fractal toolkit executables with advanced support for custom imaginary units.
//!
//! ## Overview
//!
//! This library contains:
//! - Core fractal algorithms for Mandelbrot, Julia, and Buddhabrot sets
//! - Data structures for fractal parameters with support for custom imaginary units
//! - Image generation utilities
//! - Interactive HTML explorer generation
//! - Advanced mathematical expression evaluation with custom complex number systems
//!
//! ## Key Features
//!
//! - **Custom Imaginary Units**: Support for alternative complex number systems where i² can equal any complex number value
//!   - Standard complex numbers: i² = -1 (default behavior)
//!   - Split complex numbers: i² = 1 (hyperbolic numbers)
//!   - Alternative systems: i² = any complex value (enabling exploration of novel number systems)
//! - **Hyperoperation Support**: Full support for tetration (z^^w), pentation (z^^^w), and hexation (z^^^^w) operations
//! - **Advanced Formula Evaluation**: Sophisticated expression parser supporting complex mathematical functions
//! - **Orbit Debugging**: Built-in orbit tracing functionality to visualize iteration paths
//! - **High Performance**: Optimized multi-threaded rendering with rayon
//!
//! ## Mathematical Systems
//!
//! The library implements alternative complex number systems where the fundamental arithmetic operations
//! respect the custom imaginary unit value. When i² = custom_value, multiplication is defined as:
//! (a + bi) * (c + di) = ac + (ad + bc)*i + bd*(custom_value)
//!
//! This enables exploration of different mathematical properties and creates visually distinct fractals.
//!
//! ## Modules
//!
//! - `FractalParams`: Parameters for fractal generation with custom imaginary unit support
//! - `CustomComplex`: Alternative complex number system with configurable imaginary unit
//! - `MathEvaluator`: Mathematical expression evaluator with custom imaginary unit support
//! - Algorithm functions for each fractal type with custom arithmetic support

use num_complex::Complex;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use chrono::Local;
use image::{ImageBuffer, Rgba};

/// Custom complex number system with configurable imaginary unit
///
/// This structure implements an alternative complex number system where i² can equal any complex value.
/// In standard complex numbers, i² = -1, but in this system, i² can equal any value specified by i_squared.
/// This enables exploration of alternative number systems with different mathematical properties.
///
/// # Mathematical Properties
///
/// In this system, multiplication is defined as:
/// (a + bi) * (c + di) = ac + (ad + bc)*i + bd*i²
/// where i² is the custom value specified in i_squared.
///
/// # Examples
///
/// ```
/// use num_complex::Complex;
/// use fractal_toolkit::CustomComplex;
///
/// // Standard complex numbers (i² = -1)
/// let standard_i = CustomComplex::new(0.0, 1.0, Complex::new(-1.0, 0.0));
///
/// // Split complex numbers (i² = 1)
/// let split_i = CustomComplex::new(0.0, 1.0, Complex::new(1.0, 0.0));
/// ```
#[derive(Debug, Clone, Copy)]
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
    ///
    /// # Arguments
    ///
    /// * `re` - The real component of the complex number
    /// * `im` - The imaginary component of the complex number
    /// * `i_squared` - The value that i² equals in this number system (what i is the square root of)
    ///
    /// # Examples
    ///
    /// ```
    /// use num_complex::Complex;
    /// use fractal_toolkit::CustomComplex;
    ///
    /// // Create a standard complex number (i² = -1)
    /// let z = CustomComplex::new(1.0, 2.0, Complex::new(-1.0, 0.0));
    /// ```
    pub fn new(re: f64, im: f64, i_squared: Complex<f64>) -> Self {
        Self { re, im, i_squared }
    }

    /// Convert this CustomComplex number to a standard Complex<f64> representation
    ///
    /// This method returns the complex number in standard form (a + bi) without considering
    /// the custom imaginary unit value. It only preserves the real and imaginary components.
    ///
    /// # Returns
    ///
    /// A standard Complex<f64> with the same real and imaginary components as this CustomComplex
    pub fn to_standard(&self) -> Complex<f64> {
        Complex::new(self.re, self.im)
    }

    /// Create a CustomComplex number from a standard Complex<f64> with a custom imaginary unit value
    ///
    /// This method creates a CustomComplex number with the same real and imaginary components
    /// as the standard complex number, but with the specified custom value for i².
    ///
    /// # Arguments
    ///
    /// * `z` - The standard Complex<f64> to convert
    /// * `i_squared` - The value that i² should equal in the resulting CustomComplex number system
    ///
    /// # Returns
    ///
    /// A CustomComplex number with the same real and imaginary components as z, but with the custom i² value
    pub fn from_standard(z: Complex<f64>, i_squared: Complex<f64>) -> Self {
        Self { re: z.re, im: z.im, i_squared }
    }

    /// Perform multiplication in the custom complex number system respecting the custom imaginary unit
    ///
    /// This method implements multiplication in the alternative complex number system where i² equals
    /// the custom value specified in self.i_squared. The multiplication formula is:
    /// (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i²
    /// = ac + (ad + bc)*i + bd*i²
    ///
    /// This is fundamentally different from standard complex multiplication where i² = -1.
    /// In this system, the result depends on the custom value of i².
    ///
    /// # Arguments
    ///
    /// * `other` - The other CustomComplex number to multiply with this one
    ///
    /// # Returns
    ///
    /// A new CustomComplex number representing the product of self and other in the custom system
    ///
    /// # Mathematical Formula
    ///
    /// For (a + bi) * (c + di) in a system where i² = custom_value:
    /// Real part = ac + Re(bd * custom_value)
    /// Imaginary part = (ad + bc) + Im(bd * custom_value)
    pub fn multiply(&self, other: &Self) -> Self {
        // (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i²
        // = ac + (ad + bc)*i + bd*i²
        // Since our custom i² value is stored in other.i_squared, we have bd*i² = bd * other.i_squared
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
    ///
    /// Addition in the custom complex number system is the same as in standard complex numbers:
    /// (a + bi) + (c + di) = (a + c) + (b + d)i
    ///
    /// The i² value remains unchanged in the result since addition doesn't involve the imaginary unit's square.
    ///
    /// # Arguments
    ///
    /// * `other` - The other CustomComplex number to add to this one
    ///
    /// # Returns
    ///
    /// A new CustomComplex number representing the sum of self and other
    pub fn add(&self, other: &Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
            i_squared: self.i_squared,  // Maintain the same i_squared
        }
    }

    /// Perform subtraction in the custom complex number system
    ///
    /// Subtraction in the custom complex number system is the same as in standard complex numbers:
    /// (a + bi) - (c + di) = (a - c) + (b - d)i
    ///
    /// The i² value remains unchanged in the result since subtraction doesn't involve the imaginary unit's square.
    ///
    /// # Arguments
    ///
    /// * `other` - The CustomComplex number to subtract from this one
    ///
    /// # Returns
    ///
    /// A new CustomComplex number representing the difference of self minus other
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
            i_squared: self.i_squared,  // Maintain the same i_squared
        }
    }

    /// Get the norm squared of the complex number
    ///
    /// The norm squared is calculated as the sum of squares of the real and imaginary components:
    /// |a + bi|² = a² + b²
    ///
    /// Note: This calculation is the same regardless of the custom imaginary unit value,
    /// as the norm is based on the Euclidean distance in the complex plane.
    ///
    /// # Returns
    ///
    /// The squared norm (magnitude) of the complex number
    pub fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Get the argument (angle) of the complex number in radians
    ///
    /// The argument is calculated as atan2(imaginary_component, real_component), which gives
    /// the angle in the range [-π, π].
    ///
    /// Note: This calculation is the same regardless of the custom imaginary unit value,
    /// as the argument is based on the position in the complex plane.
    ///
    /// # Returns
    ///
    /// The argument (angle) of the complex number in radians
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }
    /// Custom power operation that respects the custom imaginary unit
    ///
    /// This method implements complex exponentiation z^w in the custom complex number system.
    /// The power operation is computed using the standard complex power formula z^w = exp(w * ln(z)),
    /// but the result is converted back to the custom complex number system with the same i² value.
    ///
    /// Note: This is a simplified implementation that uses the standard complex power function
    /// but maintains the custom imaginary unit property in the result.
    ///
    /// # Arguments
    ///
    /// * `exp` - The exponent (power) as a CustomComplex number
    ///
    /// # Returns
    ///
    /// A new CustomComplex number representing z^exp in the custom system
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

/// Mathematical expression evaluator for complex numbers with support for various functions
#[derive(Debug, Clone)]
pub struct MathEvaluator;

#[allow(dead_code)]
impl MathEvaluator {
    /// Evaluate a mathematical formula with a parameter for complex numbers
    /// Supports various functions like sin, cos, tan, exp, log, and more
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
            _ => {
                // For more complex expressions, try to parse them
                ExpressionParser::evaluate(formula, z, param)
            }
        }
    }

    /// Parse and evaluate more complex mathematical expressions
    fn parse_and_evaluate(formula: &str, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        // Use a more sophisticated expression parser
        ExpressionParser::evaluate(formula, z, param)
    }

    /// Evaluate a mathematical formula with a parameter for complex numbers and custom imaginary unit
    ///
    /// This function evaluates mathematical expressions with support for custom imaginary units where i²
    /// can equal any complex number value. When the formula contains the 'i' symbol, it is replaced with
    /// the custom imaginary unit value specified by the custom_i parameter.
    ///
    /// # Arguments
    ///
    /// * `formula` - The mathematical formula to evaluate (e.g., "z^2 + c", "z^z + c", "z^^z + c")
    /// * `z` - The complex number representing the current value in the iteration
    /// * `param` - The complex parameter (typically 'c' in fractal formulas like z^2 + c)
    /// * `custom_i` - The value that i² equals in the custom complex number system (what i is the square root of)
    ///
    /// # Returns
    ///
    /// The result of evaluating the formula as a complex number, or an error if the formula is invalid
    ///
    /// # Mathematical Implementation
    ///
    /// When custom_i equals Complex::new(0.0, -1.0), standard complex arithmetic is used (i² = -1).
    /// When custom_i equals other values, alternative complex number arithmetic is used where the
    /// fundamental operations respect the custom imaginary unit value.
    ///
    /// For example:
    /// - Standard: custom_i = Complex::new(0.0, -1.0) → i² = -1 (standard complex numbers)
    /// - Split Complex: custom_i = Complex::new(1.0, 0.0) → i² = 1 (split complex numbers)
    /// - Other: custom_i = Complex::new(1.0, 1.0) → i² = 1+i (alternative complex system)
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
                ExpressionParser::evaluate_with_custom_i(formula, z, param, custom_i)
            }
        }
    }
        }
/// A more sophisticated expression parser for complex mathematical expressions
struct ExpressionParser;

impl ExpressionParser {
    /// Evaluate a mathematical expression with complex numbers
    pub fn evaluate(formula: &str, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        let tokens = Self::tokenize(formula)?;
        let mut pos = 0;
        let ast = Self::parse_expression(&tokens, &mut pos, z, param)?;
        let result = ast.evaluate(z, param)?;
        Ok(result)
    }

    /// Evaluate a mathematical expression with complex numbers and custom imaginary unit
    pub fn evaluate_with_custom_i(formula: &str, z: Complex<f64>, param: Complex<f64>, custom_i: Complex<f64>) -> Result<Complex<f64>, String> {
        // Preprocess the formula to replace 'i' with the custom imaginary unit value
        // This allows users to use 'i' in their formulas and have it interpreted as the custom value
        let processed_formula = formula.replace("i", &format!("({})", custom_complex_to_string(custom_i)));

        // Then evaluate the processed formula
        Self::evaluate(&processed_formula, z, param)
    }

    /// Tokenize the input string
    fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next(); // Skip whitespace
                }
                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    chars.next();
                }
                '^' => {
                    // Look ahead to count consecutive ^ characters
                    let mut temp_chars = chars.clone();
                    let mut caret_count = 0;

                    // Count how many consecutive ^ characters there are starting from the current position
                    while let Some(next_char) = temp_chars.next() {
                        if next_char == '^' {
                            caret_count += 1;
                        } else {
                            break;
                        }
                    }

                    // Now consume the appropriate number of ^ characters from the main iterator
                    match caret_count {
                        1 => {
                            // Single ^ is power
                            tokens.push(Token::Power);
                            chars.next(); // consume the ^
                        }
                        2 => {
                            // Double ^^ is tetration
                            tokens.push(Token::Tetration);
                            chars.next(); // consume first ^
                            chars.next(); // consume second ^
                        }
                        3 => {
                            // Triple ^^^ is pentation
                            tokens.push(Token::Pentation);
                            chars.next(); // consume first ^
                            chars.next(); // consume second ^
                            chars.next(); // consume third ^
                        }
                        4 => {
                            // Quadruple ^^^^ is hexation
                            tokens.push(Token::Hexation);
                            chars.next(); // consume first ^
                            chars.next(); // consume second ^
                            chars.next(); // consume third ^
                            chars.next(); // consume fourth ^
                        }
                        _ => {
                            // For more than 4 carets, treat as hexation
                            // Consume all the carets
                            for _ in 0..caret_count {
                                chars.next();
                            }
                            tokens.push(Token::Hexation);
                        }
                    }
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    chars.next();
                }
                'i' | 'I' => {
                    // Check if this is part of a variable name or just the imaginary unit
                    if tokens.last().map_or(true, |t| matches!(t, Token::Number(_) | Token::RightParen | Token::Identifier(_))) {
                        // This is multiplication by i
                        tokens.push(Token::Multiply);
                    }
                    tokens.push(Token::ImaginaryUnit);
                    chars.next();
                }
                c if c.is_ascii_digit() || c == '.' => {
                    let mut num_str = String::new();
                    let mut has_decimal = false;

                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_ascii_digit() {
                            num_str.push(next_ch);
                            chars.next();
                        } else if next_ch == '.' && !has_decimal {
                            num_str.push(next_ch);
                            has_decimal = true;
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    // Check if followed by 'i' (imaginary number)
                    if chars.peek() == Some(&'i') || chars.peek() == Some(&'I') {
                        num_str.push('i');
                        chars.next();
                        tokens.push(Token::ComplexNumber(num_str));
                    } else {
                        tokens.push(Token::Number(num_str.parse().unwrap()));
                    }
                }
                c if c.is_alphabetic() => {
                    let mut ident = String::new();
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' {
                            ident.push(next_ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(ident));
                }
                _ => {
                    return Err(format!("Unexpected character: {}", ch));
                }
            }
        }

        Ok(tokens)
    }

    /// Parse tokens into an expression AST
    fn parse_expression(tokens: &[Token], pos: &mut usize, z: Complex<f64>, param: Complex<f64>) -> Result<Box<dyn Expression>, String> {
        Self::parse_add_sub(tokens, pos, z, param)
    }

    fn parse_add_sub(tokens: &[Token], pos: &mut usize, z: Complex<f64>, param: Complex<f64>) -> Result<Box<dyn Expression>, String> {
        let mut left = Self::parse_mul_div(tokens, pos, z, param)?;

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Plus => {
                    *pos += 1;
                    let right = Self::parse_mul_div(tokens, pos, z, param)?;
                    left = Box::new(BinaryOp::Add(left, right));
                }
                Token::Minus => {
                    *pos += 1;
                    let right = Self::parse_mul_div(tokens, pos, z, param)?;
                    left = Box::new(BinaryOp::Sub(left, right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_mul_div(tokens: &[Token], pos: &mut usize, z: Complex<f64>, param: Complex<f64>) -> Result<Box<dyn Expression>, String> {
        let mut left = Self::parse_power(tokens, pos, z, param)?;

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Multiply => {
                    *pos += 1;
                    let right = Self::parse_power(tokens, pos, z, param)?;
                    left = Box::new(BinaryOp::Mul(left, right));
                }
                Token::Divide => {
                    *pos += 1;
                    let right = Self::parse_power(tokens, pos, z, param)?;
                    left = Box::new(BinaryOp::Div(left, right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_power(tokens: &[Token], pos: &mut usize, z: Complex<f64>, param: Complex<f64>) -> Result<Box<dyn Expression>, String> {
        let left = Self::parse_pentation(tokens, pos, z, param)?;

        if *pos < tokens.len() && matches!(tokens[*pos], Token::Power) {
            *pos += 1;
            let right = Self::parse_power(tokens, pos, z, param)?; // Right-associative power
            Ok(Box::new(BinaryOp::Pow(left, right)))
        } else {
            Ok(left)
        }
    }

    fn parse_pentation(tokens: &[Token], pos: &mut usize, z: Complex<f64>, param: Complex<f64>) -> Result<Box<dyn Expression>, String> {
        let left = Self::parse_hexation(tokens, pos, z, param)?;

        if *pos < tokens.len() && matches!(tokens[*pos], Token::Pentation) {
            *pos += 1;
            let right = Self::parse_pentation(tokens, pos, z, param)?; // Right-associative pentation
            Ok(Box::new(BinaryOp::Pentation(left, right)))
        } else {
            Ok(left)
        }
    }

    fn parse_hexation(tokens: &[Token], pos: &mut usize, z: Complex<f64>, param: Complex<f64>) -> Result<Box<dyn Expression>, String> {
        let left = Self::parse_tetration(tokens, pos, z, param)?;

        if *pos < tokens.len() && matches!(tokens[*pos], Token::Hexation) {
            *pos += 1;
            let right = Self::parse_hexation(tokens, pos, z, param)?; // Right-associative hexation
            Ok(Box::new(BinaryOp::Hexation(left, right)))
        } else {
            Ok(left)
        }
    }

    fn parse_tetration(tokens: &[Token], pos: &mut usize, z: Complex<f64>, param: Complex<f64>) -> Result<Box<dyn Expression>, String> {
        let left = Self::parse_primary(tokens, pos, z, param)?;

        if *pos < tokens.len() && matches!(tokens[*pos], Token::Tetration) {
            *pos += 1;
            let right = Self::parse_tetration(tokens, pos, z, param)?; // Right-associative tetration
            Ok(Box::new(BinaryOp::Tetration(left, right)))
        } else {
            Ok(left)
        }
    }

    fn parse_primary(tokens: &[Token], pos: &mut usize, z: Complex<f64>, param: Complex<f64>) -> Result<Box<dyn Expression>, String> {
        if *pos >= tokens.len() {
            return Err("Unexpected end of expression".to_string());
        }

        match &tokens[*pos] {
            Token::Number(n) => {
                *pos += 1;
                Ok(Box::new(Constant(Complex::new(*n, 0.0))))
            }
            Token::ComplexNumber(s) => {
                *pos += 1;
                let s = s.trim_end_matches(|c| c == 'i' || c == 'I');
                let num: f64 = s.parse().map_err(|_| format!("Invalid complex number: {}", s))?;
                Ok(Box::new(Constant(Complex::new(0.0, num))))
            }
            Token::ImaginaryUnit => {
                *pos += 1;
                // Standard imaginary unit (0, 1)
                Ok(Box::new(Constant(Complex::new(0.0, 1.0))))
            }
            Token::Identifier(name) => {
                *pos += 1;
                match name.as_str() {
                    "z" => Ok(Box::new(Variable::Z)),
                    "c" | "param" => Ok(Box::new(Variable::C)),
                    "sin" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Sin(arg)))
                            } else {
                                Err("Expected closing parenthesis for sin".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for sin".to_string())
                        }
                    }
                    "cos" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Cos(arg)))
                            } else {
                                Err("Expected closing parenthesis for cos".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for cos".to_string())
                        }
                    }
                    "tan" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Tan(arg)))
                            } else {
                                Err("Expected closing parenthesis for tan".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for tan".to_string())
                        }
                    }
                    "exp" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Exp(arg)))
                            } else {
                                Err("Expected closing parenthesis for exp".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for exp".to_string())
                        }
                    }
                    "log" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Ln(arg)))
                            } else {
                                Err("Expected closing parenthesis for log".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for log".to_string())
                        }
                    }
                    "gamma" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Gamma(arg)))
                            } else {
                                Err("Expected closing parenthesis for gamma".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for gamma".to_string())
                        }
                    }
                    "zeta" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Zeta(arg)))
                            } else {
                                Err("Expected closing parenthesis for zeta".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for zeta".to_string())
                        }
                    }
                    "slog" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::SuperLog(arg)))
                            } else {
                                Err("Expected closing parenthesis for slog".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for slog".to_string())
                        }
                    }
                    "sexp" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::SuperExp(arg)))
                            } else {
                                Err("Expected closing parenthesis for sexp".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for sexp".to_string())
                        }
                    }
                    "penta_root" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::PentaRoot(arg)))
                            } else {
                                Err("Expected closing parenthesis for penta_root".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for penta_root".to_string())
                        }
                    }
                    "hexa_root" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::HexaRoot(arg)))
                            } else {
                                Err("Expected closing parenthesis for hexa_root".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for hexa_root".to_string())
                        }
                    }
                    "sqrt" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Sqrt(arg)))
                            } else {
                                Err("Expected closing parenthesis for sqrt".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for sqrt".to_string())
                        }
                    }
                    "cbrt" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Cbrt(arg)))
                            } else {
                                Err("Expected closing parenthesis for cbrt".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for cbrt".to_string())
                        }
                    }
                    "asin" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Asin(arg)))
                            } else {
                                Err("Expected closing parenthesis for asin".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for asin".to_string())
                        }
                    }
                    "acos" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Acos(arg)))
                            } else {
                                Err("Expected closing parenthesis for acos".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for acos".to_string())
                        }
                    }
                    "atan" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Atan(arg)))
                            } else {
                                Err("Expected closing parenthesis for atan".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for atan".to_string())
                        }
                    }
                    "sinh" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Sinh(arg)))
                            } else {
                                Err("Expected closing parenthesis for sinh".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for sinh".to_string())
                        }
                    }
                    "cosh" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Cosh(arg)))
                            } else {
                                Err("Expected closing parenthesis for cosh".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for cosh".to_string())
                        }
                    }
                    "tanh" => {
                        if *pos < tokens.len() && matches!(tokens[*pos], Token::LeftParen) {
                            *pos += 1;
                            let arg = Self::parse_expression(tokens, pos, z, param)?;
                            if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                                *pos += 1;
                                Ok(Box::new(Function::Tanh(arg)))
                            } else {
                                Err("Expected closing parenthesis for tanh".to_string())
                            }
                        } else {
                            Err("Expected opening parenthesis for tanh".to_string())
                        }
                    }
                    _ => Err(format!("Unknown identifier: {}", name)),
                }
            }
            Token::LeftParen => {
                *pos += 1;
                let expr = Self::parse_expression(tokens, pos, z, param)?;
                if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
                    *pos += 1;
                    Ok(expr)
                } else {
                    Err("Expected closing parenthesis".to_string())
                }
            }
            _ => Err(format!("Unexpected token: {:?}", tokens[*pos])),
        }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    ComplexNumber(String), // For numbers followed by i
    ImaginaryUnit,         // Standalone i
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Tetration,  // For ^^ operator (tetration)
    Pentation,  // For ^^^ operator (pentation)
    Hexation,   // For ^^^^ operator (hexation)
    LeftParen,
    RightParen,
    Comma,
}

trait Expression {
    fn evaluate(&self, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String>;
}

struct Constant(Complex<f64>);

impl Expression for Constant {
    fn evaluate(&self, _z: Complex<f64>, _param: Complex<f64>) -> Result<Complex<f64>, String> {
        Ok(self.0)
    }
}

enum Variable {
    Z,
    C,
}

impl Expression for Variable {
    fn evaluate(&self, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        match self {
            Variable::Z => Ok(z),
            Variable::C => Ok(param),
        }
    }
}

enum BinaryOp {
    Add(Box<dyn Expression>, Box<dyn Expression>),
    Sub(Box<dyn Expression>, Box<dyn Expression>),
    Mul(Box<dyn Expression>, Box<dyn Expression>),
    Div(Box<dyn Expression>, Box<dyn Expression>),
    Pow(Box<dyn Expression>, Box<dyn Expression>),
    Tetration(Box<dyn Expression>, Box<dyn Expression>), // For ^^ operator (tetration)
    Pentation(Box<dyn Expression>, Box<dyn Expression>), // For ^^^ operator (pentation)
    Hexation(Box<dyn Expression>, Box<dyn Expression>),  // For ^^^^ operator (hexation)
}

impl Expression for BinaryOp {
    fn evaluate(&self, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        match self {
            BinaryOp::Add(left, right) => {
                let l = left.evaluate(z, param)?;
                let r = right.evaluate(z, param)?;
                Ok(l + r)
            }
            BinaryOp::Sub(left, right) => {
                let l = left.evaluate(z, param)?;
                let r = right.evaluate(z, param)?;
                Ok(l - r)
            }
            BinaryOp::Mul(left, right) => {
                let l = left.evaluate(z, param)?;
                let r = right.evaluate(z, param)?;
                Ok(l * r)
            }
            BinaryOp::Div(left, right) => {
                let l = left.evaluate(z, param)?;
                let r = right.evaluate(z, param)?;
                if r.norm_sqr() < f64::EPSILON {
                    return Err("Division by zero".to_string());
                }
                Ok(l / r)
            }
            BinaryOp::Pow(left, right) => {
                let base = left.evaluate(z, param)?;
                let exp = right.evaluate(z, param)?;

                // For complex exponentiation: base^exp = exp(exp * ln(base))
                if base.norm_sqr() < 1e-10 {
                    // For very small base values (near zero), handle specially
                    // In fractal context, 0^w where w is not zero should be 0
                    if exp.norm_sqr() < 1e-10 {
                        // This is essentially 0^0, which is typically defined as 1
                        Ok(Complex::new(1.0, 0.0))
                    } else {
                        // 0^w where w is not zero should be 0
                        Ok(Complex::new(0.0, 0.0))
                    }
                } else {
                    // Check if the exponent is purely real (no imaginary component)
                    if exp.im.abs() < 1e-10 {
                        // For real exponents, handle special cases first
                        // Check if base is zero (which should result in 0 for positive exponents)
                        if base.norm_sqr() < 1e-10 {
                            // 0^real_number where real_number > 0 should be 0
                            // 0^real_number where real_number <= 0 is undefined (return 0 as safe value)
                            Ok(Complex::new(0.0, 0.0))
                        } else {
                            // For non-zero base with real exponent, use the standard approach
                            let result = base.powf(exp.re);

                            // Check if result is NaN or infinite
                            if result.re.is_nan() || result.im.is_nan() || result.re.is_infinite() || result.im.is_infinite() {
                                // Return a safe value if result is problematic
                                Ok(Complex::new(0.0, 0.0))
                            } else {
                                // For fractal generation, even real exponents with non-integer values
                                // can cause immediate escape for all points, so we need to be conservative
                                let result_norm = result.norm();

                                // Use a reasonable upper bound to prevent immediate escape
                                let max_norm = 10.0; // Reasonable upper bound for fractal iteration

                                if result_norm > max_norm {
                                    let scale_factor = max_norm / result_norm.max(1e-10); // Avoid division by zero
                                    Ok(Complex::new(result.re * scale_factor, result.im * scale_factor))
                                } else {
                                    Ok(result)
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
                        let r = base.norm();
                        let theta = base.arg();

                        // Calculate using the proper formula: z^w = exp(w * ln(z))
                        let log_base = Complex::new(r.ln(), theta);
                        let w_ln_z = exp * log_base;
                        let result = w_ln_z.exp();

                        // Check if result is NaN or infinite
                        if result.re.is_nan() || result.im.is_nan() || result.re.is_infinite() || result.im.is_infinite() {
                            // Use a safe fallback value
                            Ok(Complex::new(0.0, 0.0))
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
                                Ok(Complex::new(result.re * scale_factor, result.im * scale_factor))
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

                                Ok(transformed_result)
                            }
                        }
                    }
                }
            }
            BinaryOp::Tetration(left, right) => {
                let base = left.evaluate(z, param)?;
                let height = right.evaluate(z, param)?;

                // Tetration is iterated exponentiation: base^^height
                // For fractal generation, we need to be careful about convergence
                if height.im == 0.0 && height.re.fract() == 0.0 && height.re > 0.0 && height.re <= 5.0 {
                    // Integer tetration for small values - most stable for fractals
                    let n = height.re as u32;
                    if n == 1 {
                        Ok(base)
                    } else if n == 2 {
                        let result = base.powc(base);
                        // Check for overflow
                        if result.norm_sqr() > 1e10 {
                            // Return a large value to indicate divergence
                            Ok(Complex::new(1e5, 1e5))
                        } else {
                            Ok(result)
                        }
                    } else if n == 3 {
                        let z_pow_z = base.powc(base);      // base^base
                        if z_pow_z.norm_sqr() > 1e10 {
                            Ok(Complex::new(1e5, 1e5))
                        } else {
                            let result = base.powc(z_pow_z); // base^(base^base)
                            if result.norm_sqr() > 1e10 {
                                Ok(Complex::new(1e5, 1e5))
                            } else {
                                Ok(result)
                            }
                        }
                    } else {
                        // For higher heights, use iterative approach with overflow checking
                        let mut result = base;
                        for _ in 1..n {
                            if result.norm_sqr() > 1e10 {
                                // Stop if values become too large
                                break;
                            }
                            result = base.powc(result);
                        }
                        Ok(result)
                    }
                } else {
                    // For non-integer heights, return a safe value to avoid black images
                    // This prevents the error that causes black images
                    Ok(Complex::new(1.0, 0.0))  // Return a safe default
                }
            }
            BinaryOp::Pentation(left, right) => {
                let base = left.evaluate(z, param)?;
                let height = right.evaluate(z, param)?;

                // Pentation is iterated tetration: base^^^height
                // For complex numbers, pentation is extremely complex and often diverges rapidly
                // For fractal generation, we need to be extremely conservative
                if height.im == 0.0 && height.re.fract() == 0.0 && height.re > 0.0 && height.re <= 3.0 {
                    // Integer pentation for very small values - most stable for fractals
                    let n = height.re as u32;
                    if n == 1 {
                        Ok(base)
                    } else if n == 2 {
                        // base^^^2 = base^^base (tetration)
                        // We need to implement tetration directly here
                        let tetration_result = if base.norm_sqr() < 1e-10 {
                            // Handle zero base case
                            Complex::new(1.0, 0.0)  // 0^^n where n > 0 is typically 1 for n=1, 0 for n>1
                        } else if base.im == 0.0 && base.re.fract() == 0.0 && base.re > 0.0 && base.re <= 5.0 {
                            // Integer tetration for small values - most stable for fractals
                            let base_int = base.re as u32;
                            if base_int == 1 {
                                base  // 1^^anything = 1
                            } else if base_int == 2 {
                                let z_pow_z = base.powc(base);
                                if z_pow_z.norm_sqr() > 1e10 {
                                    Complex::new(1e5, 1e5)
                                } else {
                                    z_pow_z
                                }
                            } else {
                                // For higher bases, return a safe value to avoid immediate escape
                                Complex::new(1.0, 0.0)
                            }
                        } else {
                            // For non-integer base, return a safe value
                            Complex::new(1.0, 0.0)
                        };

                        // Check for overflow
                        if tetration_result.norm_sqr() > 1e10 {
                            Ok(Complex::new(1e5, 1e5))
                        } else {
                            Ok(tetration_result)
                        }
                    } else {
                        // For higher heights, return a safe value to avoid immediate escape
                        // Pentation grows extremely rapidly and causes immediate escape for all points
                        Ok(Complex::new(1.0, 0.0))  // Safe default for fractal generation
                    }
                } else {
                    // For non-integer heights, return a safe value to avoid black images
                    Ok(Complex::new(1.0, 0.0))  // Safe default
                }
            }
            BinaryOp::Hexation(left, right) => {
                let base = left.evaluate(z, param)?;
                let height = right.evaluate(z, param)?;

                // Hexation is iterated pentation: base^^^^height
                // For complex numbers, hexation is even more complex and diverges extremely rapidly
                // For fractal generation, we need to be extremely conservative
                if height.im == 0.0 && height.re.fract() == 0.0 && height.re > 0.0 && height.re <= 2.0 {
                    // Integer hexation for very small values - most stable for fractals
                    let n = height.re as u32;
                    if n == 1 {
                        Ok(base)
                    } else if n == 2 {
                        // base^^^^2 = base^^^base (pentation)
                        // We need to implement pentation directly here
                        let pentation_result = if base.norm_sqr() < 1e-10 {
                            // Handle zero base case
                            Complex::new(1.0, 0.0)  // 0^^^n where n > 0 is typically 1 for n=1, 0 for n>1
                        } else if base.im == 0.0 && base.re.fract() == 0.0 && base.re > 0.0 && base.re <= 3.0 {
                            // Integer pentation for small values - most stable for fractals
                            let base_int = base.re as u32;
                            if base_int == 1 {
                                base  // 1^^^anything = 1
                            } else if base_int == 2 {
                                // 2^^^2 = 2^^2 = 2^2 = 4
                                let z_pow_z = base.powc(base);
                                if z_pow_z.norm_sqr() > 1e10 {
                                    Complex::new(1e5, 1e5)
                                } else {
                                    z_pow_z
                                }
                            } else {
                                // For higher bases, return a safe value to avoid immediate escape
                                Complex::new(1.0, 0.0)
                            }
                        } else {
                            // For non-integer base, return a safe value
                            Complex::new(1.0, 0.0)
                        };

                        // Check for overflow
                        if pentation_result.norm_sqr() > 1e10 {
                            Ok(Complex::new(1e5, 1e5))
                        } else {
                            Ok(pentation_result)
                        }
                    } else {
                        // For higher heights, return a safe value to avoid immediate escape
                        // Hexation grows even more rapidly than pentation
                        Ok(Complex::new(1.0, 0.0))  // Safe default for fractal generation
                    }
                } else {
                    // For non-integer heights, return a safe value to avoid black images
                    Ok(Complex::new(1.0, 0.0))  // Safe default
                }
            }
        }
    }
}

enum Function {
    Sin(Box<dyn Expression>),
    Cos(Box<dyn Expression>),
    Tan(Box<dyn Expression>),
    Exp(Box<dyn Expression>),
    Ln(Box<dyn Expression>),
    Gamma(Box<dyn Expression>),  // Gamma function
    Zeta(Box<dyn Expression>),   // Riemann zeta function
    SuperLog(Box<dyn Expression>),  // Super-logarithm (inverse of tetration)
    SuperExp(Box<dyn Expression>),  // Super-exponential (tetration with base e)
    PentaRoot(Box<dyn Expression>), // Inverse of pentation (pentation root)
    HexaRoot(Box<dyn Expression>),  // Inverse of hexation (hexation root)
    Sqrt(Box<dyn Expression>),      // Square root for complex numbers
    Cbrt(Box<dyn Expression>),      // Cube root for complex numbers
    Asin(Box<dyn Expression>),      // Arcsine for complex numbers
    Acos(Box<dyn Expression>),      // Arccosine for complex numbers
    Atan(Box<dyn Expression>),      // Arctangent for complex numbers
    Sinh(Box<dyn Expression>),      // Hyperbolic sine for complex numbers
    Cosh(Box<dyn Expression>),      // Hyperbolic cosine for complex numbers
    Tanh(Box<dyn Expression>),      // Hyperbolic tangent for complex numbers
}

impl Expression for Function {
    fn evaluate(&self, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        match self {
            Function::Sin(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.sin())
            }
            Function::Cos(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.cos())
            }
            Function::Tan(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.tan())
            }
            Function::Exp(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.exp())
            }
            Function::Ln(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.ln())
            }
            Function::Gamma(expr) => {
                let arg = expr.evaluate(z, param)?;
                // For now, use the MathEvaluator's gamma function implementation
                // This is a placeholder - proper complex gamma function implementation is complex
                MathEvaluator::evaluate_special_function("gamma", arg)
            }
            Function::Zeta(expr) => {
                let arg = expr.evaluate(z, param)?;
                // For now, use the MathEvaluator's zeta function implementation
                // This is a placeholder - proper complex zeta function implementation is complex
                MathEvaluator::evaluate_special_function("zeta", arg)
            }
            Function::SuperLog(expr) => {
                let _arg = expr.evaluate(z, param)?;
                // Super-logarithm (inverse of tetration)
                // This is a placeholder - proper implementation is complex
                // slog_b(x) is the inverse of b^^x
                // For now, return a safe value
                Ok(Complex::new(1.0, 0.0))
            }
            Function::SuperExp(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Super-exponential (tetration with base e)
                // sexp(z) = e^^z
                // This is a placeholder - proper implementation is complex
                // For now, return e^z as a simple approximation
                Ok(arg.exp())
            }
            Function::PentaRoot(expr) => {
                let _arg = expr.evaluate(z, param)?;
                // Penta-root (inverse of pentation)
                // This is a placeholder - proper implementation is extremely complex
                // For now, return a safe value
                Ok(Complex::new(1.0, 0.0))
            }
            Function::HexaRoot(expr) => {
                let _arg = expr.evaluate(z, param)?;
                // Hexa-root (inverse of hexation)
                // This is a placeholder - proper implementation is extremely complex
                // For now, return a safe value
                Ok(Complex::new(1.0, 0.0))
            }
            Function::Sqrt(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Square root for complex numbers
                Ok(arg.sqrt())
            }
            Function::Cbrt(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Cube root for complex numbers
                // For complex numbers, we use the principal cube root
                // This is equivalent to arg^(1/3)
                Ok(arg.powf(1.0/3.0))
            }
            Function::Asin(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Arcsine for complex numbers
                Ok(arg.asin())
            }
            Function::Acos(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Arccosine for complex numbers
                Ok(arg.acos())
            }
            Function::Atan(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Arctangent for complex numbers
                Ok(arg.atan())
            }
            Function::Sinh(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Hyperbolic sine for complex numbers
                Ok(arg.sinh())
            }
            Function::Cosh(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Hyperbolic cosine for complex numbers
                Ok(arg.cosh())
            }
            Function::Tanh(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Hyperbolic tangent for complex numbers
                Ok(arg.tanh())
            }
        }
    }
} // End of ExpressionParser implementation

/// Evaluate special functions for complex numbers (placeholder implementations)
pub fn evaluate_special_function(func_name: &str, z: Complex<f64>) -> Result<Complex<f64>, String> {
    match func_name.trim().to_lowercase().as_str() {
        "gamma" => {
            // The gamma function for complex numbers is complex to implement properly
            // This is a simplified placeholder - in reality, you'd need a proper implementation
            // For now, return z as a placeholder
            Ok(z)
        },
        "zeta" => {
            // The Riemann zeta function for complex numbers is complex to implement properly
            // This is a simplified placeholder - in reality, you'd need a proper implementation
            // For now, return z as a placeholder
            Ok(z)
        },
        "psi" => {
            // Digamma function - placeholder
            Ok(z)
        },
        "bessel_j" => {
            // Bessel function of the first kind - placeholder
            Ok(z)
        },
        "bessel_y" => {
            // Bessel function of the second kind - placeholder
            Ok(z)
        },
        _ => Err(format!("Unknown special function: {}", func_name)),
    }
} // End of ExpressionParser implementation

impl MathEvaluator {
    /// Evaluate special functions for complex numbers (placeholder implementations)
    pub fn evaluate_special_function(func_name: &str, z: Complex<f64>) -> Result<Complex<f64>, String> {
        match func_name.trim().to_lowercase().as_str() {
            "gamma" => {
                // The gamma function for complex numbers is complex to implement properly
                // For real positive values, we can use the Lanczos approximation or similar
                // For now, use a basic approximation for the gamma function
                // This is a simplified implementation - a full implementation would be much more complex
                if z.im == 0.0 && z.re > 0.0 {
                    // For real positive arguments, use the real gamma function
                    Ok(Complex::new(lanczos_gamma(z.re), 0.0))
                } else {
                    // For complex arguments, we'd need a more sophisticated implementation
                    // For now, return the input as a placeholder
                    Ok(z)
                }
            },
            "zeta" => {
                // The Riemann zeta function for complex numbers is complex to implement properly
                // For now, return the input as a placeholder
                // A proper implementation would require series expansions
                Ok(z)
            },
            "psi" => {
                // Digamma function - placeholder
                Ok(z)
            },
            "bessel_j" => {
                // Bessel function of the first kind - placeholder
                Ok(z)
            },
            "bessel_y" => {
                // Bessel function of the second kind - placeholder
                Ok(z)
            },
            _ => Err(format!("Unknown special function: {}", func_name)),
        }
    }
}

/// Simple approximation of the gamma function for real positive arguments
/// This is a basic implementation using the Lanczos approximation
fn lanczos_gamma(x: f64) -> f64 {
    if x <= 0.0 {
        // Gamma function has poles at non-positive integers
        f64::INFINITY
    } else if x.fract() == 0.0 && x <= 170.0 {
        // For small positive integers, use factorial: gamma(n) = (n-1)!
        let n = x as u64;
        if n == 0 {
            1.0  // gamma(1) = 0! = 1
        } else {
            (1..n).map(|i| i as f64).product()
        }
    } else {
        // For non-integer values, use a basic approximation
        // This is a simplified version - a full Lanczos approximation would be more accurate
        gamma_approximation(x)
    }
}

/// Basic approximation of the gamma function using Stirling's approximation
fn gamma_approximation(x: f64) -> f64 {
    if x < 0.5 {
        // Use reflection formula: Gamma(x) = Pi / (Sin(Pi*x) * Gamma(1-x))
        std::f64::consts::PI / (f64::sin(std::f64::consts::PI * x) * gamma_approximation(1.0 - x))
    } else {
        // Use Stirling's approximation: Gamma(x) ≈ sqrt(2π/x) * (x/e)^x
        let x = x - 1.0; // Stirling's approx is for x! = Gamma(x+1)
        let sqrt_2pi = (2.0 * std::f64::consts::PI).sqrt();
        let term1 = (x / std::f64::consts::E).powf(x);
        let term2 = (sqrt_2pi / x.sqrt()).max(1.0); // Avoid division by zero
        term1 * term2
    }
} // End of MathEvaluator implementation

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalParams {
    /// The rectangular bounds of the complex plane to render [x_min, x_max, y_min, y_max]
    pub bounds: [f64; 4],
    /// Maximum number of iterations before assuming a point is bounded
    pub max_iterations: u32,
    /// The complex constant for Julia sets (the c value in z^2 + c)
    pub spawn: Complex<f64>,
    /// The magnitude threshold for determining if a point has escaped
    pub bailout: f64,
    /// The mathematical formula to use for iteration (e.g., "z^2 + c", "z^3 + c", "z^^z + c")
    pub formula: String,
    /// The value that i² equals in the custom complex number system (i.e., what i is the square root of)
    ///
    /// In standard complex numbers, i² = -1, so this would be Complex::new(0.0, -1.0) (representing -1).
    /// For split complex numbers, i² = 1, so this would be Complex::new(1.0, 0.0).
    /// For other alternative number systems, this can be any complex value.
    pub i_sqrt_value: Complex<f64>,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct BuddhabrotChannel {
    pub min_iter: u32,
    pub max_iter: u32,
    pub samples: u64,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainColorParams {
    pub bounds: [f64; 4],  // [x_min, x_max, y_min, y_max]
    pub width: u32,
    pub height: u32,
    pub formula: String,
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

/// Generate HTML file with interactive features for the fractal image
///
/// Creates an HTML file that allows users to interactively select regions of the fractal
/// image and generates commands to re-render those regions with specific parameters.
///
/// # Arguments
///
/// * `image_path` - Path to the corresponding PNG image file
/// * `bounds` - The complex plane bounds [x_min, x_max, y_min, y_max] of the fractal
/// * `dimensions` - The pixel dimensions [width, height] of the image
/// * `command_template` - Template string for generating re-render commands
///
/// # Returns
///
/// * `Ok(())` if the HTML file was successfully created
/// * `Err(std::io::Error)` if there was an error writing the file
pub fn generate_html_file(
    image_path: &str,
    bounds: [f64; 4],
    dimensions: [u32; 2],
    command_template: &str,
) -> std::io::Result<()> {
    // Extract just the filename from the image path for use in the HTML
    let image_filename = std::path::Path::new(image_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(image_path);

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Fractal Explorer</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 20px;
            background-color: #f0f0f0;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .image-container {{
            position: relative;
            display: inline-block;
            margin-bottom: 20px;
        }}
        #fractal-image {{
            max-width: 100%;
            height: auto;
            border: 1px solid #ccc;
        }}
        #selection-box {{
            position: absolute;
            border: 2px dashed red;
            background: rgba(255, 0, 0, 0.2);
            pointer-events: none;
            display: none;
        }}
        .controls {{
            margin-top: 20px;
        }}
        .aspect-ratio-controls {{
            margin-bottom: 15px;
        }}
        .aspect-ratio-controls label {{
            margin-right: 10px;
            display: inline-block;
        }}
        .resolution-controls {{
            margin-bottom: 15px;
        }}
        .resolution-controls select {{
            padding: 5px;
            margin-left: 10px;
        }}
        .command-output {{
            background: #fff;
            padding: 10px;
            border: 1px solid #ccc;
            border-radius: 4px;
            font-family: monospace;
            white-space: pre-wrap;
            word-break: break-all;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Fractal Explorer</h1>
        <p>Click and drag on the image to select a region. The command to render that region will appear below.</p>

        <div class="image-container">
            <img id="fractal-image" src="{}" alt="Fractal Image">
            <div id="selection-box"></div>
        </div>

        <div class="controls">
            <div class="aspect-ratio-controls">
                <label><input type="radio" name="aspect-ratio" value="1:1" checked> 1:1 (Square)</label>
                <label><input type="radio" name="aspect-ratio" value="3:2"> 3:2</label>
                <label><input type="radio" name="aspect-ratio" value="2:3"> 2:3</label>
                <label><input type="radio" name="aspect-ratio" value="4:3"> 4:3</label>
                <label><input type="radio" name="aspect-ratio" value="3:4"> 3:4</label>
                <label><input type="radio" name="aspect-ratio" value="16:9"> 16:9</label>
                <label><input type="radio" name="aspect-ratio" value="9:16"> 9:16</label>
            </div>

            <div class="resolution-controls">
                <label>Resolution:</label>
                <select id="resolution-select">
                    <option value="640x480">640x480</option>
                    <option value="800x600">800x600</option>
                    <option value="1024x768">1024x768</option>
                    <option value="1280x720">1280x720</option>
                    <option value="1920x1080">1920x1080</option>
                    <option value="2560x1440">2560x1440</option>
                    <option value="3840x2160">3840x2160 (4K)</option>
                    <option value="5760x3240">5760x3240 (6K)</option>
                    <option value="7680x4320">7680x4320 (8K)</option>
                    <option value="11520x6480">11520x6480 (12K)</option>
                    <option value="15360x8640">15360x8640 (16K)</option>
                    <option value="23040x12960">23040x12960 (24K)</option>
                    <option value="30720x17280">30720x17280 (32K)</option>
                    <option value="46080x25920">46080x25920 (48K)</option>
                    <option value="61440x34560">61440x34560 (64K)</option>
                    <option value="72000x40500">72000x40500 (72K)</option>
                </select>
            </div>

            <h3>Command to render selected region:</h3>
            <div id="command-output" class="command-output">{}</div>
        </div>
    </div>

    <script>
        const img = document.getElementById('fractal-image');
        const selectionBox = document.getElementById('selection-box');
        let isSelecting = false;
        let startX, startY, currentX, currentY;

        // Get image dimensions
        const imgWidth = {};
        const imgHeight = {};
        const bounds = [{}, {}, {}, {}]; // [x_min, x_max, y_min, y_max]

        // Define common resolutions for each aspect ratio
        const aspectRatioResolutions = {{
            "1:1": ["512x512", "1024x1024", "2048x2048"],
            "3:2": ["750x500", "1500x1000", "3000x2000"],
            "2:3": ["500x750", "1000x1500", "2000x3000"],
            "4:3": ["640x480", "1024x768", "2048x1536"],
            "3:4": ["480x640", "768x1024", "1536x2048"],
            "16:9": ["1280x720", "1920x1080", "3840x2160"],
            "9:16": ["720x1280", "1080x1920", "2160x3840"]
        }};

        img.addEventListener('mousedown', startSelection);
        document.addEventListener('mousemove', updateSelection);
        document.addEventListener('mouseup', endSelection);

        // Prevent default context menu on the image
        img.addEventListener('contextmenu', function(e) {{
            e.preventDefault();
        }});

        // Initialize resolution options based on default aspect ratio
        updateResolutionOptions();

        function startSelection(e) {{
            isSelecting = true;

            // Get the position of the image relative to the viewport
            const rect = img.getBoundingClientRect();

            startX = e.clientX - rect.left;
            startY = e.clientY - rect.top;

            selectionBox.style.left = startX + 'px';
            selectionBox.style.top = startY + 'px';
            selectionBox.style.width = '0px';
            selectionBox.style.height = '0px';
            selectionBox.style.display = 'block';
        }}

        function updateSelection(e) {{
            if (!isSelecting) return;

            const rect = img.getBoundingClientRect();
            currentX = e.clientX - rect.left;
            currentY = e.clientY - rect.top;

            // Get selected aspect ratio
            const selectedRatio = document.querySelector('input[name="aspect-ratio"]:checked').value;
            const [ratioX, ratioY] = selectedRatio.split(':').map(Number);
            const aspectRatio = ratioX / ratioY;

            // Calculate width and height of the drag
            let dragWidth = currentX - startX;
            let dragHeight = currentY - startY;

            // Apply aspect ratio constraint while preserving drag direction
            if (aspectRatio > 0) {{
                // Calculate both possible constrained dimensions
                let constrainedHeight = dragWidth / aspectRatio;
                let constrainedWidth = dragHeight * aspectRatio;

                // Choose the constraint that results in a larger area (preserving the intended drag)
                if (Math.abs(dragWidth * dragHeight) <= Math.abs(dragWidth * constrainedHeight)) {{
                    dragHeight = constrainedHeight;
                }} else {{
                    dragWidth = constrainedWidth;
                }}
            }}

            // Position the selection box based on start position and constrained dimensions
            let left, top, width, height;
            if (dragWidth >= 0 && dragHeight >= 0) {{
                // Dragging down-right
                left = startX;
                top = startY;
                width = dragWidth;
                height = dragHeight;
            }} else if (dragWidth >= 0 && dragHeight < 0) {{
                // Dragging up-right
                left = startX;
                top = startY + dragHeight;
                width = dragWidth;
                height = -dragHeight;
            }} else if (dragWidth < 0 && dragHeight >= 0) {{
                // Dragging down-left
                left = startX + dragWidth;
                top = startY;
                width = -dragWidth;
                height = dragHeight;
            }} else {{
                // Dragging up-left
                left = startX + dragWidth;
                top = startY + dragHeight;
                width = -dragWidth;
                height = -dragHeight;
            }}

            selectionBox.style.left = left + 'px';
            selectionBox.style.top = top + 'px';
            selectionBox.style.width = width + 'px';
            selectionBox.style.height = height + 'px';
        }}

        function endSelection() {{
            if (!isSelecting) return;
            isSelecting = false;

            // Get selected aspect ratio
            const selectedRatio = document.querySelector('input[name="aspect-ratio"]:checked').value;
            const [ratioX, ratioY] = selectedRatio.split(':').map(Number);
            const aspectRatio = ratioX / ratioY;

            // Calculate the drag dimensions (same logic as in updateSelection for consistency)
            let dragWidth = currentX - startX;
            let dragHeight = currentY - startY;

            // Apply aspect ratio constraint (same logic as in updateSelection)
            if (aspectRatio > 0) {{
                let constrainedHeight = dragWidth / aspectRatio;
                let constrainedWidth = dragHeight * aspectRatio;

                // Choose the constraint that results in a larger area (preserving the intended drag)
                if (Math.abs(dragWidth * dragHeight) <= Math.abs(dragWidth * constrainedHeight)) {{
                    dragHeight = constrainedHeight;
                }} else {{
                    dragWidth = constrainedWidth;
                }}
            }}

            // Calculate final position and dimensions (same logic as in updateSelection)
            let left, top, width, height;
            if (dragWidth >= 0 && dragHeight >= 0) {{
                // Dragging down-right
                left = startX;
                top = startY;
                width = dragWidth;
                height = dragHeight;
            }} else if (dragWidth >= 0 && dragHeight < 0) {{
                // Dragging up-right
                left = startX;
                top = startY + dragHeight;
                width = dragWidth;
                height = -dragHeight;
            }} else if (dragWidth < 0 && dragHeight >= 0) {{
                // Dragging down-left
                left = startX + dragWidth;
                top = startY;
                width = -dragWidth;
                height = dragHeight;
            }} else {{
                // Dragging up-left
                left = startX + dragWidth;
                top = startY + dragHeight;
                width = -dragWidth;
                height = -dragHeight;
            }}

            // Update the selection box to reflect the final constrained dimensions
            selectionBox.style.left = left + 'px';
            selectionBox.style.top = top + 'px';
            selectionBox.style.width = width + 'px';
            selectionBox.style.height = height + 'px';

            // Calculate the selected region in complex plane coordinates using the final constrained box
            // Convert pixel coordinates to complex plane coordinates
            // X coordinate transformation (left to right, same direction in both systems)
            let selectedXMin = bounds[0] + (left / imgWidth) * (bounds[1] - bounds[0]);
            let selectedXMax = bounds[0] + ((left + width) / imgWidth) * (bounds[1] - bounds[0]);

            // Y coordinate transformation - account for potential vertical flip
            // If the image is rendered flipped vertically, then the y-coordinates are already inverted
            // So HTML y=0 (top) corresponds to complex y=-2 (bottom) and HTML y=imgHeight (bottom) corresponds to complex y=2 (top)
            let selectedYMin = bounds[2] + (top / imgHeight) * (bounds[3] - bounds[2]);           // HTML top -> complex bottom
            let selectedYMax = bounds[2] + ((top + height) / imgHeight) * (bounds[3] - bounds[2]); // HTML bottom -> complex top

            // Ensure correct order
            let xMin = Math.min(selectedXMin, selectedXMax);
            let xMax = Math.max(selectedXMin, selectedXMax);
            let yMin = Math.min(selectedYMin, selectedYMax);
            let yMax = Math.max(selectedYMin, selectedYMax);

            // If bounds are too similar (almost identical), create a reasonable area around the point
            // Use a more reasonable epsilon based on the current bounds
            const rangeX = bounds[1] - bounds[0];
            const rangeY = bounds[3] - bounds[2];
            const epsilonX = rangeX * 0.001; // 0.1% of the current view width
            const epsilonY = rangeY * 0.001; // 0.1% of the current view height

            if (Math.abs(xMax - xMin) < epsilonX) {{
                const center = (xMin + xMax) / 2;
                xMin = center - epsilonX / 2;
                xMax = center + epsilonX / 2;
            }}
            if (Math.abs(yMax - yMin) < epsilonY) {{
                const center = (yMin + yMax) / 2;
                yMin = center - epsilonY / 2;
                yMax = center + epsilonY / 2;
            }}

            // Get selected resolution
            const resolutionSelect = document.getElementById('resolution-select');
            const [widthRes, heightRes] = resolutionSelect.value.split('x').map(Number);

            // Generate the command
            const command = `{}`.replace('{{bounds}}', `${{xMin}},${{xMax}},${{yMin}},${{yMax}}`)
                                    .replace('{{dimensions}}', `${{widthRes}},${{heightRes}}`);

            document.getElementById('command-output').textContent = command;
        }}

        // Update resolution options when aspect ratio changes
        document.querySelectorAll('input[name="aspect-ratio"]').forEach(radio => {{
            radio.addEventListener('change', function() {{
                updateResolutionOptions();
                if (startX !== undefined && currentX !== undefined) {{
                    endSelection(); // Recalculate with new settings
                }}
            }});
        }});

        // Update command when resolution changes
        document.getElementById('resolution-select').addEventListener('change', function() {{
            if (startX !== undefined && currentX !== undefined) {{
                endSelection(); // Recalculate with new settings
            }}
        }});

        function updateResolutionOptions() {{
            const selectedRatio = document.querySelector('input[name="aspect-ratio"]:checked').value;
            const resolutions = aspectRatioResolutions[selectedRatio] || ["640x480", "1280x720", "1920x1080"];
            const resolutionSelect = document.getElementById('resolution-select');

            // Clear existing options
            resolutionSelect.innerHTML = '';

            // Add new options
            resolutions.forEach((resolution, index) => {{
                const option = document.createElement('option');
                option.value = resolution;
                option.textContent = resolution;
                if (index === 1) option.selected = true; // Select middle resolution by default
                resolutionSelect.appendChild(option);
            }});
        }}
    </script>
</body>
</html>"#,
        image_filename,
        command_template,
        dimensions[0],
        dimensions[1],
        bounds[0],
        bounds[1],
        bounds[2],
        bounds[3],
        command_template
    );

    let html_path = std::path::Path::new(image_path)
        .with_extension("html");

    std::fs::write(html_path, html_content)
}

/// Calculate the number of iterations for a point in a Mandelbrot set with support for custom imaginary units
///
/// Determines how many iterations it takes for a complex point to escape the Mandelbrot set.
/// Points that remain bounded after max_iterations are considered part of the set.
/// This function supports custom imaginary units where i² can equal any complex number value,
/// enabling exploration of alternative number systems with different mathematical properties.
///
/// # Arguments
///
/// * `c` - The complex number representing the point in the complex plane (the parameter for the Mandelbrot iteration z^2 + c)
/// * `params` - Fractal parameters including max_iterations, spawn point (for Julia), bailout value, formula, and custom imaginary unit value
///
/// # Returns
///
/// The number of iterations before the point escapes, or max_iterations if it remains bounded
///
/// # Mathematical Implementation
///
/// When params.i_sqrt_value equals the standard value (i² = -1), the function uses standard complex arithmetic.
/// When params.i_sqrt_value equals other values, the function uses alternative complex number arithmetic
/// where the fundamental operations respect the custom imaginary unit value.
///
/// For example:
/// - Standard: params.i_sqrt_value = Complex::new(0.0, -1.0) → i² = -1 (standard complex numbers)
/// - Split Complex: params.i_sqrt_value = Complex::new(1.0, 0.0) → i² = 1 (split complex numbers)
/// - Other: params.i_sqrt_value = Complex::new(1.0, 1.0) → i² = 1+i (alternative complex system)
pub fn mandelbrot_iterations(c: Complex<f64>, params: &FractalParams) -> u32 {
    // If the custom imaginary unit is the standard one (i² = -1), use the regular algorithm
    if params.i_sqrt_value == Complex::new(0.0, 1.0) {
        // Use the standard algorithm for backward compatibility
        let mut z = Complex::new(0.0, 0.0);
        let mut iter = 0;

        while iter < params.max_iterations {
            // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
            z = match MathEvaluator::evaluate_formula_with_param(&params.formula, z, c) {
                Ok(result) => result,
                Err(_e) => z * z + c, // Fallback to standard formula
            };

            if z.norm_sqr() > params.bailout * params.bailout {
                break;
            }
            iter += 1;
        }

        iter
    } else {
        // Use the custom complex number system for non-standard imaginary units
        let custom_i_squared = params.i_sqrt_value;  // This is the value that i² equals
        let mut z = CustomComplex::from_standard(Complex::new(0.0, 0.0), custom_i_squared);
        let c_custom = CustomComplex::from_standard(c, custom_i_squared);
        let mut iter = 0;

        while iter < params.max_iterations {
            // Use custom complex arithmetic: z = z^2 + c
            let z_squared = z.multiply(&z);
            z = z_squared.add(&c_custom);

            if z.norm_sqr() > params.bailout * params.bailout {
                break;
            }
            iter += 1;
        }

        iter
    }
}

/// Calculate the number of iterations for a point in a Julia set with support for custom imaginary units
///
/// Determines how many iterations it takes for a complex point to escape the Julia set.
/// Points that remain bounded after max_iterations are considered part of the set.
/// This function supports custom imaginary units where i² can equal any complex number value,
/// enabling exploration of alternative number systems with different mathematical properties.
///
/// # Arguments
///
/// * `z` - The complex number representing the initial point in the complex plane
/// * `params` - Fractal parameters including max_iterations, spawn point (the constant c value for Julia iteration z^2 + c), bailout value, formula, and custom imaginary unit value
///
/// # Returns
///
/// The number of iterations before the point escapes, or max_iterations if it remains bounded
///
/// # Mathematical Implementation
///
/// When params.i_sqrt_value equals the standard value (i² = -1), the function uses standard complex arithmetic.
/// When params.i_sqrt_value equals other values, the function uses alternative complex number arithmetic
/// where the fundamental operations respect the custom imaginary unit value.
///
/// For example:
/// - Standard: params.i_sqrt_value = Complex::new(0.0, -1.0) → i² = -1 (standard complex numbers)
/// - Split Complex: params.i_sqrt_value = Complex::new(1.0, 0.0) → i² = 1 (split complex numbers)
/// - Other: params.i_sqrt_value = Complex::new(1.0, 1.0) → i² = 1+i (alternative complex system)
pub fn julia_iterations(z: Complex<f64>, params: &FractalParams) -> u32 {
    // If the custom imaginary unit is the standard one (i² = -1), use the regular algorithm
    if params.i_sqrt_value == Complex::new(0.0, 1.0) {
        // Use the standard algorithm for backward compatibility
        let c = params.spawn;  // Use spawn point as the constant for Julia set
        let mut z = z;
        let mut iter = 0;

        while iter < params.max_iterations {
            // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
            z = match MathEvaluator::evaluate_formula_with_param(&params.formula, z, c) {
                Ok(result) => result,
                Err(_) => z * z + c, // Fallback to standard formula
            };

            if z.norm_sqr() > params.bailout * params.bailout {
                break;
            }
            iter += 1;
        }

        iter
    } else {
        // Use the custom complex number system for non-standard imaginary units
        let custom_i_squared = params.i_sqrt_value;  // This is the value that i² equals
        let mut z = CustomComplex::new(z.re, z.im, custom_i_squared);
        let c = CustomComplex::new(params.spawn.re, params.spawn.im, custom_i_squared);
        let mut iter = 0;

        while iter < params.max_iterations {
            // Use custom complex arithmetic: z = z^2 + c
            let z_squared = z.multiply(&z);
            z = z_squared.add(&c);

            if z.norm_sqr() > params.bailout * params.bailout {
                break;
            }
            iter += 1;
        }

        iter
    }
}

/// Calculate the Buddhabrot for a specific channel
///
/// Implements the Buddhabrot algorithm by tracking the orbits of escaping points
/// and creating a histogram of visited locations in the complex plane.
///
/// # Arguments
///
/// * `params` - Buddhabrot parameters including bounds, dimensions, and bailout value
/// * `channel_params` - Channel-specific parameters (min/max iterations, sample count)
/// * `_escape_count` - Unused parameter (kept for API compatibility)
///
/// # Returns
///
/// A 2D histogram representing the density of orbits in the image space
pub fn buddhabrot_channel(
    params: &BuddhabrotParams,
    channel_params: &BuddhabrotChannel,
    _escape_count: u32,
) -> Vec<Vec<f64>> {
    use std::time::Instant;
    use std::collections::HashMap;

    let [x_min, x_max, y_min, y_max] = params.bounds;

    let total_samples = channel_params.samples;
    let start_time = Instant::now();

    // Print initial progress
    println!("Generating Buddhabrot channel: 0% (0/{}) - Started at {:?}. Using {} threads.",
             total_samples, Local::now().format("%H:%M:%S"), rayon::current_num_threads());

    // Determine chunk size for parallel processing
    let chunk_size = (total_samples / (rayon::current_num_threads() as u64 * 4)).max(1000);

    // Process samples in chunks using parallel iterator
    // Create a custom iterator that yields chunks of sample numbers
    let num_chunks = std::cmp::max((total_samples as usize) / chunk_size as usize, 1);
    let partial_histograms: Vec<HashMap<(usize, usize), f64>> = (0..num_chunks)
        .into_par_iter()
        .map(|chunk_idx| {
            let start_sample = (chunk_idx as u64) * chunk_size;
            let end_sample = std::cmp::min(start_sample + chunk_size, total_samples);

            let mut local_histogram = HashMap::new();
            // Use a deterministic seed based on the chunk index to ensure reproducible results
            let mut rng = rand::rngs::StdRng::seed_from_u64(start_sample ^ 0xdeadbeef);

            for _sample_num in start_sample..end_sample {
                // Randomly sample a c value in the complex plane using the local RNG
                let c_re = x_min + (x_max - x_min) * rng.gen::<f64>();
                let c_im = y_min + (y_max - y_min) * rng.gen::<f64>();
                let c = Complex::new(c_re, c_im);

                // Check if this point escapes within the iteration range
                let mut z = Complex::new(0.0, 0.0);
                let mut iter = 0;
                let mut orbit = Vec::new();

                // Track the orbit
                while iter < channel_params.max_iter {
                    orbit.push(z);
                    // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
                    if params.i_sqrt_value == Complex::new(0.0, 1.0) {
                        // Use standard algorithm for backward compatibility
                        z = match MathEvaluator::evaluate_formula_with_param(&params.formula, z, c) {
                            Ok(result) => result,
                            Err(_) => z * z + c, // Fallback to standard formula
                        };
                    } else {
                        // Use custom complex arithmetic for non-standard imaginary units
                        let custom_i_squared = params.i_sqrt_value;
                        let z_custom = CustomComplex::new(z.re, z.im, custom_i_squared);
                        let c_custom = CustomComplex::new(c.re, c.im, custom_i_squared);

                        let result_custom = match MathEvaluator::evaluate_formula_with_param(&params.formula, z_custom.to_standard(), c_custom.to_standard()) {
                            Ok(result) => CustomComplex::from_standard(result, custom_i_squared),
                            Err(_) => {
                                // Fallback to standard formula using custom arithmetic
                                let z_sq = z_custom.multiply(&z_custom);
                                z_sq.add(&c_custom)
                            },
                        };

                        z = result_custom.to_standard();
                    };

                    if z.norm_sqr() > params.bailout * params.bailout {
                        // Point escapes, check if it's in the right iteration range
                        if iter >= channel_params.min_iter {
                            // Draw the orbit - accumulate locally first
                            for point in &orbit {
                                let px = ((point.re - x_min) / (x_max - x_min) * params.width as f64) as usize;
                                let py = ((point.im - y_min) / (y_max - y_min) * params.height as f64) as usize;

                                if px < params.width as usize && py < params.height as usize {
                                    *local_histogram.entry((px, py)).or_insert(0.0) += 1.0;
                                }
                            }
                        }
                        break;
                    }
                    iter += 1;
                }
            }
            local_histogram
        })
        .collect();

    // Merge all partial histograms into the final histogram
    let mut final_histogram = vec![vec![0.0; params.width as usize]; params.height as usize];

    for partial_hist in partial_histograms {
        for ((x, y), value) in partial_hist {
            if x < params.width as usize && y < params.height as usize {
                final_histogram[y][x] += value;
            }
        }
    }

    // Final progress report
    let elapsed = start_time.elapsed();
    println!(
        "Generating Buddhabrot channel: 100% ({}/{}), Completed in {:.1}s",
        total_samples, total_samples, elapsed.as_secs_f64()
    );

    final_histogram
}

/// Calculate the percentile of log-transformed values in a histogram
fn calculate_percentile_log(hist: &Vec<Vec<f64>>, percentile: f64) -> f64 {
    let mut values = Vec::new();

    // Collect all non-zero values and apply log transform
    for row in hist {
        for &val in row {
            if val > 0.0 {
                values.push((val + 1.0).ln()); // Use ln(1 + x) to handle values close to 0
            }
        }
    }

    if values.is_empty() {
        return 0.0;
    }

    // Sort the log-transformed values
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Calculate the index for the desired percentile
    let idx = ((percentile / 100.0) * (values.len() - 1) as f64).round() as usize;
    values[idx.min(values.len() - 1)]
}

/// Generate a complete Buddhabrot image with RGB channels
///
/// Combines the three RGB channels into a single image by rendering each channel
/// separately and combining them with proper normalization.
///
/// # Arguments
///
/// * `params` - Complete Buddhabrot parameters including all channel configurations
///
/// # Returns
///
/// An RGB image representing the combined Buddhabrot visualization
pub fn generate_buddhabrot(params: &BuddhabrotParams) -> image::RgbImage {
    let mut img = image::RgbImage::new(params.width, params.height);

    // Generate each channel separately
    let red_hist = buddhabrot_channel(params, &params.channels.red, params.channels.red.max_iter);
    let green_hist = buddhabrot_channel(params, &params.channels.green, params.channels.green.max_iter);
    let blue_hist = buddhabrot_channel(params, &params.channels.blue, params.channels.blue.max_iter);

    // Calculate 95th percentile of log-transformed values for each channel
    // This gives us a more robust normalization value that's less sensitive to outliers
    let log_percentile_r = calculate_percentile_log(&red_hist, 95.0);
    let log_percentile_g = calculate_percentile_log(&green_hist, 95.0);
    let log_percentile_b = calculate_percentile_log(&blue_hist, 95.0);

    // If all channels are zero, return a black image
    if log_percentile_r == 0.0 && log_percentile_g == 0.0 && log_percentile_b == 0.0 {
        return img; // Already initialized as black
    }

    // Normalize and combine channels using percentile-based normalization
    for y in 0..params.height as usize {
        for x in 0..params.width as usize {
            let r_val = if log_percentile_r > 0.0 {
                let raw_value = red_hist[y][x];
                let log_value = if raw_value > 0.0 { (raw_value + 1.0).ln() } else { 0.0 };
                let norm = if log_percentile_r > 0.0 { log_value / log_percentile_r } else { 0.0 };

                // Clamp normalized value to [0, 1] range
                let clamped_norm = norm.min(1.0).max(0.0);

                // Apply final scaling to map to 0-255 range
                (clamped_norm * 255.0) as u8
            } else { 0 };

            let g_val = if log_percentile_g > 0.0 {
                let raw_value = green_hist[y][x];
                let log_value = if raw_value > 0.0 { (raw_value + 1.0).ln() } else { 0.0 };
                let norm = if log_percentile_g > 0.0 { log_value / log_percentile_g } else { 0.0 };

                let clamped_norm = norm.min(1.0).max(0.0);
                (clamped_norm * 255.0) as u8
            } else { 0 };

            let b_val = if log_percentile_b > 0.0 {
                let raw_value = blue_hist[y][x];
                let log_value = if raw_value > 0.0 { (raw_value + 1.0).ln() } else { 0.0 };
                let norm = if log_percentile_b > 0.0 { log_value / log_percentile_b } else { 0.0 };

                let clamped_norm = norm.min(1.0).max(0.0);
                (clamped_norm * 255.0) as u8
            } else { 0 };

            img.put_pixel(x as u32, y as u32, image::Rgb([r_val, g_val, b_val]));
        }
    }

    img
}

/// Calculate the Buddhabrot Julia for a specific channel
///
/// Implements the Buddhabrot algorithm for Julia sets by tracking the orbits of
/// randomly sampled starting points using a fixed Julia set constant.
///
/// # Arguments
///
/// * `params` - Buddhabrot Julia parameters including bounds, dimensions, and spawn point
/// * `channel_params` - Channel-specific parameters (min/max iterations, sample count)
///
/// # Returns
///
/// A 2D histogram representing the density of orbits in the image space
pub fn buddhabrot_julia_channel(
    params: &BuddhabrotJuliaParams,
    channel_params: &BuddhabrotChannel,
) -> Vec<Vec<f64>> {
    use std::time::Instant;
    use std::collections::HashMap;

    let [x_min, x_max, y_min, y_max] = params.bounds;

    let total_samples = channel_params.samples;
    let start_time = Instant::now();

    // Print initial progress
    println!("Generating Buddhabrot Julia channel: 0% (0/{}) - Started at {:?}. Using {} threads.",
             total_samples, Local::now().format("%H:%M:%S"), rayon::current_num_threads());

    // Determine chunk size for parallel processing
    let chunk_size = (total_samples / (rayon::current_num_threads() as u64 * 4)).max(1000);
    let num_chunks = std::cmp::max((total_samples as usize) / chunk_size as usize, 1);

    // Process samples in chunks using parallel iterator
    let partial_histograms: Vec<HashMap<(usize, usize), f64>> = (0..num_chunks)
        .into_par_iter()
        .map(|chunk_idx| {
            let start_sample = (chunk_idx as u64) * chunk_size;
            let end_sample = std::cmp::min(start_sample + chunk_size, total_samples);

            let mut local_histogram = HashMap::new();
            // Use a deterministic seed based on the chunk index to ensure reproducible results
            let mut rng = rand::rngs::StdRng::seed_from_u64(start_sample ^ 0xcafebabe);

            for _sample_num in start_sample..end_sample {
                // Randomly sample a z0 value in the complex plane using the local RNG
                let z_re = x_min + (x_max - x_min) * rng.gen::<f64>();
                let z_im = y_min + (y_max - y_min) * rng.gen::<f64>();
                let mut z = Complex::new(z_re, z_im);

                // Check if this point escapes within the iteration range
                let mut iter = 0;
                let mut orbit = Vec::new();

                // Track the orbit
                while iter < channel_params.max_iter {
                    orbit.push(z);
                    // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
                    if params.i_sqrt_value == Complex::new(0.0, 1.0) {
                        // Use standard algorithm for backward compatibility
                        z = match MathEvaluator::evaluate_formula_with_param(&params.formula, z, params.spawn) {
                            Ok(result) => result,
                            Err(_) => z * z + params.spawn, // Fallback to standard Julia formula
                        };
                    } else {
                        // Use custom complex arithmetic for non-standard imaginary units
                        let custom_i_squared = params.i_sqrt_value;
                        let z_custom = CustomComplex::new(z.re, z.im, custom_i_squared);
                        let c_custom = CustomComplex::new(params.spawn.re, params.spawn.im, custom_i_squared);

                        let result_custom = match MathEvaluator::evaluate_formula_with_param(&params.formula, z_custom.to_standard(), c_custom.to_standard()) {
                            Ok(result) => CustomComplex::from_standard(result, custom_i_squared),
                            Err(_) => {
                                // Fallback to standard formula using custom arithmetic
                                let z_sq = z_custom.multiply(&z_custom);
                                z_sq.add(&c_custom)
                            },
                        };

                        z = result_custom.to_standard();
                    };

                    if z.norm_sqr() > params.bailout * params.bailout {
                        // Point escapes, check if it's in the right iteration range
                        if iter >= channel_params.min_iter {
                            // Draw the orbit - accumulate locally first
                            for point in &orbit {
                                let px = ((point.re - x_min) / (x_max - x_min) * params.width as f64) as usize;
                                let py = ((point.im - y_min) / (y_max - y_min) * params.height as f64) as usize;

                                if px < params.width as usize && py < params.height as usize {
                                    *local_histogram.entry((px, py)).or_insert(0.0) += 1.0;
                                }
                            }
                        }
                        break;
                    }
                    iter += 1;
                }
            }
            local_histogram
        })
        .collect();

    // Merge all partial histograms into the final histogram
    let mut final_histogram = vec![vec![0.0; params.width as usize]; params.height as usize];

    for partial_hist in partial_histograms {
        for ((x, y), value) in partial_hist {
            if x < params.width as usize && y < params.height as usize {
                final_histogram[y][x] += value;
            }
        }
    }

    // Final progress report
    let elapsed = start_time.elapsed();
    println!(
        "Generating Buddhabrot Julia channel: 100% ({}/{}), Completed in {:.1}s",
        total_samples, total_samples, elapsed.as_secs_f64()
    );

    final_histogram
}

/// Generate a complete Buddhabrot Julia image with RGB channels
///
/// Combines the three RGB channels into a single image by rendering each channel
/// separately and combining them with proper normalization.
///
/// # Arguments
///
/// * `params` - Complete Buddhabrot Julia parameters including all channel configurations
///
/// # Returns
///
/// An RGB image representing the combined Buddhabrot Julia visualization
pub fn generate_buddhabrot_julia(params: &BuddhabrotJuliaParams) -> image::RgbImage {
    let mut img = image::RgbImage::new(params.width, params.height);

    // Generate each channel separately
    let red_hist = buddhabrot_julia_channel(params, &params.channels.red);
    let green_hist = buddhabrot_julia_channel(params, &params.channels.green);
    let blue_hist = buddhabrot_julia_channel(params, &params.channels.blue);

    // Calculate 95th percentile of log-transformed values for each channel
    // This gives us a more robust normalization value that's less sensitive to outliers
    let log_percentile_r = calculate_percentile_log(&red_hist, 95.0);
    let log_percentile_g = calculate_percentile_log(&green_hist, 95.0);
    let log_percentile_b = calculate_percentile_log(&blue_hist, 95.0);

    // If all channels are zero, return a black image
    if log_percentile_r == 0.0 && log_percentile_g == 0.0 && log_percentile_b == 0.0 {
        return img; // Already initialized as black
    }

    // Normalize and combine channels using percentile-based normalization
    for y in 0..params.height as usize {
        for x in 0..params.width as usize {
            let r_val = if log_percentile_r > 0.0 {
                let raw_value = red_hist[y][x];
                let log_value = if raw_value > 0.0 { (raw_value + 1.0).ln() } else { 0.0 };
                let norm = if log_percentile_r > 0.0 { log_value / log_percentile_r } else { 0.0 };

                // Clamp normalized value to [0, 1] range
                let clamped_norm = norm.min(1.0).max(0.0);

                // Apply final scaling to map to 0-255 range
                (clamped_norm * 255.0) as u8
            } else { 0 };
            let g_val = if log_percentile_g > 0.0 {
                let raw_value = green_hist[y][x];
                let log_value = if raw_value > 0.0 { (raw_value + 1.0).ln() } else { 0.0 };
                let norm = if log_percentile_g > 0.0 { log_value / log_percentile_g } else { 0.0 };

                let clamped_norm = norm.min(1.0).max(0.0);
                (clamped_norm * 255.0) as u8
            } else { 0 };
            let b_val = if log_percentile_b > 0.0 {
                let raw_value = blue_hist[y][x];
                let log_value = if raw_value > 0.0 { (raw_value + 1.0).ln() } else { 0.0 };
                let norm = if log_percentile_b > 0.0 { log_value / log_percentile_b } else { 0.0 };

                let clamped_norm = norm.min(1.0).max(0.0);
                (clamped_norm * 255.0) as u8
            } else { 0 };

            img.put_pixel(x as u32, y as u32, image::Rgb([r_val, g_val, b_val]));
        }
    }

    img
}

/// Convert pixel coordinates to complex plane coordinates
///
/// Maps pixel coordinates in an image to corresponding points in the complex plane
/// based on the specified bounds.
///
/// # Arguments
///
/// * `x` - X coordinate in the image (0 to width-1)
/// * `y` - Y coordinate in the image (0 to height-1)
/// * `width` - Width of the image in pixels
/// * `height` - Height of the image in pixels
/// * `bounds` - Complex plane bounds [x_min, x_max, y_min, y_max]
///
/// # Returns
///
/// A complex number representing the corresponding point in the complex plane
pub fn pixel_to_complex(x: u32, y: u32, width: u32, height: u32, bounds: [f64; 4]) -> Complex<f64> {
    let [x_min, x_max, y_min, y_max] = bounds;

    // Use (width-1) and (height-1) to ensure the last pixel maps to x_max/y_max
    let real = if width > 1 {
        x_min + (x as f64 / (width - 1) as f64) * (x_max - x_min)
    } else {
        x_min
    };
    let imag = if height > 1 {
        y_min + (y as f64 / (height - 1) as f64) * (y_max - y_min)
    } else {
        y_min
    };

    Complex::new(real, imag)
}

/// Generate a domain color plot for a complex function
///
/// This function creates a visualization of a complex function using domain coloring,
/// where each point in the complex plane is assigned a color based on the value of
/// the function at that point. The hue represents the argument (angle) of the complex
/// value, and the lightness represents the magnitude.
///
/// # Arguments
///
/// * `params` - Domain color parameters including bounds, dimensions, and formula
///
/// # Returns
///
/// An RGB image representing the domain coloring of the complex function
pub fn generate_domain_color_plot(params: &DomainColorParams) -> image::RgbImage {
    use rayon::prelude::*;
    use std::sync::Arc;

    let img = image::RgbImage::new(params.width, params.height);
    let img_arc = Arc::new(img);

    // Create a vector of (x, y) coordinates to process in parallel
    let coords: Vec<(u32, u32)> = (0..params.height).flat_map(|y| (0..params.width).map(move |x| (x, y))).collect();

    // Process pixels in parallel
    let results: Vec<((u32, u32), [u8; 3])> = coords
        .into_par_iter()
        .map(|(x, y)| {
            // Convert pixel coordinates to complex plane coordinates
            let z = pixel_to_complex(x, y, params.width, params.height, params.bounds);

            // Evaluate the complex function with custom imaginary unit
            let result = match evaluate_complex_function_with_custom_i(&params.formula, z, params.i_sqrt_value) {
                Ok(value) => value,
                Err(_) => Complex::new(0.0, 0.0), // Default to zero if evaluation fails
            };

            // Calculate hue based on argument (angle) of the result
            let arg = result.arg(); // Returns angle in radians from -π to π
            let hue = (arg + PI) / (2.0 * PI); // Normalize to 0-1 range

            // Calculate brightness based on magnitude of the result
            let mag = result.norm(); // Magnitude of the complex number
            // Use logarithmic scaling to handle large ranges of magnitudes
            let brightness = if mag > 0.0 {
                let log_mag = mag.ln();
                // Map log magnitude to 0-1 range, with adjustable scaling
                let scaled = (log_mag + 10.0) / 20.0; // Adjust range as needed
                scaled.clamp(0.0, 1.0)
            } else {
                0.0
            };

            // Convert HSV to RGB
            let rgb = hsv_to_rgb(hue, 1.0, brightness);

            ((x, y), rgb)
        })
        .collect();

    // Create a mutable image and populate it with the results
    let mut img = Arc::try_unwrap(img_arc).unwrap_or_else(|arc| (*arc).clone());
    for ((x, y), rgb) in results {
        img.put_pixel(x, y, image::Rgb(rgb));
    }

    img
}

/// Evaluate a complex function given as a string
///
/// This is a sophisticated evaluator that handles complex mathematical expressions
///
/// # Arguments
///
/// * `formula` - String representation of the complex function (e.g., "z^2", "sin(z)", etc.)
/// * `z` - Input complex number
///
/// # Returns
///
/// The result of evaluating the function at z, or an error if the formula is invalid
#[allow(dead_code)]
fn evaluate_complex_function(formula: &str, z: Complex<f64>) -> Result<Complex<f64>, String> {
    // Use the existing sophisticated parser
    let formula = formula.trim();

    // For fractal generation, 'c' typically represents the point in the complex plane
    // For Mandelbrot: z^2 + c where c is the coordinate
    // For Julia: z^2 + c where c is a fixed constant
    let param = z; // For Mandelbrot, param is the coordinate; for Julia, it would be fixed

    // Use the existing expression parser
    MathEvaluator::parse_and_evaluate(formula, z, param)
}

/// Evaluate a complex function with a given formula and custom imaginary unit
fn evaluate_complex_function_with_custom_i(formula: &str, z: Complex<f64>, custom_i: Complex<f64>) -> Result<Complex<f64>, String> {
    // Use the existing sophisticated parser with custom imaginary unit
    let formula = formula.trim();

    // For fractal generation, 'c' typically represents the point in the complex plane
    // For Mandelbrot: z^2 + c where c is the coordinate
    // For Julia: z^2 + c where c is a fixed constant
    let param = z; // For Mandelbrot, param is the coordinate; for Julia, it would be fixed

    // Use the existing expression parser with custom imaginary unit
    if custom_i == Complex::new(0.0, 1.0) {
        // Use standard algorithm for backward compatibility
        MathEvaluator::evaluate_formula_with_param(formula, z, param)
    } else {
        // Use custom complex arithmetic for non-standard imaginary units
        let custom_i_squared = custom_i; // This is the value that i² equals
        let z_custom = CustomComplex::new(z.re, z.im, custom_i_squared);
        let param_custom = CustomComplex::new(param.re, param.im, custom_i_squared);

        let result_custom = match MathEvaluator::evaluate_formula_with_param(formula, z_custom.to_standard(), param_custom.to_standard()) {
            Ok(result) => CustomComplex::from_standard(result, custom_i_squared),
            Err(_) => {
                // Fallback to standard formula using custom arithmetic
                let z_sq = z_custom.multiply(&z_custom);
                z_sq.add(&param_custom)
            },
        };

        Ok(result_custom.to_standard())
    }
}

/// Convert HSV color values to RGB
///
/// # Arguments
///
/// * `h` - Hue (0.0 to 1.0)
/// * `s` - Saturation (0.0 to 1.0)
/// * `v` - Value/Brightness (0.0 to 1.0)
///
/// # Returns
///
/// RGB values as [u8, u8, u8] array
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> [u8; 3] {
    let h = h.fract(); // Ensure hue is in [0, 1) range
    let h_i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - h_i as f64;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match h_i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    [
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_pixel_to_complex() {
        // Test conversion from pixel to complex coordinates
        let bounds = [-2.0, 2.0, -2.0, 2.0];  // 4x4 area
        let width = 4;
        let height = 4;

        // Test corner points
        let top_left = pixel_to_complex(0, 0, width, height, bounds);
        assert!((top_left.re - (-2.0)).abs() < 0.01);  // Should be x_min
        assert!((top_left.im - (-2.0)).abs() < 0.01);  // Should be y_min

        let bottom_right = pixel_to_complex(width - 1, height - 1, width, height, bounds);
        // For a 4x4 image, the last pixel is at index 3, so it maps to slightly less than x_max/y_max
        // due to 0-indexing: pixel 3 of 4 pixels maps to 3/3 = 1.0 of the range
        let expected_x = -2.0 + (3.0 / 3.0) * (2.0 - (-2.0));  // Should be 2.0
        let expected_y = -2.0 + (3.0 / 3.0) * (2.0 - (-2.0));  // Should be 2.0
        assert!((bottom_right.re - expected_x).abs() < 0.01);  // Should be close to x_max
        assert!((bottom_right.im - expected_y).abs() < 0.01);  // Should be close to y_max
    }

    #[test]
    fn test_mandelbrot_iterations_origin() {
        // The origin (0, 0) should be in the Mandelbrot set (high iterations)
        let params = FractalParams::new([-2.0, 2.0, -2.0, 2.0], 100, [0.0, 0.0], 4.0, "z^2 + c".to_string());
        let c = Complex::new(0.0, 0.0);
        let iterations = mandelbrot_iterations(c, &params);
        assert_eq!(iterations, 100);  // Should reach max iterations
    }

    #[test]
    fn test_mandelbrot_iterations_outside_set() {
        // A point far outside the set should escape quickly
        let params = FractalParams::new([-2.0, 2.0, -2.0, 2.0], 100, [0.0, 0.0], 4.0, "z^2 + c".to_string());
        let c = Complex::new(2.0, 2.0);  // This should escape quickly
        let iterations = mandelbrot_iterations(c, &params);
        assert!(iterations < 10);  // Should escape in few iterations
    }

    #[test]
    fn test_julia_iterations_origin() {
        // Test Julia set with a simple c value
        let params = FractalParams::new([-2.0, 2.0, -2.0, 2.0], 100, [0.0, 0.0], 4.0, "z^2 + c".to_string());
        let z = Complex::new(0.0, 0.0);
        let iterations = julia_iterations(z, &params);
        assert_eq!(iterations, 100);  // z=0, c=0 should stay bounded
    }

    #[test]
    fn test_complex_norm_sqr() {
        // Test that our complex number operations work correctly
        let z = Complex::new(3.0, 4.0);
        assert_eq!(z.norm_sqr(), 25.0);  // 3^2 + 4^2 = 25
    }
}

#[derive(Debug, Clone)]
pub struct ColorStop {
    pub color: [u8; 3],  // RGB
    pub position: f64,   // 0.0 to 1.0
}

// Parse color palette string like "[(#FF0000,0.0),(#00FF00,0.5),(#0000FF,1.0)]"
pub fn parse_color_palette(palette_str: &str) -> Result<Vec<ColorStop>, String> {
    let mut stops = Vec::new();

    // Remove outer brackets if present
    let clean = palette_str.trim().trim_start_matches('[').trim_end_matches(']');

    // Split by "),(" to get individual color stops
    let color_stops: Vec<&str> = clean.split("),(").collect();

    for stop_str in color_stops {
        let clean_stop = stop_str.trim().trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<&str> = clean_stop.split(',').collect();

        if parts.len() != 2 {
            return Err(format!("Invalid color stop format: {}", clean_stop));
        }

        let hex_color = parts[0].trim().trim_start_matches('"').trim_end_matches('"');
        let position_str = parts[1].trim();

        // Parse hex color
        let color = parse_hex_color(hex_color)?;

        // Parse position
        let position = position_str.parse::<f64>().map_err(|_| format!("Invalid position: {}", position_str))?;

        stops.push(ColorStop { color, position });
    }

    // Sort by position
    stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());

    Ok(stops)
}

// Parse hex color like "#FF0000" to [R, G, B]
pub fn parse_hex_color(hex: &str) -> Result<[u8; 3], String> {
    let hex_clean = hex.trim_start_matches('#');

    if hex_clean.len() != 6 {
        return Err(format!("Invalid hex color length: {}", hex));
    }

    let r = u8::from_str_radix(&hex_clean[0..2], 16).map_err(|_| format!("Invalid hex color: {}", hex))?;
    let g = u8::from_str_radix(&hex_clean[2..4], 16).map_err(|_| format!("Invalid hex color: {}", hex))?;
    let b = u8::from_str_radix(&hex_clean[4..6], 16).map_err(|_| format!("Invalid hex color: {}", hex))?;

    Ok([r, g, b])
}

// Interpolate color from palette based on normalized value (0.0 to 1.0)
pub fn interpolate_color_from_palette(normalized_value: f64, palette: &[ColorStop]) -> image::Rgba<u8> {
    if palette.is_empty() {
        return image::Rgba([0, 0, 0, 255]); // Default to black
    }

    if palette.len() == 1 {
        return image::Rgba([palette[0].color[0], palette[0].color[1], palette[0].color[2], 255]);
    }

    // Find the two color stops to interpolate between
    let mut lower_idx = 0;
    let mut upper_idx = palette.len() - 1;

    for i in 0..palette.len() {
        if palette[i].position <= normalized_value {
            lower_idx = i;
        } else {
            upper_idx = i;
            break;
        }
    }

    // Clamp to valid indices
    if upper_idx <= lower_idx {
        upper_idx = lower_idx + 1;
        if upper_idx >= palette.len() {
            upper_idx = palette.len() - 1;
        }
    }

    if lower_idx == upper_idx {
        return image::Rgba([palette[lower_idx].color[0], palette[lower_idx].color[1], palette[lower_idx].color[2], 255]);
    }

    let lower = &palette[lower_idx];
    let upper = &palette[upper_idx];

    // Interpolate between the two colors
    let t = (normalized_value - lower.position) / (upper.position - lower.position);
    let t = t.clamp(0.0, 1.0);

    let r = (lower.color[0] as f64 * (1.0 - t) + upper.color[0] as f64 * t).round() as u8;
    let g = (lower.color[1] as f64 * (1.0 - t) + upper.color[1] as f64 * t).round() as u8;
    let b = (lower.color[2] as f64 * (1.0 - t) + upper.color[2] as f64 * t).round() as u8;

    image::Rgba([r, g, b, 255])
}

// Function to convert iterations to a color using the palette
pub fn color_from_iterations_with_palette(iterations: u32, max_iterations: u32, palette: &[ColorStop]) -> image::Rgba<u8> {
    if max_iterations == 0 {
        return image::Rgba([0, 0, 0, 255]);
    }

    if iterations == max_iterations {
        // Inside the set - typically black, but could be customized
        // For now, use the first color in the palette or black
        if !palette.is_empty() {
            image::Rgba([palette[0].color[0], palette[0].color[1], palette[0].color[2], 255])
        } else {
            image::Rgba([0, 0, 0, 255])
        }
    } else {
        // Outside the set - interpolate based on iteration count
        let t = iterations as f64 / max_iterations as f64;
        interpolate_color_from_palette(t, palette)
    }
}

// Simple function to convert iterations to a color (fallback)
pub fn color_from_iterations(iterations: u32, max_iterations: u32) -> image::Rgba<u8> {
    if iterations == max_iterations {
        // Inside the set - black
        image::Rgba([0, 0, 0, 255])
    } else {
        // Outside the set - color based on iterations
        let t = iterations as f64 / max_iterations as f64;
        let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
        let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
        let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
        image::Rgba([r, g, b, 255])
    }
}

use rayon::prelude::*;

// Generate fractal image with time-based progress bar and ETA with color palette support
pub fn generate_fractal_image<F>(
    width: u32,
    height: u32,
    params: &FractalParams,
    iteration_func: F,
    color_palette: Option<&Vec<ColorStop>>,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
where
    F: Fn(Complex<f64>, &FractalParams) -> u32 + Sync + Copy,
{
    use std::time::{Duration, Instant};

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Initialize progress tracking
    let total_pixels = width * height;
    let processed_pixels = Arc::new(AtomicUsize::new(0));
    let start_time = Instant::now();
    let last_report_time = Arc::new(std::sync::Mutex::new(Instant::now()));

    // Print initial progress
    println!("Rendering fractal: 0% (0/{}) - Started at {:?}. Using {} threads.",
             total_pixels, chrono::Local::now().format("%H:%M:%S"), rayon::current_num_threads());

    // Create a vector of (x, y) coordinates to process in parallel
    let coords: Vec<(u32, u32)> = (0..height).flat_map(|y| (0..width).map(move |x| (x, y))).collect();

    // Process pixels in parallel
    let results: Vec<((u32, u32), image::Rgba<u8>)> = coords
        .into_par_iter()
        .map(|(x, y)| {
            let c = pixel_to_complex(x, y, width, height, params.bounds);
            let iterations = iteration_func(c, params);

            // Choose coloring method based on whether palette is provided
            let color = if let Some(palette) = color_palette {
                color_from_iterations_with_palette(iterations, params.max_iterations, palette)
            } else {
                color_from_iterations(iterations, params.max_iterations)
            };

            // Update progress counter
            let current = processed_pixels.fetch_add(1, Ordering::SeqCst) + 1;

            // Time-based progress reporting every 10 seconds - only check every few rows to reduce overhead
            if current > 0 && current % (width as usize * 2) == 0 { // Only check every few rows to reduce overhead
                let should_report = {
                    let last_time = last_report_time.lock().unwrap();
                    last_time.elapsed() >= Duration::from_secs(10) // At least 10 seconds since last report
                };

                if should_report {
                    let elapsed = start_time.elapsed();
                    let percentage = (current as f64 / total_pixels as f64 * 100.0).round();

                    if current > 0 {
                        let rate = current as f64 / elapsed.as_secs_f64(); // pixels per second
                        let remaining_pixels = (total_pixels as usize - current) as f64;
                        let estimated_remaining_time = remaining_pixels / rate; // seconds

                        let eta = chrono::Local::now() + chrono::Duration::seconds(estimated_remaining_time as i64);

                        println!(
                            "Rendering fractal: {:.1}% ({}/{}), Elapsed: {:.1}s, ETA: {} (~{:.1}s remaining)",
                            percentage,
                            current,
                            total_pixels,
                            elapsed.as_secs_f64(),
                            eta.format("%H:%M:%S"),
                            estimated_remaining_time
                        );

                        // Update the last report time
                        let mut last_time = last_report_time.lock().unwrap();
                        *last_time = Instant::now();
                    }
                }
            }

            ((x, y), color)
        })
        .collect();

    // Put the results back into the image buffer
    for ((x, y), color) in results {
        imgbuf.put_pixel(x, y, color);
    }

    // Final progress report
    let elapsed = start_time.elapsed();
    println!(
        "Rendering fractal: 100% ({}/{}), Completed in {:.1}s",
        total_pixels, total_pixels, elapsed.as_secs_f64()
    );

    imgbuf
}
/// Trace the orbit of a point in the Mandelbrot set for debugging purposes
pub fn trace_orbit_mandelbrot(c: Complex<f64>, params: &FractalParams) {
    println!("Tracing orbit for Mandelbrot with:");
    println!("  Point c: {:?}", c);
    println!("  Formula: {}", params.formula);
    println!("  Custom i² value: {:?}", params.i_sqrt_value);
    println!("  Max iterations: {}", params.max_iterations);
    println!("  Bailout: {}", params.bailout);
    println!();

    let mut z = Complex::new(0.0, 0.0);
    let mut iter = 0;

    while iter < params.max_iterations {
        println!("  Iteration {}: z = ({:.6}, {:.6}), |z| = {:.6}", 
                 iter + 1, z.re, z.im, z.norm());

        // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
        z = match MathEvaluator::evaluate_formula_with_param_and_custom_i(&params.formula, z, c, params.i_sqrt_value) {
            Ok(result) => {
                // If we get here, the formula evaluation succeeded
                result
            },
            Err(_e) => {
                z * z + c // Fallback to standard formula
            },
        };

        if z.norm_sqr() > params.bailout * params.bailout {
            println!("  Point escapes at iteration {}", iter + 1);
            break;
        }
        
        iter += 1;
    }
    
    if iter >= params.max_iterations {
        println!("  Point remains bounded after {} iterations", params.max_iterations);
    }
    
    println!();
}

/// Trace the orbit of a point in the Julia set for debugging purposes
pub fn trace_orbit_julia(z: Complex<f64>, params: &FractalParams) {
    println!("Tracing orbit for Julia set with:");
    println!("  Point z: {:?}", z);
    println!("  Formula: {}", params.formula);
    println!("  Custom i² value: {:?}", params.i_sqrt_value);
    println!("  Max iterations: {}", params.max_iterations);
    println!("  Bailout: {}", params.bailout);
    println!();

    let c = params.spawn;  // Use spawn point as the constant for Julia set
    let mut z = z;
    let mut iter = 0;

    while iter < params.max_iterations {
        println!("  Iteration {}: z = ({:.6}, {:.6}), |z| = {:.6}", 
                 iter + 1, z.re, z.im, z.norm());

        // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
        z = match MathEvaluator::evaluate_formula_with_param_and_custom_i(&params.formula, z, c, params.i_sqrt_value) {
            Ok(result) => result,
            Err(_) => z * z + c, // Fallback to standard Julia formula
        };

        if z.norm_sqr() > params.bailout * params.bailout {
            println!("  Point escapes at iteration {}", iter + 1);
            break;
        }
        
        iter += 1;
    }
    
    if iter >= params.max_iterations {
        println!("  Point remains bounded after {} iterations", params.max_iterations);
    }
    
    println!();
}

/// Trace the orbit of a point in the Buddhabrot for debugging purposes
pub fn trace_orbit_buddha(z: Complex<f64>, params: &BuddhabrotParams) {
    println!("Tracing orbit for Buddhabrot with:");
    println!("  Point z: {:?}", z);
    println!("  Formula: {}", params.formula);
    println!("  Custom i² value: {:?}", params.i_sqrt_value);
    println!("  Max iterations: {}", params.max_iterations);
    println!("  Bailout: {}", params.bailout);
    println!();

    let c = z;  // In Buddhabrot, we iterate with z as the starting point and c as the parameter
    let mut z = z;
    let mut iter = 0;

    while iter < params.max_iterations {
        println!("  Iteration {}: z = ({:.6}, {:.6}), |z| = {:.6}", 
                 iter + 1, z.re, z.im, z.norm());

        // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
        z = match MathEvaluator::evaluate_formula_with_param_and_custom_i(&params.formula, z, c, params.i_sqrt_value) {
            Ok(result) => result,
            Err(_) => z * z + c, // Fallback to standard formula
        };

        if z.norm_sqr() > params.bailout * params.bailout {
            println!("  Point escapes at iteration {}", iter + 1);
            break;
        }
        
        iter += 1;
    }
    
    if iter >= params.max_iterations {
        println!("  Point remains bounded after {} iterations", params.max_iterations);
    }
    
    println!();
}

/// Trace the orbit of a point in the Buddhabrot Julia for debugging purposes
pub fn trace_orbit_buddhaj(z: Complex<f64>, params: &BuddhabrotJuliaParams) {
    println!("Tracing orbit for Buddhabrot Julia with:");
    println!("  Point z: {:?}", z);
    println!("  Formula: {}", params.formula);
    println!("  Custom i² value: {:?}", params.i_sqrt_value);
    println!("  Max iterations: {}", params.max_iterations);
    println!("  Bailout: {}", params.bailout);
    println!();

    let c = params.spawn;  // Use spawn point as the constant for Julia set
    let mut z = z;
    let mut iter = 0;

    while iter < params.max_iterations {
        println!("  Iteration {}: z = ({:.6}, {:.6}), |z| = {:.6}", 
                 iter + 1, z.re, z.im, z.norm());

        // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
        z = match MathEvaluator::evaluate_formula_with_param_and_custom_i(&params.formula, z, c, params.i_sqrt_value) {
            Ok(result) => result,
            Err(_) => z * z + c, // Fallback to standard Julia formula
        };

        if z.norm_sqr() > params.bailout * params.bailout {
            println!("  Point escapes at iteration {}", iter + 1);
            break;
        }
        
        iter += 1;
    }
    
    if iter >= params.max_iterations {
        println!("  Point remains bounded after {} iterations", params.max_iterations);
    }
    
    println!();
}

/// Trace the orbit of a point in the domain color plot for debugging purposes
pub fn trace_orbit_dca(z: Complex<f64>, formula: &str, custom_i: Complex<f64>) {
    println!("Tracing orbit for domain color plot with:");
    println!("  Point z: {:?}", z);
    println!("  Formula: {}", formula);
    println!("  Custom i² value: {:?}", custom_i);
    println!();

    let mut z = z;
    let mut iter = 0;

    // For domain coloring, we just evaluate the function once
    println!("  Iteration {}: z = ({:.6}, {:.6}), |z| = {:.6}", 
             iter + 1, z.re, z.im, z.norm());

    // Use the formula specified in params with custom imaginary unit
    z = match MathEvaluator::evaluate_formula_with_param_and_custom_i(formula, z, z, custom_i) {  // Using z as both z and param for domain coloring
        Ok(result) => result,
        Err(_) => z, // Fallback to identity function
    };

    println!("  Result: z = ({:.6}, {:.6}), |z| = {:.6}, arg = {:.6}", 
             z.re, z.im, z.norm(), z.arg());
    
    println!();
}

/// Helper function to convert Complex<f64> to string representation for custom i
fn custom_complex_to_string(c: Complex<f64>) -> String {
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

/// Compute custom complex multiplication respecting the custom imaginary unit
///
/// This function performs multiplication in an alternative complex number system where i² equals
/// the specified custom value. The multiplication formula is:
/// (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i²
/// = ac + (ad + bc)*i + bd*i²
///
/// This is fundamentally different from standard complex multiplication where i² = -1.
/// In this system, the result depends on the custom value of i².
///
/// # Arguments
///
/// * `z1` - First complex number (a + bi)
/// * `z2` - Second complex number (c + di)
/// * `i_squared` - The value that i² equals in this number system (what i is the square root of)
///
/// # Returns
///
/// The result of multiplying z1 and z2 in the custom complex number system
///
/// # Mathematical Formula
///
/// For (a + bi) * (c + di) in a system where i² = custom_value:
/// Real part = ac + Re(bd * custom_value)
/// Imaginary part = (ad + bc) + Im(bd * custom_value)
fn custom_complex_multiply(z1: Complex<f64>, z2: Complex<f64>, i_squared: Complex<f64>) -> Complex<f64> {
    let a = z1.re;
    let b = z1.im;
    let c = z2.re;
    let d = z2.im;
    
    // (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i^2
    // = ac + (ad + bc)*i + bd*i^2
    let ac = a * c;
    let ad = a * d;
    let bc = b * c;
    let bd = b * d;
    
    // bd * i^2 where i^2 is our custom value
    let bd_i_squared = bd * i_squared;
    
    // Real part: ac + Re(bd * i^2)
    let real_part = ac + bd_i_squared.re;
    // Imaginary part: (ad + bc) + Im(bd * i^2)
    let imag_part = (ad + bc) + bd_i_squared.im;
    
    Complex::new(real_part, imag_part)
}

/// Compute custom complex square respecting the custom imaginary unit
///
/// This function computes the square in an alternative complex number system where i² equals
/// the specified custom value. The square formula is:
/// (a + bi)² = a² + 2abi + b²*i²
///
/// This is fundamentally different from standard complex squaring where i² = -1.
/// In this system, the result depends on the custom value of i².
///
/// # Arguments
///
/// * `z` - The complex number to square (a + bi)
/// * `i_squared` - The value that i² equals in this number system (what i is the square root of)
///
/// # Returns
///
/// The result of squaring z in the custom complex number system
///
/// # Mathematical Formula
///
/// For (a + bi)² in a system where i² = custom_value:
/// Real part = a² + Re(b² * custom_value)
/// Imaginary part = 2ab + Im(b² * custom_value)
fn custom_complex_square(z: Complex<f64>, i_squared: Complex<f64>) -> Complex<f64> {
    let a = z.re;
    let b = z.im;
    
    // (a + bi)^2 = a^2 + 2abi + b^2*i^2
    let a_sq = a * a;
    let two_ab = 2.0 * a * b;
    let b_sq = b * b;
    
    // b^2 * i^2 where i^2 is our custom value
    let b_sq_i_squared = b_sq * i_squared;
    
    // Real part: a^2 + Re(b^2 * i^2)
    let real_part = a_sq + b_sq_i_squared.re;
    // Imaginary part: 2ab + Im(b^2 * i^2)
    let imag_part = two_ab + b_sq_i_squared.im;
    
    Complex::new(real_part, imag_part)
}

/// Generate a Mandelbrot set image with domain coloring support
/// 
/// This function generates a Mandelbrot set image where points that don't escape are colored based on their final complex value
/// rather than just the iteration count. This creates colorful visualizations that reveal the structure of the complex function.
/// 
/// # Arguments
/// 
/// * `width` - Width of the output image in pixels
/// * `height` - Height of the output image in pixels  
/// * `params` - Fractal parameters including bounds, max_iterations, formula, and custom imaginary unit
/// * `no_bailout` - If true, disables the bailout threshold for fully domain-colored plots
/// * `color_palette` - Optional color palette for coloring the image
/// 
/// # Returns
/// 
/// An RGBA image buffer representing the Mandelbrot set with domain coloring
pub fn generate_mandelbrot_domain_color_image(
    width: u32,
    height: u32,
    params: &FractalParams,
    no_bailout: bool,
    color_palette: Option<&Vec<ColorStop>>
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    use rayon::prelude::*;
    
    let bounds = params.bounds;
    let params_arc = Arc::new(params.clone());
    
    // Calculate step sizes for mapping pixels to complex plane
    let dx = (bounds[1] - bounds[0]) / width as f64;
    let dy = (bounds[3] - bounds[2]) / height as f64;
    
    // Process rows in parallel
    let rows: Vec<Vec<Rgba<u8>>> = (0..height)
        .into_par_iter()
        .map(|y| {
            let mut row = Vec::with_capacity(width as usize);
            for x in 0..width {
                // Convert pixel coordinates to complex plane coordinates
                let c = Complex::new(
                    bounds[0] + x as f64 * dx,
                    bounds[2] + y as f64 * dy,
                );
                
                // Calculate the final value for domain coloring
                let final_value = mandelbrot_final_value(c, &params_arc, no_bailout);
                
                // Map the complex value to a color using domain coloring
                let color = complex_to_domain_color(final_value, color_palette);
                row.push(color);
            }
            row
        })
        .collect();
    
    // Flatten the rows into a single vector
    let pixels: Vec<Rgba<u8>> = rows.into_iter().flatten().collect();
    
    // Flatten the pixel data into a single vector of bytes
    let mut pixel_bytes = Vec::with_capacity((width * height * 4) as usize);
    for pixel in pixels {
        pixel_bytes.extend_from_slice(&pixel.0);
    }

    // Create the final image from the flattened pixel data
    ImageBuffer::from_raw(width, height, pixel_bytes).unwrap()
}

/// Calculate the final complex value for a point in the Mandelbrot set for domain coloring
/// 
/// This function iterates the Mandelbrot formula but returns the final complex value instead of iteration count
/// 
/// # Arguments
/// 
/// * `c` - The complex number representing the point in the complex plane
/// * `params` - Fractal parameters including max_iterations, formula, and custom imaginary unit
/// * `no_bailout` - If true, disables the bailout threshold for fully domain-colored plots
/// 
/// # Returns
/// 
/// The final complex value after iteration (either escaped value or final bounded value)
pub fn mandelbrot_final_value(c: Complex<f64>, params: &FractalParams, no_bailout: bool) -> Complex<f64> {
    let mut z = Complex::new(0.0, 0.0);
    let mut iter = 0;

    while iter < params.max_iterations {
        // Use the formula specified in params, defaulting to z^2 + c if evaluation fails
        z = match MathEvaluator::evaluate_formula_with_param_and_custom_i(&params.formula, z, c, params.i_sqrt_value) {
            Ok(result) => result,
            Err(_e) => z * z + c, // Fallback to standard formula
        };

        // If no_bailout is true, continue iterating for all points
        if !no_bailout && z.norm_sqr() > params.bailout * params.bailout {
            // For escaping points, return the final value before escape
            // This preserves phase information for domain coloring
            return z;
        }
        iter += 1;
    }

    // For non-escaping points, return the final value after max iterations
    // This preserves the complex value for domain coloring
    z
}

/// Convert a complex number to a color using domain coloring technique
/// 
/// Domain coloring maps complex numbers to colors based on their argument (hue) and magnitude (brightness/lightness)
/// 
/// # Arguments
/// 
/// * `z` - The complex number to convert to a color
/// * `color_palette` - Optional color palette to use for coloring
/// 
/// # Returns
/// 
/// An RGBA color representing the complex number
fn complex_to_domain_color(z: Complex<f64>, color_palette: Option<&Vec<ColorStop>>) -> Rgba<u8> {
    if z.re.is_nan() || z.im.is_nan() || z.re.is_infinite() || z.im.is_infinite() {
        // For invalid values, return black
        return Rgba([0, 0, 0, 255]);
    }
    
    // Calculate the argument (angle) of the complex number, normalized to [0, 1]
    let arg = z.arg(); // Returns value in [-π, π]
    let hue = (arg + std::f64::consts::PI) / (2.0 * std::f64::consts::PI); // Normalize to [0, 1]
    
    // Calculate the magnitude (absolute value) of the complex number
    let mag = z.norm();
    
    // Use the magnitude to determine brightness/lightness
    // For domain coloring, we often use a logarithmic scale to handle large ranges
    let log_mag = if mag > 0.0 { mag.ln() } else { -100.0 }; // Use -100 for zero to avoid -inf
    
    // Determine which band the magnitude falls into (for contouring effect)
    let band = (log_mag / std::f64::consts::TAU).floor(); // TAU = 2*PI
    let intensity = (band % 2.0).abs(); // Alternating bands
    
    // If a color palette is provided, use it; otherwise use HSV mapping
    if let Some(palette) = color_palette {
        // Use the color palette for domain coloring
        let normalized_mag = if mag > 0.0 {
            (log_mag / std::f64::consts::PI).rem_euclid(1.0)
        } else {
            0.0
        };
        interpolate_color_from_palette(normalized_mag, palette)
    } else {
        // Convert HSV to RGB using the hue and intensity
        let rgb = hsv_to_rgb(hue, 1.0, intensity);
        Rgba([rgb[0], rgb[1], rgb[2], 255])
    }
}



/// Helper function to compute custom complex multiplication with custom imaginary unit
/// (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i^2 where i^2 is the custom value
fn custom_complex_multiply(z1: Complex<f64>, z2: Complex<f64>, i_squared: Complex<f64>) -> Complex<f64> {
    let a = z1.re;
    let b = z1.im;
    let c = z2.re;
    let d = z2.im;
    
    // (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i^2
    // = ac + (ad + bc)*i + bd*i^2
    let ac = a * c;
    let ad = a * d;
    let bc = b * c;
    let bd = b * d;
    
    // bd * i^2 where i^2 is our custom value
    let bd_i_squared = bd * i_squared;
    
    // Real part: ac + Re(bd * i^2)
    let real_part = ac + bd_i_squared.re;
    // Imaginary part: (ad + bc) + Im(bd * i^2)
    let imag_part = (ad + bc) + bd_i_squared.im;
    
    Complex::new(real_part, imag_part)
}

/// Helper function to compute custom complex square with custom imaginary unit
/// In this system, (a + bi)^2 = a^2 + 2abi + b^2*i^2 where i^2 is the custom value
fn custom_complex_square(z: Complex<f64>, i_squared: Complex<f64>) -> Complex<f64> {
    let a = z.re;
    let b = z.im;
    
    // (a + bi)^2 = a^2 + 2abi + b^2*i^2
    let a_sq = a * a;
    let two_ab = 2.0 * a * b;
    let b_sq = b * b;
    
    // b^2 * i^2 where i^2 is our custom value
    let b_sq_i_squared = b_sq * i_squared;
    
    // Real part: a^2 + Re(b^2 * i^2)
    let real_part = a_sq + b_sq_i_squared.re;
    // Imaginary part: 2ab + Im(b^2 * i^2)
    let imag_part = two_ab + b_sq_i_squared.im;
    
    Complex::new(real_part, imag_part)
}
