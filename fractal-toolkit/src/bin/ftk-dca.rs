use clap::Parser;
use fractal_toolkit::{DomainColorParams, generate_domain_color_plot, generate_html_file};
use rayon::ThreadPoolBuilder;

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
    
    // Create domain color parameters
    let params = DomainColorParams {
        bounds,
        width,
        height,
        formula: args.formula,
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