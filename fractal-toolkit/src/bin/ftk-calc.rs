use clap::Parser;
use fractal_toolkit::{MathEvaluator, parse_complex_number};
use num_complex::Complex;

#[derive(Parser)]
#[command(name = "ftk-calc")]
#[command(version = "1.0")]
#[command(about = "A complex number calculator for testing fractal functions and hyperoperations")]
struct Args {
    /// The mathematical expression to evaluate
    #[arg()]
    expression: String,

    /// Point coordinates for evaluation [real, imag] (default: [0, 0])
    #[arg(long, value_delimiter = ',', num_args = 1..=2, default_values_t = [0.0, 0.0])]
    point: Vec<f64>,

    /// Parameter value for the expression [real, imag] (default: [0, 0])
    #[arg(long, value_delimiter = ',', num_args = 1..=2, default_values_t = [0.0, 0.0])]
    param: Vec<f64>,

    /// Custom imaginary unit value (i = sqrt of this value), defaults to -1 if unspecified
    #[arg(long, default_value = "-1")]
    i_sqrt_value: String,

    /// Enable verbose output to show detailed computation steps
    #[arg(long)]
    verbose: bool,

    /// Enable multivalue mode to evaluate the expression with multiple values
    #[arg(long, value_delimiter = ',', num_args = 1..=3)]
    multivalue: Option<Vec<f64>>,
}

fn main() {
    let args = Args::parse();

    if let Some(ref multivalues) = args.multivalue {
        // Multivalue mode: evaluate expression with multiple values
        if multivalues.len() < 2 {
            eprintln!("Error: multivalue must have at least 2 values [start, end, step_size?]");
            std::process::exit(1);
        }

        let start = multivalues[0];
        let end = multivalues[1];
        let step = if multivalues.len() > 2 { multivalues[2] } else { 1.0 };

        if args.verbose {
            println!("Multivalue evaluation from {} to {} with step {}", start, end, step);
        }

        let mut current = start;
        while current <= end {
            // Replace 'n' in the expression with the current value
            let expr_with_n = args.expression.replace("n", &format!("{}", current));

            // Create complex numbers from point and param
            if args.point.len() != 2 || args.param.len() != 2 {
                eprintln!("Error: point and param must each have exactly 2 values [real, imag]");
                std::process::exit(1);
            }

            let z = Complex::new(args.point[0], args.point[1]);
            let param_complex = Complex::new(args.param[0], args.param[1]);

            // Parse the custom i_sqrt_value
            let i_sqrt_complex = parse_complex_number(&args.i_sqrt_value).unwrap_or_else(|_| {
                eprintln!("Error parsing i_sqrt_value, using default (0,1) for standard i");
                Complex::new(0.0, 1.0)
            });

            match MathEvaluator::evaluate_formula_with_param_and_custom_i(&expr_with_n, z, param_complex, i_sqrt_complex) {
                Ok(result) => {
                    if args.verbose {
                        println!("n = {}: {} = ({:.6}, {:.6})", current, expr_with_n, result.re, result.im);
                    } else {
                        println!("n = {}: ({:.6}, {:.6})", current, result.re, result.im);
                    }
                },
                Err(e) => {
                    eprintln!("Error evaluating '{}': {}", expr_with_n, e);
                }
            }

            current += step;
        }
    } else {
        // Single value mode: evaluate the expression once
        if args.point.len() != 2 || args.param.len() != 2 {
            eprintln!("Error: point and param must each have exactly 2 values [real, imag]");
            std::process::exit(1);
        }

        let z = Complex::new(args.point[0], args.point[1]);
        let param = Complex::new(args.param[0], args.param[1]);

        // Parse the custom i_sqrt_value
        let i_sqrt_complex = parse_complex_number(&args.i_sqrt_value).unwrap_or_else(|_| {
            eprintln!("Error parsing i_sqrt_value, using default (0,1) for standard i");
            Complex::new(0.0, 1.0)
        });

        if args.verbose {
            println!("Evaluating expression: {}", args.expression);
            println!("  Point z: {:?}", z);
            println!("  Parameter: {:?}", param);
            println!("  Custom i² value: {:?}", i_sqrt_complex);
        }

        match MathEvaluator::evaluate_formula_with_param_and_custom_i(&args.expression, z, param, i_sqrt_complex) {
            Ok(result) => {
                if args.verbose {
                    println!("Result: z = ({:.6}, {:.6}), |z| = {:.6}, arg = {:.6}", 
                             result.re, result.im, result.norm(), result.arg());
                } else {
                    println!("({:.6}, {:.6})", result.re, result.im);
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}