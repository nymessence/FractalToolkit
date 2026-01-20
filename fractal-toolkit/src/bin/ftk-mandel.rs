use clap::Parser;
use fractal_toolkit::{FractalParams, mandelbrot_iterations, generate_html_file, parse_color_palette, ColorStop, generate_fractal_image};
use image::{ImageBuffer, Rgba};
use rayon::ThreadPoolBuilder;
use num_complex::Complex;

fn init_rayon_pool() {
    let num_threads = num_cpus::get();
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to initialize Rayon thread pool");
}

// Helper function to parse a complex number from string
fn parse_complex_number(s: &str) -> Result<Complex<f64>, String> {
    let s = s.trim();

    // Handle simple cases first
    if s == "i" || s == "I" {
        return Ok(Complex::new(0.0, 1.0));
    } else if s == "-i" || s == "-I" {
        return Ok(Complex::new(0.0, -1.0));
    }

    // Try to parse as a real number
    if let Ok(real_val) = s.parse::<f64>() {
        return Ok(Complex::new(real_val, 0.0));
    }

    // Handle complex number format like "a+bi", "a-bi", "a+i", "a-i", etc.
    let s = s.replace(" ", "").replace("*", ""); // Remove spaces and multiplication symbols

    // Find all positions of + and - that are not at the beginning
    let mut plus_minus_positions = Vec::new();
    for (i, c) in s.char_indices() {
        if (c == '+' || c == '-') && i > 0 {
            plus_minus_positions.push(i);
        }
    }

    // Find the position of 'i' or 'I'
    let i_pos = s.find(|c| c == 'i' || c == 'I');

    if let Some(i_pos) = i_pos {
        // Complex number with imaginary part
        if plus_minus_positions.is_empty() {
            // Format like "ai" or "bi" where a or b is the coefficient
            let coeff_str = &s[..i_pos];
            if coeff_str.is_empty() || coeff_str == "+" {
                return Ok(Complex::new(0.0, 1.0)); // Just "i"
            } else if coeff_str == "-" {
                return Ok(Complex::new(0.0, -1.0)); // Just "-i"
            } else {
                let coeff = coeff_str.parse::<f64>()
                    .map_err(|_| format!("Invalid imaginary coefficient: {}", coeff_str))?;
                return Ok(Complex::new(0.0, coeff));
            }
        } else {
            // Complex number with both real and imaginary parts, like "a+bi" or "a-bi"
            // Find the last + or - before the i
            let mut last_sign_before_i = None;
            for &pos in plus_minus_positions.iter().rev() {
                if pos < i_pos {
                    last_sign_before_i = Some(pos);
                    break;
                }
            }

            let (real_part, imag_coeff) = if let Some(sign_pos) = last_sign_before_i {
                // Split at the last sign before i
                let real_str = &s[..sign_pos];
                let imag_str = &s[sign_pos..i_pos];

                let real_part = if real_str.is_empty() {
                    0.0
                } else {
                    real_str.parse::<f64>()
                        .map_err(|_| format!("Invalid real part: {}", real_str))?
                };

                let imag_coeff = if imag_str.is_empty() || imag_str == "+" {
                    1.0
                } else if imag_str == "-" {
                    -1.0
                } else {
                    imag_str.parse::<f64>()
                        .map_err(|_| format!("Invalid imaginary coefficient: {}", imag_str))?
                };

                (real_part, imag_coeff)
            } else {
                // Format like "a i" or "bi" where i is preceded by a coefficient
                let real_str = &s[..i_pos];
                let real_part = real_str.parse::<f64>()
                    .map_err(|_| format!("Invalid real part: {}", real_str))?;
                (real_part, 1.0) // Assume coefficient of 1 if not specified
            };

            Ok(Complex::new(real_part, imag_coeff))
        }
    } else {
        // Just a real number (already handled above, but as a fallback)
        s.parse::<f64>()
            .map(|real_val| Complex::new(real_val, 0.0))
            .map_err(|_| format!("Invalid number: {}", s))
    }
}

#[derive(Parser)]
#[command(name = "ftk-mandel")]
#[command(version = "1.0")]
#[command(about = "Generates Mandelbrot fractal images")]
struct Args {
    /// Bounds of the fractal [x_min, x_max, y_min, y_max]
    #[arg(long, value_delimiter = ',', num_args = 1..=4)]
    bounds: Vec<f64>,

    /// Maximum number of iterations
    #[arg(long, default_value_t = 64)]
    max_iterations: u32,

    /// Dimensions of the output image [width, height]
    #[arg(long, value_delimiter = ',', num_args = 1..=2)]
    dimensions: Vec<u32>,

    /// Spawn point for the fractal [real, imag]
    #[arg(long, value_delimiter = ',', num_args = 1..=2, default_values_t = [0.0, 0.0])]
    spawn: Vec<f64>,

    /// Color palette [(hex_color, position), ...]
    #[arg(long)]
    color_pallette: Option<String>,

    /// Formula for the fractal
    #[arg(long, default_value = "z^2 + c")]
    formula: String,

    /// Bailout value
    #[arg(long, default_value_t = 4.0)]
    bailout: f64,

    /// Output file name
    #[arg(long, default_value = "mandel_output.png")]
    output: String,

