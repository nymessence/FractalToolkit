use clap::Parser;
use fractal_toolkit::{DomainColorParams, generate_domain_color_plot, generate_html_file};
use rayon::ThreadPoolBuilder;
use num_complex::Complex;

fn init_rayon_pool() {
    let num_threads = num_cpus::get();
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to initialize Rayon thread pool");
}

#[derive(Parser)]
#[command(name = "ftk-dca")]
#[command(version = "1.0")]
#[command(about = "Generates domain color plots for complex functions")]
struct Args {
    /// Bounds of the complex plane [x_min, x_max, y_min, y_max]
    #[arg(long, value_delimiter = ',', num_args = 1..=4, default_values_t = [-2.0, 2.0, -2.0, 2.0])]
    bounds: Vec<f64>,
    
    /// Dimensions of the output image [width, height]
    #[arg(long, value_delimiter = ',', num_args = 1..=2, default_values_t = [1024, 1024])]
    dimensions: Vec<u32>,
    
    /// Formula for the complex function (e.g., "z^2", "sin(z)", "z^3 + z", etc.)
    #[arg(long, default_value = "z^2")]
    formula: String,
    
    /// Output file name
    #[arg(long, default_value = "domain_color_output.png")]
    output: String,

    /// Custom imaginary unit value (i = sqrt of this value), defaults to -1 if unspecified
    #[arg(long, default_value = "-1")]
    i_sqrt_value: String,

    /// Enable orbit debugging to trace the iteration path for a specific point
    #[arg(long)]
    orbit_debug: bool,
}

fn main() {
    // Initialize rayon thread pool with CPU core count
    init_rayon_pool();

    let args = Args::parse();

    println!("Generating domain color plot with:");
    println!("  Bounds: {:?}", args.bounds);
    println!("  Dimensions: {:?}", args.dimensions);
    println!("  Formula: {}", args.formula);
    println!("  Output: {}", args.output);
    
    // Validate bounds
    if args.bounds.len() != 4 {
        eprintln!("Error: bounds must have exactly 4 values [x_min, x_max, y_min, y_max]");
        std::process::exit(1);
    }
    
    // Validate dimensions
    if args.dimensions.len() != 2 {
        eprintln!("Error: dimensions must have exactly 2 values [width, height]");
        std::process::exit(1);
    }
    
    let width = args.dimensions[0];
    let height = args.dimensions[1];
    let bounds = [args.bounds[0], args.bounds[1], args.bounds[2], args.bounds[3]];
    
    // Parse the custom i_sqrt_value
    let i_sqrt_complex = parse_complex_number(&args.i_sqrt_value).unwrap_or_else(|_| {
        eprintln!("Error parsing i_sqrt_value, using default (0,1) for standard i");
        num_complex::Complex::new(0.0, 1.0)
    });

    // If orbit debugging is enabled, trace the orbit for a specific point
    if args.orbit_debug {
        // Use a central point in the view for debugging
        let debug_point = num_complex::Complex::new(0.0, 0.0);
        println!("Orbit debug for point: {:?}", debug_point);
        fractal_toolkit::trace_orbit_dca(debug_point, &args.formula, i_sqrt_complex);
        return; // Exit after debugging
    }

    // Create domain color parameters
    let params = DomainColorParams {
        bounds,
        width,
        height,
        formula: args.formula,
        i_sqrt_value: i_sqrt_complex,
    };
    
    // Generate the domain color plot
    let img = generate_domain_color_plot(&params);
    
    // Save the image
    img.save(&args.output).expect("Failed to save image");
    println!("Domain color plot saved to {}", args.output);
    
    // Generate command template for the HTML
    let command_template = format!(
        "ftk-dca --bounds={{bounds}} --dimensions={{dimensions}} --formula=\"{}\" --output=\"dca_zoom_$(date +%Y%m%d_%H%M%S).png\"",
        params.formula
    );
    
    // Generate the HTML file with axis marks
    if let Err(e) = generate_html_file(&args.output, bounds, [width, height], &command_template) {
        eprintln!("Error generating HTML file: {}", e);
    } else {
        println!("HTML explorer saved to {}", 
                 std::path::Path::new(&args.output).with_extension("html").display());
    }
}

// Helper function to parse a complex number from string
fn parse_complex_number(s: &str) -> Result<Complex<f64>, String> {
    let s = s.trim();

    // Handle simple cases first
    if s == "i" || s == "I" {
        return Ok(Complex::new(0.0, 1.0));
    }

    // Try to parse as a real number
    if let Ok(real_val) = s.parse::<f64>() {
        return Ok(Complex::new(real_val, 0.0));
    }

    // Handle complex number format like "a+bi", "a-bi", "a+b*i", etc.
    let mut real_part = 0.0;
    let mut imag_part = 0.0;

    // Check if it contains 'i' or 'I'
    if s.contains('i') || s.contains('I') {
        let s = s.replace(" ", ""); // Remove spaces
        let s = s.replace("*", ""); // Remove multiplication symbols

        // Handle cases like "i", "-i", "+i"
        if s == "i" || s == "+i" || s == "I" || s == "+I" {
            return Ok(Complex::new(0.0, 1.0));
        } else if s == "-i" || s == "-I" {
            return Ok(Complex::new(0.0, -1.0));
        }

        #[allow(unused_assignments)]
        let mut real_str = "";
        #[allow(unused_assignments)]
        let mut imag_str = "";

        // Find the position of the imaginary part
        if let Some(i_pos) = s.find(|c| c == 'i' || c == 'I') {
            let before_i = &s[..i_pos];

            // Look for the last occurrence of + or - before the i
            if let Some(last_sign_pos) = before_i.rfind(|c: char| c == '+' || c == '-') {
                if last_sign_pos == 0 {
                    // Starts with a sign, e.g., "-2.5i" or "+3.2i"
                    real_str = "0";
                    imag_str = &s;
                } else {
                    // Has both real and imaginary parts, e.g., "1.5+2.3i"
                    real_str = &s[..last_sign_pos];
                    imag_str = &s[last_sign_pos..i_pos];
                }
            } else {
                // Just an imaginary number, e.g., "2.5i"
                real_str = "0";
                imag_str = &s[..i_pos];
            }

            // Parse real part
            if !real_str.is_empty() {
                real_part = real_str.parse::<f64>().map_err(|_| format!("Invalid real part: {}", real_str))?;
            }

            // Parse imaginary part
            if !imag_str.is_empty() {
                if imag_str == "+" || imag_str == "" {
                    imag_part = 1.0;
                } else if imag_str == "-" {
                    imag_part = -1.0;
                } else {
                    imag_part = imag_str.parse::<f64>().map_err(|_| format!("Invalid imaginary part: {}", imag_str))?;
                }
            }
        } else {
            // Just a real number
            real_part = s.parse::<f64>().map_err(|_| format!("Invalid number: {}", s))?;
        }
    } else {
        // Just a real number
        real_part = s.parse::<f64>().map_err(|_| format!("Invalid number: {}", s))?;
    }

    Ok(Complex::new(real_part, imag_part))
}