use clap::Parser;
use fractal_toolkit::{BuddhabrotParams, BuddhabrotChannels, BuddhabrotChannel, generate_buddhabrot, generate_html_file};

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
}

fn main() {
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
    
    // Create Buddhabrot parameters
    let params = BuddhabrotParams::new(
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