    /// Custom imaginary unit value (i = sqrt of this value), defaults to -1 if unspecified
    #[arg(long, default_value = "-1")]
    i_sqrt_value: String,

    /// Enable orbit debugging to trace the iteration path for a specific point
    #[arg(long)]
    orbit_debug: bool,

    /// Point coordinates for orbit debugging [real, imag] (requires --orbit-debug)
    #[arg(long, value_delimiter = ',', num_args = 1..=2, default_values_t = [0.0, 0.0])]
    debug_point: Vec<f64>,

    /// Enable domain coloring mode to color points based on their final complex value
    #[arg(long)]
    domain_color: bool,

    /// Disable bailout threshold for fully domain-colored plots (use with --domain-color)
    #[arg(long)]
    no_bailout: bool,
}

fn main() {
    // Initialize rayon thread pool with CPU core count
    init_rayon_pool();

    let args = Args::parse();

    println!("Generating Mandelbrot set with:");
    println!("  Bounds: {:?}", args.bounds);
    println!("  Max iterations: {}", args.max_iterations);
    println!("  Dimensions: {:?}", args.dimensions);
    println!("  Spawn: {:?}", args.spawn);
    println!("  Formula: {}", args.formula);
    println!("  Bailout: {}", args.bailout);
    println!("  Output: {}", args.output);

    if let Some(ref palette) = args.color_pallette {
        println!("  Color palette: {}", palette);
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

    if args.bounds.len() != 4 {
        eprintln!("Error: bounds must have exactly 4 values [x_min, x_max, y_min, y_max]");
        std::process::exit(1);
    }
    let bounds = [args.bounds[0], args.bounds[1], args.bounds[2], args.bounds[3]];

    // Parse the custom i_sqrt_value
    let i_sqrt_complex = parse_complex_number(&args.i_sqrt_value).unwrap_or_else(|e| {
        eprintln!("Error parsing i_sqrt_value '{}': {}", args.i_sqrt_value, e);
        eprintln!("Using default (0,1) for standard i (i² = -1)");
        num_complex::Complex::new(0.0, 1.0)
    });

    // Create fractal parameters
    let formula_clone = args.formula.clone();
    let mut params = FractalParams::new(
        bounds,
        args.max_iterations,
        [args.spawn[0], args.spawn[1]],
        args.bailout,
        formula_clone,
    );
    params.i_sqrt_value = i_sqrt_complex;

    // If orbit debugging is enabled, trace the orbit for a specific point
    if args.orbit_debug {
        // Validate debug point
        if args.debug_point.len() != 2 {
            eprintln!("Error: debug-point must have exactly 2 values [real, imag]");
            std::process::exit(1);
        }

        // Use the specified debug point
        let debug_point = num_complex::Complex::new(args.debug_point[0], args.debug_point[1]);
        println!("Orbit debug for point: {:?}", debug_point);
        fractal_toolkit::trace_orbit_mandelbrot(debug_point, &params);
        return; // Exit after debugging
    }

    // Parse color palette if provided
    let color_palette = if let Some(ref palette_str) = args.color_pallette {
        match parse_color_palette(palette_str) {
            Ok(palette) => {
                println!("Using color palette with {} stops", palette.len());
                Some(palette)
            },
            Err(e) => {
                eprintln!("Error parsing color palette: {}", e);
                eprintln!("Using default coloring instead.");
                None
            }
        }
    } else {
        None
    };

    // Generate the fractal image
    let img = if args.domain_color {
        // Use domain coloring mode
        fractal_toolkit::generate_mandelbrot_domain_color_image(width, height, &params, args.no_bailout, color_palette.as_ref())
    } else {
        generate_mandelbrot_image(width, height, &params, color_palette.as_ref())
    };

    // Save the image
    img.save(&args.output).expect("Failed to save image");
    println!("Mandelbrot image saved to {}", args.output);

    // Generate command template for the HTML
    let command_template = if let Some(ref palette) = args.color_pallette {
        format!(
            "ftk-mandel --bounds={{bounds}} --dimensions={{dimensions}} --max-iterations={} --spawn={},{} --color-pallette=\"{}\" --bailout={} --formula=\"{}\" --output=\"mandel_zoom_$(date +%Y%m%d_%H%M%S).png\"",
            args.max_iterations,
            args.spawn[0],
            args.spawn[1],
            palette,
            args.bailout,
            args.formula.clone()
        )
    } else {
        format!(
            "ftk-mandel --bounds={{bounds}} --dimensions={{dimensions}} --max-iterations={} --spawn={},{} --bailout={} --formula=\"{}\" --output=\"mandel_zoom_$(date +%Y%m%d_%H%M%S).png\"",
            args.max_iterations,
            args.spawn[0],
            args.spawn[1],
            args.bailout,
            args.formula.clone()
        )
    };

    // Generate the HTML file
    if let Err(e) = generate_html_file(&args.output, bounds, [width, height], &command_template) {
        eprintln!("Error generating HTML file: {}", e);
    } else {
        println!("HTML explorer saved to {}",
                 std::path::Path::new(&args.output).with_extension("html").display());
    }
}

fn generate_mandelbrot_image(width: u32, height: u32, params: &FractalParams, color_palette: Option<&Vec<ColorStop>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    generate_fractal_image(width, height, params, |c, p| mandelbrot_iterations(c, p), color_palette)
}