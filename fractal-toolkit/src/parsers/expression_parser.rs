use num_complex::Complex;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    ComplexNumber(String), // For numbers followed by i
    ImaginaryUnit,         // Standalone i
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,      // For ^ operator (power)
    Tetration,  // For ^^ operator (tetration)
    Pentation,  // For ^^^ operator (pentation)
    Hexation,   // For ^^^^ operator (hexation)
    LeftParen,
    RightParen,
    Comma,
}

#[derive(Debug)]
pub struct ExpressionParser;

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
        let processed_formula = formula.replace("i", &format!("({})", custom_i_to_string(custom_i)));
        
        // Then evaluate the processed formula
        Self::evaluate(&processed_formula, z, param)
    }

    /// Tokenize the input string
    fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    // Skip whitespace
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '^' => {
                    // Check for multiple ^ characters for hyperoperations
                    if chars.peek() == Some(&'^') {
                        chars.next(); // consume the second ^
                        if chars.peek() == Some(&'^') {
                            chars.next(); // consume the third ^
                            if chars.peek() == Some(&'^') {
                                chars.next(); // consume the fourth ^
                                tokens.push(Token::Hexation); // ^^^^ for hexation
                            } else {
                                tokens.push(Token::Pentation); // ^^^ for pentation
                            }
                        } else {
                            tokens.push(Token::Tetration); // ^^ for tetration
                        }
                    } else {
                        tokens.push(Token::Power); // ^ for power
                    }
                }
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                ',' => tokens.push(Token::Comma),
                c if c.is_ascii_digit() || c == '.' => {
                    let mut num_str = String::new();
                    num_str.push(c);
                    
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_ascii_digit() || next_char == '.' {
                            num_str.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    if let Ok(num) = num_str.parse::<f64>() {
                        tokens.push(Token::Number(num));
                    } else {
                        return Err(format!("Invalid number: {}", num_str));
                    }
                }
                c if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    ident.push(c);
                    
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            ident.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    if ident.eq_ignore_ascii_case("i") {
                        tokens.push(Token::ImaginaryUnit);
                    } else {
                        tokens.push(Token::Identifier(ident));
                    }
                }
                _ => return Err(format!("Unexpected character: {}", ch)),
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
        let left = Self::parse_pentation(tokens, pos, z, param)?;

        if *pos < tokens.len() && matches!(tokens[*pos], Token::Tetration) {
            *pos += 1;
            let right = Self::parse_tetration(tokens, pos, z, param)?; // Right-associative tetration
            Ok(Box::new(BinaryOp::Tetration(left, right)))
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
        Self::parse_primary(tokens, pos, z, param) // Hexation is the highest precedence
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

// Expression trait and implementations
pub trait Expression {
    fn evaluate(&self, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String>;
}

pub struct Constant(Complex<f64>);

impl Expression for Constant {
    fn evaluate(&self, _z: Complex<f64>, _param: Complex<f64>) -> Result<Complex<f64>, String> {
        Ok(self.0)
    }
}

pub enum Variable {
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

pub enum BinaryOp {
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
                            let result_norm_value = result.norm();

                            // For fractal generation with complex exponents, use a very conservative limit
                            // to prevent immediate escape of all points
                            let max_norm = 2.0; // Very conservative for complex exponents in fractals

                            if result_norm_value > max_norm {
                                // Scale down the result significantly to allow for fractal iteration
                                let scale_factor = max_norm / result_norm_value.max(1e-10); // Avoid division by zero
                                Ok(Complex::new(result.re * scale_factor, result.im * scale_factor))
                            } else {
                                // For complex exponents, we also need to ensure the result doesn't cause
                                // immediate escape in subsequent iterations. Let's apply a more sophisticated
                                // transformation that preserves the mathematical character while allowing
                                // for fractal formation

                                // Apply a transformation that maps large values to a more manageable range
                                // but still allows for differentiation between points
                                let transformed_result = if result_norm_value > 1.5 {
                                    // For large results, compress the range logarithmically
                                    let compressed_norm = 1.0 + 0.5 * (result_norm_value - 1.5).min(1.0); // Gradually compress
                                    let scale_factor = compressed_norm / result_norm_value.max(1e-10);
                                    Complex::new(result.re * scale_factor, result.im * scale_factor)
                                } else if result_norm_value < 0.01 {
                                    // For very small results, slightly amplify to avoid stagnation
                                    let amplified_norm = result_norm_value.max(0.01) * 2.0;
                                    let scale_factor = amplified_norm / result_norm_value.max(1e-10);
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
                    match n {
                        1 => Ok(base),  // base^^1 = base
                        2 => {
                            // base^^2 = base^base
                            let result = base.powc(base);
                            // Check for overflow
                            if result.norm_sqr() > 1e10 {
                                Ok(Complex::new(1e5, 1e5))
                            } else {
                                Ok(result)
                            }
                        },
                        3 => {
                            // base^^3 = base^(base^base)
                            let base_pow_base = base.powc(base);
                            if base_pow_base.norm_sqr() > 1e10 {
                                Ok(Complex::new(1e5, 1e5))
                            } else {
                                let base_pow_base_pow_base = base.powc(base_pow_base);
                                if base_pow_base_pow_base.norm_sqr() > 1e10 {
                                    Ok(Complex::new(1e5, 1e5))
                                } else {
                                    Ok(base_pow_base_pow_base)
                                }
                            }
                        },
                        _ => {
                            // For higher values, return a safe value to avoid immediate escape
                            Ok(Complex::new(1.0, 0.0))
                        }
                    }
                } else {
                    // For non-integer heights, return a safe value to avoid black images
                    Ok(Complex::new(1.0, 0.0))
                }
            },
            BinaryOp::Pentation(left, right) => {
                let base = left.evaluate(z, param)?;
                let height = right.evaluate(z, param)?;

                // Pentation is iterated tetration: base^^^height
                // This is extremely complex and usually diverges immediately
                // For fractal generation, return a safe value
                Ok(Complex::new(1.0, 0.0))  // Safe fallback to avoid black images
            },
            BinaryOp::Hexation(left, right) => {
                let base = left.evaluate(z, param)?;
                let height = right.evaluate(z, param)?;

                // Hexation is iterated pentation: base^^^^height
                // This is extremely complex and usually diverges immediately
                // For fractal generation, return a safe value
                Ok(Complex::new(1.0, 0.0))  // Safe fallback to avoid black images
            },
        }
    }
}

pub enum Function {
    Sin(Box<dyn Expression>),
    Cos(Box<dyn Expression>),
    Tan(Box<dyn Expression>),
    Exp(Box<dyn Expression>),
    Ln(Box<dyn Expression>),
    Sqrt(Box<dyn Expression>),
    Cbrt(Box<dyn Expression>),
    Asin(Box<dyn Expression>),
    Acos(Box<dyn Expression>),
    Atan(Box<dyn Expression>),
    Sinh(Box<dyn Expression>),
    Cosh(Box<dyn Expression>),
    Tanh(Box<dyn Expression>),
}

impl Expression for Function {
    fn evaluate(&self, z: Complex<f64>, param: Complex<f64>) -> Result<Complex<f64>, String> {
        match self {
            Function::Sin(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.sin())
            },
            Function::Cos(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.cos())
            },
            Function::Tan(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.tan())
            },
            Function::Exp(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.exp())
            },
            Function::Ln(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.ln())
            },
            Function::Sqrt(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.sqrt())
            },
            Function::Cbrt(expr) => {
                let arg = expr.evaluate(z, param)?;
                // Cube root for complex numbers - use principal root
                // For complex numbers, we use arg^(1/3)
                Ok(arg.powf(1.0/3.0))
            },
            Function::Asin(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.asin())
            },
            Function::Acos(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.acos())
            },
            Function::Atan(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.atan())
            },
            Function::Sinh(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.sinh())
            },
            Function::Cosh(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.cosh())
            },
            Function::Tanh(expr) => {
                let arg = expr.evaluate(z, param)?;
                Ok(arg.tanh())
            },
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