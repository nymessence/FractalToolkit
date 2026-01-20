use num_complex::Complex;

/// Utility functions for fractal generation
pub mod utils {
    use super::*;

    /// Convert pixel coordinates to complex plane coordinates
    pub fn pixel_to_complex(x: u32, y: u32, width: u32, height: u32, bounds: [f64; 4]) -> Complex<f64> {
        let [x_min, x_max, y_min, y_max] = bounds;
        
        let real = x_min + (x as f64 / width as f64) * (x_max - x_min);
        let imag = y_min + (y as f64 / height as f64) * (y_max - y_min);
        
        Complex::new(real, imag)
    }

    /// Parse a complex number from a string representation
    /// Supports formats like: "1", "i", "-i", "2i", "1+2i", "1-2i", etc.
    pub fn parse_complex_number(s: &str) -> Result<Complex<f64>, String> {
        let s = s.trim();
        
        // Handle special cases
        if s == "i" {
            return Ok(Complex::new(0.0, 1.0));
        } else if s == "-i" {
            return Ok(Complex::new(0.0, -1.0));
        }
        
        // Handle pure real numbers
        if let Ok(real_val) = s.parse::<f64>() {
            return Ok(Complex::new(real_val, 0.0));
        }
        
        // Handle pure imaginary numbers like "2i", "-3i", etc.
        if s.ends_with('i') || s.ends_with('I') {
            let coeff_str = &s[..s.len()-1]; // Remove the 'i'
            if let Ok(coeff) = coeff_str.parse::<f64>() {
                return Ok(Complex::new(0.0, coeff));
            }
        }
        
        // Handle complex numbers in the form "a+bi", "a-bi", etc.
        // This is a simplified parser - a full implementation would be more complex
        // For now, we'll handle the most common cases
        
        // Look for + or - that's not at the beginning (indicating the real/imaginary separator)
        let mut plus_minus_pos = None;
        for (i, c) in s.char_indices() {
            if (c == '+' || c == '-') && i > 0 {
                plus_minus_pos = Some(i);
                break;
            }
        }
        
        if let Some(pos) = plus_minus_pos {
            let real_part = &s[..pos];
            let imag_part = &s[pos..];
            
            // Remove the 'i' from the imaginary part if present
            let imag_part_clean = if imag_part.ends_with('i') || imag_part.ends_with('I') {
                &imag_part[..imag_part.len()-1]
            } else {
                imag_part
            };
            
            let real_val = if real_part.is_empty() {
                0.0
            } else {
                real_part.parse::<f64>().map_err(|_| format!("Invalid real part: {}", real_part))?
            };
            
            let imag_val = if imag_part_clean.is_empty() || imag_part_clean == "+" {
                1.0
            } else if imag_part_clean == "-" {
                -1.0
            } else {
                imag_part_clean.parse::<f64>().map_err(|_| format!("Invalid imaginary part: {}", imag_part_clean))?
            };
            
            return Ok(Complex::new(real_val, imag_val));
        }
        
        Err(format!("Unable to parse complex number: {}", s))
    }
}