use num_complex::Complex;

fn main() {
    // Test complex exponentiation with various points
    let exponent = Complex::new(2.7, 0.3); // (2.7+0.3i)
    
    // Test points within radius 2
    let test_points = vec![
        Complex::new(0.0, 0.0),      // origin
        Complex::new(1.0, 0.0),      // real axis
        Complex::new(0.0, 1.0),      // imaginary axis
        Complex::new(-1.0, 0.0),     // negative real
        Complex::new(0.0, -1.0),     // negative imaginary
        Complex::new(0.5, 0.5),      // first quadrant
        Complex::new(-0.5, 0.5),     // second quadrant
        Complex::new(-0.5, -0.5),    // third quadrant
        Complex::new(0.5, -0.5),     // fourth quadrant
        Complex::new(1.5, 0.8),      // outside unit circle
        Complex::new(-1.2, 0.9),     // more test points
        Complex::new(0.3, -1.1),     // another point
    ];
    
    println!("Testing complex exponentiation z^({}+{}i):", exponent.re, exponent.im);
    for point in test_points {
        // Calculate z^w = exp(w * ln(z))
        if point.norm_sqr() < 1e-10 {
            println!("z = ({:.3}, {:.3}): z^w = (0.000, 0.000) [base too small]", point.re, point.im);
            continue;
        }
        
        let r = point.norm();
        let theta = point.arg();
        let log_z = Complex::new(r.ln(), theta);
        let w_log_z = exponent * log_z;
        let result = w_log_z.exp();
        
        let z_to_w_plus_c = result + point;  // z^w + c where c is the point
        
        println!("z = ({:.3}, {:.3}): z^w = ({:.3}, {:.3}), |z^w| = {:.3}, z^w+c = ({:.3}, {:.3}), |z^w+c| = {:.3}", 
                 point.re, point.im, 
                 result.re, result.im, result.norm(),
                 z_to_w_plus_c.re, z_to_w_plus_c.im, z_to_w_plus_c.norm());
    }
}