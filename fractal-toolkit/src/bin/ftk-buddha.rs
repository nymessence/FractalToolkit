use clap::Parser;
use fractal_toolkit::{BuddhabrotParams, BuddhabrotChannels, BuddhabrotChannel, generate_buddhabrot, generate_html_file};
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
#[command(name = "ftk-buddha")]
#[command(version = "1.0")]
#[command(about = "Generates Buddhabrot fractal images")]
struct Args {
    /// Bounds of the fractal [x_min, x_max, y_min, y_max] (default: [-2.0, 1.0, -1.2, 1.2] for Mandelbrot region)
    #[arg(long, value_delimiter = ',', num_args = 1..=4, default_values_t = [-2.0, 1.0, -1.2, 1.2])]
    bounds: Vec<f64>,
    
    /// Dimensions of the output image [width, height]
    #[arg(long, value_delimiter = ',', num_args = 1..=2)]
    dimensions: Vec<u32>,
    
    /// Minimum iterations for points to be considered
    #[arg(long, default_value_t = 10)]
    min_iterations: u32,
    
    /// Maximum iterations to check
    #[arg(long, default_value_t = 100)]
    max_iterations: u32,
    
    /// Number of random samples to take
    #[arg(long, default_value_t = 1000000)]
    samples: u64,
    
    /// Bailout value
    #[arg(long, default_value_t = 4.0)]
    bailout: f64,
    
    /// Formula for the fractal
    #[arg(long, default_value = "z^2 + c")]
    formula: String,
    
    /// Red channel: min_iter,max_iter,samples
    #[arg(long, value_delimiter = ',', num_args = 1..=3)]
    red_channel: Vec<u64>,
    
    /// Green channel: min_iter,max_iter,samples
    #[arg(long, value_delimiter = ',', num_args = 1..=3)]
    green_channel: Vec<u64>,
    
    /// Blue channel: min_iter,max_iter,samples
    #[arg(long, value_delimiter = ',', num_args = 1..=3)]
    blue_channel: Vec<u64>,
    
    /// Output file name
    #[arg(long, default_value = "buddha_output.png")]
    output: String,

    /// Custom imaginary unit value (i = sqrt of this value), defaults to -1 if unspecified
    #[arg(long, default_value = "-1")]
    i_sqrt_value: String,
}

fn main() {
    // Initialize rayon thread pool with CPU core count
    init_rayon_pool();

    let args = Args::parse();

    println!("Generating Buddhabrot with:");
    println!("  Bounds: {:?}", args.bounds);
    println!("  Dimensions: {:?}", args.dimensions);
    println!("  Min iterations: {}", args.min_iterations);
    println!("  Max iterations: {}", args.max_iterations);
    println!("  Samples: {}", args.samples);
    println!("  Bailout: {}", args.bailout);
    println!("  Formula: {}", args.formula);
    println!("  Red channel: {:?}", args.red_channel);
    println!("  Green channel: {:?}", args.green_channel);
    println!("  Blue channel: {:?}", args.blue_channel);
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

    if width == 0 || height == 0 {
        eprintln!("Error: dimensions must be greater than 0");
        std::process::exit(1);
    }

    // Validate channel parameters
    if args.red_channel.len() != 3 {
        eprintln!("Error: red-channel must have exactly 3 values [min_iter, max_iter, samples]");
        std::process::exit(1);
    }
    
    if args.green_channel.len() != 3 {
        eprintln!("Error: green-channel must have exactly 3 values [min_iter, max_iter, samples]");
        std::process::exit(1);
    }
    
    if args.blue_channel.len() != 3 {
        eprintln!("Error: blue-channel must have exactly 3 values [min_iter, max_iter, samples]");
        std::process::exit(1);
    }
    
    let width = args.dimensions[0];
    let height = args.dimensions[1];
    let bounds = [args.bounds[0], args.bounds[1], args.bounds[2], args.bounds[3]];
    
    // Create channel configurations
    let red_channel = BuddhabrotChannel {
        min_iter: args.red_channel[0] as u32,
        max_iter: args.red_channel[1] as u32,
        samples: args.red_channel[2],
    };
    
    let green_channel = BuddhabrotChannel {
        min_iter: args.green_channel[0] as u32,
        max_iter: args.green_channel[1] as u32,
        samples: args.green_channel[2],
    };
    
    let blue_channel = BuddhabrotChannel {
        min_iter: args.blue_channel[0] as u32,
        max_iter: args.blue_channel[1] as u32,
        samples: args.blue_channel[2],
    };
    
    // Parse the custom i_sqrt_value
    let i_sqrt_complex = parse_complex_number(&args.i_sqrt_value).unwrap_or_else(|_| {
        eprintln!("Error parsing i_sqrt_value, using default (0,1) for standard i");
        Complex::new(0.0, 1.0)
    });

    // Create Buddhabrot parameters
    let mut params = BuddhabrotParams::new(
        bounds,
        width,
        height,
        args.min_iterations,
        args.max_iterations,
        args.samples,
        args.bailout,
        args.formula,
        BuddhabrotChannels {
            red: red_channel.clone(),
            green: green_channel.clone(),
            blue: blue_channel.clone(),
        },
    );
    params.i_sqrt_value = i_sqrt_complex;
    
    // Generate the Buddhabrot image
    let img = generate_buddhabrot(&params);
    
    // Save the image
    img.save(&args.output).expect("Failed to save image");
    println!("Buddhabrot image saved to {}", args.output);
    
    // Generate command template for the HTML
    let command_template = format!(
        "ftk-buddha --bounds={{bounds}} --dimensions={{dimensions}} --min-iterations={} --max-iterations={} --samples={} --bailout={} --formula=\"{}\" --red-channel={},{},{} --green-channel={},{},{} --blue-channel={},{},{} --output=\"buddha_zoom_$(date +%Y%m%d_%H%M%S).png\"",
        args.min_iterations,
        args.max_iterations,
        args.samples,
        args.bailout,
        params.formula,
        red_channel.min_iter, red_channel.max_iter, red_channel.samples,
        green_channel.min_iter, green_channel.max_iter, green_channel.samples,
        blue_channel.min_iter, blue_channel.max_iter, blue_channel.samples
    );
    
    // Generate the HTML file
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

        // Split on '+' or '-' but preserve the signs
        let mut real_str = "";
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