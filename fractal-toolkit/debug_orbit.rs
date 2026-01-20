use num_complex::Complex;

/// Debugging function to trace orbit behavior with custom imaginary unit
fn debug_orbit_iteration(z_init: Complex<f64>, c: Complex<f64>, custom_i: Complex<f64>, max_iter: u32, formula: &str) {
    println!("Debugging orbit for formula: {}", formula);
    println!("Initial z: {:?}", z_init);
    println!("Parameter c: {:?}", c);
    println!("Custom i (where i² = {:?}): {:?}", custom_i, custom_i * custom_i);
    println!();

    let mut z = z_init;
    for iter in 0..max_iter {
        // Simple z^2 + c iteration for debugging
        z = z * z + c;
        
        println!("Iteration {}: z = {:?}, |z| = {:.6}", iter + 1, z, z.norm());
        
        if z.norm_sqr() > 16.0 {  // Standard bailout
            println!("Point escapes at iteration {}", iter + 1);
            break;
        }
        
        if iter >= max_iter - 1 {
            println!("Point remains bounded after {} iterations", max_iter);
        }
    }
    println!();
}

fn main() {
    // Test with standard complex numbers (i² = -1)
    println!("=== STANDARD COMPLEX (i² = -1) ===");
    debug_orbit_iteration(
        Complex::new(0.0, 0.0),  // Initial z
        Complex::new(-0.7269, 0.1889),  // Point in interesting region
        Complex::new(0.0, 1.0),  // Standard i where i² = -1
        16,  // Max iterations
        "z^2 + c"
    );
    
    // Test with split complex numbers (i² = 1)
    println!("=== SPLIT COMPLEX (i² = 1) ===");
    debug_orbit_iteration(
        Complex::new(0.0, 0.0),  // Initial z
        Complex::new(-0.7269, 0.1889),  // Point in interesting region
        Complex::new(1.0, 0.0),  // Custom i where i² = 1
        16,  // Max iterations
        "z^2 + c"
    );
    
    // Test with another custom value (i² = -i)
    println!("=== CUSTOM IMAGINARY UNIT (i² = -i) ===");
    debug_orbit_iteration(
        Complex::new(0.0, 0.0),  // Initial z
        Complex::new(-0.7269, 0.1889),  // Point in interesting region
        Complex::new(0.0, -1.0),  // Custom i where i² = -i (since i*(-i) = -i² = -(-1) = 1... wait, that's not right)
        16,  // Max iterations
        "z^2 + c"
    );
    
    // Actually, if we want i² = -i, we need to find what value satisfies that
    // For i² = a + bi, we need to solve (x + yi)² = a + bi
    // (x + yi)² = x² - y² + 2xyi = a + bi
    // So x² - y² = a and 2xy = b
    // For i² = -i = 0 + (-1)i, we have a=0, b=-1
    // So x² - y² = 0 and 2xy = -1
    // From x² = y² and 2xy = -1, we get x = ±y and 2x² = -1 or -2x² = -1
    // This gives us x² = 1/2, so x = ±√(1/2), y = ∓√(1/2) (opposite signs because 2xy = -1)
    let sqrt_half = (0.5f64).sqrt();
    let custom_i = Complex::new(sqrt_half, -sqrt_half);  // One solution to i² = -i
    println!("=== CUSTOM IMAGINARY UNIT (i² ≈ -i) ===");
    println!("Custom i value: {:?}", custom_i);
    println!("i² = {:?}", custom_i * custom_i);
    debug_orbit_iteration(
        Complex::new(0.0, 0.0),  // Initial z
        Complex::new(-0.7269, 0.1889),  // Point in interesting region
        custom_i,
        16,  // Max iterations
        "z^2 + c"
    );
}