use clap::Parser;
use fractal_toolkit::{FractalParams, julia_iterations, pixel_to_complex, ColorPoint, generate_html_file};
use num_complex::Complex;
use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::Write;

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
}

fn main() {
    let args = Args::parse();

    println!("Generating Julia set with:");
    println!("  Bounds: {:?}", args.bounds);
    println!("  Max iterations: {}", args.max_iterations);
    println!("  Dimensions: {:?}", args.dimensions);
    println!("  Spawn: {:?}", args.spawn);
    println!("  Formula: {}", args.formula);
    println!("  Bailout: {}", args.bailout);
    println!("  Output: {}", args.output);

    if let Some(palette) = args.color_pallette {
        println!("  Color palette: {}", palette);
    }

    // Validate dimensions
    if args.dimensions.len() != 2 {
        eprintln!("Error: dimensions must have exactly 2 values [width, height]");
        std::process::exit(1);
    }

    if args.bounds.len() != 4 {
        eprintln!("Error: bounds must have exactly 4 values [x_min, x_max, y_min, y_max]");
        std::process::exit(1);
    }

    let width = args.dimensions[0];
    let height = args.dimensions[1];
    let bounds = [args.bounds[0], args.bounds[1], args.bounds[2], args.bounds[3]];

    // Create fractal parameters
    let formula_clone = args.formula.clone();
    let params = FractalParams::new(
        bounds,
        args.max_iterations,
        [args.spawn[0], args.spawn[1]],
        args.bailout,
        formula_clone,
    );

    // Generate the fractal image
    let img = generate_julia_image(width, height, &params);

    // Save the image
    img.save(&args.output).expect("Failed to save image");
    println!("Julia set image saved to {}", args.output);

    // Generate command template for the HTML
    let command_template = format!(
        "ftk-julia --bounds {{bounds}} --dimensions {{dimensions}} --max-iterations {} --spawn {},{} --bailout {} --formula \"{}\" --output \"julia_zoom_$(date +%Y%m%d_%H%M%S).png\"",
        args.max_iterations,
        args.spawn[0],
        args.spawn[1],
        args.bailout,
        args.formula.clone()
    );

    // Generate the HTML file
    if let Err(e) = generate_html_file(&args.output, bounds, [width, height], &command_template) {
        eprintln!("Error generating HTML file: {}", e);
    } else {
        println!("HTML explorer saved to {}",
                 std::path::Path::new(&args.output).with_extension("html").display());
    }
}

fn generate_julia_image(width: u32, height: u32, params: &FractalParams) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let z = pixel_to_complex(x, y, width, height, params.bounds);
        let iterations = julia_iterations(z, params);

        // Simple coloring based on iterations
        let color = color_from_iterations(iterations, params.max_iterations);
        *pixel = color;
    }

    imgbuf
}

// Simple function to convert iterations to a color
fn color_from_iterations(iterations: u32, max_iterations: u32) -> Rgba<u8> {
    if iterations == max_iterations {
        // Inside the set - black
        Rgba([0, 0, 0, 255])
    } else {
        // Outside the set - color based on iterations
        let t = iterations as f64 / max_iterations as f64;
        let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
        let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
        let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
        Rgba([r, g, b, 255])
    }
}