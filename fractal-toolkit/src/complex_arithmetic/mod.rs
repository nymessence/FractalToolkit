use num_complex::Complex;

/// Custom complex number system with configurable imaginary unit
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
        // Ensure both operands have the same i_squared value for consistency
        assert_eq!(self.i_squared, other.i_squared, "Cannot multiply custom complex numbers with different i² values");

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
        let bd_i_squared = bd * self.i_squared;

        // Real part: ac + Re(bd * i²)
        let real_part = ac + bd_i_squared.re;
        // Imaginary part: (ad + bc) + Im(bd * i²)
        let imag_part = (ad + bc) + bd_i_squared.im;

        Self {
            re: real_part,
            im: imag_part,
            i_squared: self.i_squared,  // Maintain the same i_squared
        }
    }

    /// Custom addition
    pub fn add(&self, other: &Self) -> Self {
        // Addition is the same regardless of the imaginary unit: (a + bi) + (c + di) = (a+c) + (b+d)i
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
            i_squared: self.i_squared,  // Maintain the same i_squared
        }
    }

    /// Custom subtraction
    pub fn subtract(&self, other: &Self) -> Self {
        // Subtraction is the same regardless of the imaginary unit: (a + bi) - (c + di) = (a-c) + (b-d)i
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

    /// Get the argument (angle) of the complex number
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Get the norm (magnitude) of the complex number
    pub fn norm(&self) -> f64 {
        self.norm_sqr().sqrt()
    }
}

/// Helper function to compute custom complex multiplication with custom imaginary unit
/// (a + bi) * (c + di) = ac + ad*i + bc*i + bd*i^2 where i^2 is the custom value
pub fn custom_complex_multiply(z1: Complex<f64>, z2: Complex<f64>, i_squared: Complex<f64>) -> Complex<f64> {
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
pub fn custom_complex_square(z: Complex<f64>, i_squared: Complex<f64>) -> Complex<f64> {
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