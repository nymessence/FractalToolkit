use num_complex::Complex;

/// Debug function to trace orbits for z^z + c with different imaginary units
fn debug_orbit(formula: &str, z_init: Complex<f64>, c: Complex<f64>, custom_i: Complex<f64>, max_iter: u32, bailout: f64) {
    println!("Formula: {}", formula);
    println!("Initial z: {:?}", z_init);
    println!("Parameter c: {:?}", c);
    println!("Custom i (where i² = {:?}): {:?}", custom_i, custom_i * custom_i);
    println!("Max iterations: {}, Bailout: {}", max_iter, bailout);
    println!();

    let mut z = z_init;
    for iter in 0..max_iter {
        // Apply the formula z^z + c
        // For z^z, we use the formula z^z = exp(z * ln(z))
        let ln_z = Complex::new(z.norm().ln(), z.arg());  // ln(z) = ln(|z|) + i*arg(z)
        let z_ln_z = z * ln_z;  // z * ln(z)
        let z_pow_z = z_ln_z.exp();  // exp(z * ln(z)) = z^z
        z = z_pow_z + c;  // z^z + c

        println!("Iteration {}: z = ({:.6}, {:.6}), |z| = {:.6}", iter + 1, z.re, z.im, z.norm());

        if z.norm_sqr() > bailout * bailout {
            println!("Point escapes at iteration {}", iter + 1);
            break;
        }

        if iter >= max_iter - 1 {
            println!("Point remains bounded after {} iterations", max_iter);
        }
    }
    println!("\n---\n");
}

fn main() {
    // Test with standard complex numbers (i² = -1)
    println!("=== STANDARD COMPLEX (i² = -1) ===");
    debug_orbit(
        "z^z + c", 
        Complex::new(-0.75, 0.1),  // A point in the complex plane
        Complex::new(-0.75, 0.1),  // Same as c parameter
        Complex::new(0.0, 1.0),    // Standard i where i² = -1
        16,                        // Max iterations
        4.0                        // Bailout
    );

    // Test with split complex numbers (i² = 1)
    println!("=== SPLIT COMPLEX (i² = 1) ===");
    debug_orbit(
        "z^z + c", 
        Complex::new(-0.75, 0.1),  // A point in the complex plane
        Complex::new(-0.75, 0.1),  // Same as c parameter
        Complex::new(1.0, 0.0),    // Custom i where i² = 1
        16,                        // Max iterations
        4.0                        // Bailout
    );

    // Test with another custom value
    println!("=== CUSTOM VALUE (i² = 1+i) ===");
    debug_orbit(
        "z^z + c", 
        Complex::new(-0.75, 0.1),  // A point in the complex plane
        Complex::new(-0.75, 0.1),  // Same as c parameter
        Complex::new(1.0, 1.0),    // Custom i where i² = 1+i
        16,                        // Max iterations
        4.0                        // Bailout
    );
}