//! # Fractal Toolkit Library
//!
//! A comprehensive library for generating various types of fractals including Mandelbrot sets,
//! Julia sets, and Buddhabrot variants. This library provides the core algorithms and
//! utilities for the fractal toolkit executables.
//!
//! ## Overview
//!
//! This library contains:
//! - Core fractal algorithms for Mandelbrot, Julia, and Buddhabrot sets
//! - Data structures for fractal parameters
//! - Image generation utilities
//! - Interactive HTML explorer generation
//!
//! ## Modules
//!
//! - `FractalParams`: Parameters for standard Mandelbrot and Julia sets
//! - `BuddhabrotParams`: Parameters for Buddhabrot rendering with RGB channels
//! - `BuddhabrotJuliaParams`: Parameters for Buddhabrot Julia sets
//! - Algorithm functions for each fractal type

use num_complex::Complex;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

/// Mathematical expression evaluator for complex numbers with support for various functions
#[derive(Debug, Clone)]
pub struct MathEvaluator;

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
                Self::parse_and_evaluate(formula, z, param)
            }
        }
    }

    /// Parse and evaluate more complex mathematical expressions
    fn parse_and_evaluate(formula: &str, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        // Use a more sophisticated expression parser
        ExpressionParser::evaluate(formula, z, param)
    }
} // End of first MathEvaluator implementation block

/// A more sophisticated expression parser for complex mathematical expressions
struct ExpressionParser;

