use clap::Parser;
use fractal_toolkit::{FractalParams, mandelbrot_iterations, pixel_to_complex, generate_html_file, parse_color_palette, ColorStop, generate_fractal_image};
use image::{ImageBuffer, Rgba};

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
}

fn main() {
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

    // Create fractal parameters
    let formula_clone = args.formula.clone();
    let params = FractalParams::new(
        bounds,
        args.max_iterations,
        [args.spawn[0], args.spawn[1]],
        args.bailout,
        formula_clone,
    );

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
    let img = generate_mandelbrot_image(width, height, &params, color_palette.as_ref());

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