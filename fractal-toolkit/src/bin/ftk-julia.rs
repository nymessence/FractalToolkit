use clap::Parser;
use fractal_toolkit::{FractalParams, julia_iterations, generate_html_file, parse_color_palette, ColorStop, generate_fractal_image};
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

#[derive(Parser)]
#[command(name = "ftk-julia")]
#[command(version = "1.0")]
#[command(about = "Generates Julia set fractal images")]
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
    #[arg(long, default_value = "julia_output.png")]
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
}

fn main() {
    // Initialize rayon thread pool with CPU core count
    init_rayon_pool();

    let args = Args::parse();

    println!("Generating Julia set with:");
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
    let i_sqrt_complex = parse_complex_number(&args.i_sqrt_value).unwrap_or_else(|_| {
        eprintln!("Error parsing i_sqrt_value, using default (0,1) for standard i");
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
        fractal_toolkit::trace_orbit_julia(debug_point, &params);
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
    let img = generate_julia_image(width, height, &params, color_palette.as_ref());

    // Save the image
    img.save(&args.output).expect("Failed to save image");
    println!("Julia set image saved to {}", args.output);

    // Generate command template for the HTML
    let command_template = if let Some(ref palette) = args.color_pallette {
        format!(
            "ftk-julia --bounds={{bounds}} --dimensions={{dimensions}} --max-iterations={} --spawn={},{} --color-pallette=\"{}\" --bailout={} --formula=\"{}\" --output=\"julia_zoom_$(date +%Y%m%d_%H%M%S).png\"",
            args.max_iterations,
            args.spawn[0],
            args.spawn[1],
            palette,
            args.bailout,
            args.formula.clone()
        )
    } else {
        format!(
            "ftk-julia --bounds={{bounds}} --dimensions={{dimensions}} --max-iterations={} --spawn={},{} --bailout={} --formula=\"{}\" --output=\"julia_zoom_$(date +%Y%m%d_%H%M%S).png\"",
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

fn generate_julia_image(width: u32, height: u32, params: &FractalParams, color_palette: Option<&Vec<ColorStop>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    generate_fractal_image(width, height, params, |z, p| julia_iterations(z, p), color_palette)
}