impl ExpressionParser {
    /// Evaluate a mathematical expression with complex numbers
    pub fn evaluate(formula: &str, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        let tokens = Self::tokenize(formula)?;
        let mut pos = 0;
        let ast = Self::parse_expression(&tokens, &mut pos, z, param)?;
        Ok(ast.evaluate(z, param)?)
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
                    // Check for tetration operator ^^
                    if chars.peek() == Some(&'^') {
                        chars.next(); // consume the second ^
                        tokens.push(Token::Tetration);
                    } else {
                        tokens.push(Token::Power);
                    }
                    chars.next();
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
        let left = Self::parse_tetration(tokens, pos, z, param)?;

        if *pos < tokens.len() && matches!(tokens[*pos], Token::Power) {
            *pos += 1;
            let right = Self::parse_power(tokens, pos, z, param)?; // Right-associative power
            Ok(Box::new(BinaryOp::Pow(left, right)))
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
                if base.norm_sqr() < f64::EPSILON {
                    // 0^x where x is not zero
                    if exp.re != 0.0 || exp.im != 0.0 {
                        Ok(Complex::new(0.0, 0.0))
                    } else {
                        // 0^0 is typically defined as 1
                        Ok(Complex::new(1.0, 0.0))
                    }
                } else {
                    // Use the formula: z^w = exp(w * ln(z))
                    let log_base = base.ln();
                    let result = (exp * log_base).exp();
                    Ok(result)
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

/// Regular tetration for real base and height
/// Uses the regular iteration method for bases in the range (1, e^(1/e))
fn regular_tetration(base: f64, height: f64) -> f64 {
    if height.fract() == 0.0 {
        // For integer heights, use the iterative approach
        let n = height as u32;
        if n == 0 {
            1.0  // By convention, base^^0 = 1
        } else {
            let mut result = base;
            for _ in 1..n {
                result = base.powf(result);
            }
            result
        }
    } else {
        // For non-integer heights, use the regular iteration method
        // This is a simplified implementation - a full implementation would be much more complex
        // We'll use linear approximation between integer values
        let int_part = height.floor();
        let frac_part = height - int_part;

        if frac_part < 0.001 {  // Essentially an integer
            return regular_tetration(base, int_part);
        }

        // Calculate base^^(int_part) and base^^(int_part + 1)
        let lower = regular_tetration(base, int_part);
        let upper = base.powf(lower);  // This is base^^(int_part + 1)

        // Linear interpolation
        lower + frac_part * (upper - lower)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalParams {
    pub bounds: [f64; 4],           // [x_min, x_max, y_min, y_max]
    pub max_iterations: u32,
    pub spawn: Complex<f64>,        // For Julia sets
    pub bailout: f64,
    pub formula: String,
}

impl FractalParams {
    pub fn new(bounds: [f64; 4], max_iterations: u32, spawn: [f64; 2], bailout: f64, formula: String) -> Self {
        Self {
            bounds,
            max_iterations,
            spawn: Complex::new(spawn[0], spawn[1]),
            bailout,
            formula,
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
                    <option value="1280x720" selected>1280x720</option>
                    <option value="1920x1080">1920x1080</option>
                    <option value="2560x1440">2560x1440</option>
                    <option value="3840x2160">3840x2160</option>
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

            const width = Math.abs(currentX - startX);
            const height = Math.abs(currentY - startY);

            const left = Math.min(startX, currentX);
            const top = Math.min(startY, currentY);

            selectionBox.style.left = left + 'px';
            selectionBox.style.top = top + 'px';
            selectionBox.style.width = width + 'px';
            selectionBox.style.height = height + 'px';
        }}

        function endSelection() {{
            if (!isSelecting) return;
            isSelecting = false;

            // Calculate the selected region in complex plane coordinates
            const selectedXMin = bounds[0] + (startX / imgWidth) * (bounds[1] - bounds[0]);
            const selectedXMax = bounds[0] + (currentX / imgWidth) * (bounds[1] - bounds[0]);
            const selectedYMin = bounds[2] + (startY / imgHeight) * (bounds[3] - bounds[2]);
            const selectedYMax = bounds[2] + (currentY / imgHeight) * (bounds[3] - bounds[2]);

            // Ensure correct order
            const xMin = Math.min(selectedXMin, selectedXMax);
            const xMax = Math.max(selectedXMin, selectedXMax);
            const yMin = Math.min(selectedYMin, selectedYMax);
            const yMax = Math.max(selectedYMin, selectedYMax);

            // Get selected aspect ratio
            const selectedRatio = document.querySelector('input[name="aspect-ratio"]:checked').value;
            const [ratioX, ratioY] = selectedRatio.split(':').map(Number);

            // Get selected resolution
            const resolutionSelect = document.getElementById('resolution-select');
            const [width, height] = resolutionSelect.value.split('x').map(Number);

            // Generate the command
            const command = `{}`.replace('{{bounds}}', `[${{xMin}}, ${{xMax}}, ${{yMin}}, ${{yMax}}]`)
                                    .replace('{{dimensions}}', `[${{width}}, ${{height}}]`);

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
        image_path,
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

/// Calculate the number of iterations for a point in the Mandelbrot set
///
/// Determines how many iterations it takes for a complex point to escape the Mandelbrot set.
/// Points that remain bounded after max_iterations are considered part of the set.
///
/// # Arguments
///
/// * `c` - The complex number representing the point in the complex plane
/// * `params` - Fractal parameters including max_iterations and bailout value
///
/// # Returns
///
/// The number of iterations before the point escapes, or max_iterations if it remains bounded
pub fn mandelbrot_iterations(c: Complex<f64>, params: &FractalParams) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
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
}

/// Calculate the number of iterations for a point in a Julia set
///
/// Determines how many iterations it takes for a complex point to escape the Julia set.
/// Points that remain bounded after max_iterations are considered part of the set.
///
/// # Arguments
///
/// * `z` - The complex number representing the point in the complex plane
/// * `params` - Fractal parameters including max_iterations, spawn point (constant c), and bailout value
///
/// # Returns
///
/// The number of iterations before the point escapes, or max_iterations if it remains bounded
pub fn julia_iterations(z: Complex<f64>, params: &FractalParams) -> u32 {
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
    let mut histogram = vec![vec![0.0; params.width as usize]; params.height as usize];
    let [x_min, x_max, y_min, y_max] = params.bounds;

    // Generate random samples
    let mut rng = rand::rngs::StdRng::seed_from_u64(42); // Fixed seed for reproducibility

    for _ in 0..channel_params.samples {
        // Randomly sample a c value in the complex plane
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
            z = match MathEvaluator::evaluate_formula_with_param(&params.formula, z, c) {
                Ok(result) => result,
                Err(_) => z * z + c, // Fallback to standard formula
            };

            if z.norm_sqr() > params.bailout * params.bailout {
                // Point escapes, check if it's in the right iteration range
                if iter >= channel_params.min_iter {
                    // Draw the orbit
                    for point in &orbit {
                        let px = ((point.re - x_min) / (x_max - x_min) * params.width as f64) as usize;
                        let py = ((point.im - y_min) / (y_max - y_min) * params.height as f64) as usize;

                        if px < params.width as usize && py < params.height as usize {
                            histogram[py][px] += 1.0;
                        }
                    }
                }
                break;
            }
            iter += 1;
        }
    }

    histogram
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

    // Find max values for normalization
    let max_r = red_hist.iter().flatten().fold(0.0_f64, |a, &b| a.max(b));
    let max_g = green_hist.iter().flatten().fold(0.0_f64, |a, &b| a.max(b));
    let max_b = blue_hist.iter().flatten().fold(0.0_f64, |a, &b| a.max(b));

    // Normalize and combine channels with logarithmic scaling for better dynamic range
    for y in 0..params.height as usize {
        for x in 0..params.width as usize {
            let r_val = if max_r > 0.0 {
                let norm = red_hist[y][x] / max_r;
                // Use logarithmic scaling: log(1 + factor * normalized_value) / log(1 + factor)
                // This enhances visibility of low values while preserving high values
                let factor = 1000.0; // Adjust for desired contrast
                let log_norm = (1.0 + factor * norm).ln() / (1.0 + factor).ln();
                (log_norm * 255.0) as u8
            } else { 0 };
            let g_val = if max_g > 0.0 {
                let norm = green_hist[y][x] / max_g;
                let factor = 1000.0;
                let log_norm = (1.0 + factor * norm).ln() / (1.0 + factor).ln();
                (log_norm * 255.0) as u8
            } else { 0 };
            let b_val = if max_b > 0.0 {
                let norm = blue_hist[y][x] / max_b;
                let factor = 1000.0;
                let log_norm = (1.0 + factor * norm).ln() / (1.0 + factor).ln();
                (log_norm * 255.0) as u8
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
    let mut histogram = vec![vec![0.0; params.width as usize]; params.height as usize];
    let [x_min, x_max, y_min, y_max] = params.bounds;

    // Generate random samples
    let mut rng = rand::rngs::StdRng::seed_from_u64(42); // Fixed seed for reproducibility

    for _ in 0..channel_params.samples {
        // Randomly sample a z0 value in the complex plane
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
            z = match MathEvaluator::evaluate_formula_with_param(&params.formula, z, params.spawn) {
                Ok(result) => result,
                Err(_) => z * z + params.spawn, // Fallback to standard Julia formula
            };

            if z.norm_sqr() > params.bailout * params.bailout {
                // Point escapes, check if it's in the right iteration range
                if iter >= channel_params.min_iter {
                    // Draw the orbit
                    for point in &orbit {
                        let px = ((point.re - x_min) / (x_max - x_min) * params.width as f64) as usize;
                        let py = ((point.im - y_min) / (y_max - y_min) * params.height as f64) as usize;

                        if px < params.width as usize && py < params.height as usize {
                            histogram[py][px] += 1.0;
                        }
                    }
                }
                break;
            }
            iter += 1;
        }
    }

    histogram
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

    // Find max values for normalization
    let max_r = red_hist.iter().flatten().fold(0.0_f64, |a, &b| a.max(b));
    let max_g = green_hist.iter().flatten().fold(0.0_f64, |a, &b| a.max(b));
    let max_b = blue_hist.iter().flatten().fold(0.0_f64, |a, &b| a.max(b));

    // Normalize and combine channels with logarithmic scaling for better dynamic range
    for y in 0..params.height as usize {
        for x in 0..params.width as usize {
            let r_val = if max_r > 0.0 {
                let norm = red_hist[y][x] / max_r;
                // Use logarithmic scaling: log(1 + factor * normalized_value) / log(1 + factor)
                // This enhances visibility of low values while preserving high values
                let factor = 1000.0; // Adjust for desired contrast
                let log_norm = (1.0 + factor * norm).ln() / (1.0 + factor).ln();
                (log_norm * 255.0) as u8
            } else { 0 };
            let g_val = if max_g > 0.0 {
                let norm = green_hist[y][x] / max_g;
                let factor = 1000.0;
                let log_norm = (1.0 + factor * norm).ln() / (1.0 + factor).ln();
                (log_norm * 255.0) as u8
            } else { 0 };
            let b_val = if max_b > 0.0 {
                let norm = blue_hist[y][x] / max_b;
                let factor = 1000.0;
                let log_norm = (1.0 + factor * norm).ln() / (1.0 + factor).ln();
                (log_norm * 255.0) as u8
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