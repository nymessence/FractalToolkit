# Fractal Toolkit

A comprehensive Rust-based command-line tool for generating beautiful fractal images, including Mandelbrot, Julia sets, and Buddhabrot variants.

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Usage](#usage)
   - [Mandelbrot Generator](#mandelbrot-generator)
   - [Julia Set Generator](#julia-set-generator)
   - [Buddhabrot Generator](#buddhabrot-generator)
   - [Buddhabrot Julia Generator](#buddhabrot-julia-generator)
4. [Parameters Explained](#parameters-explained)
5. [Advanced Usage](#advanced-usage)
6. [Interactive HTML Explorer](#interactive-html-explorer)
7. [Development](#development)

## Features

- **Multiple Fractal Types**: Generate Mandelbrot sets, Julia sets, Buddhabrot, and Buddhabrot Julia fractals
- **High-Quality Output**: PNG image generation with customizable dimensions
- **RGB Channel Control**: For Buddhabrot variants, individual control over red, green, and blue channels with different iteration ranges
- **Interactive HTML Explorer**: Click-and-drag selection with command generation for re-rendering
- **Customizable Parameters**: Extensive control over bounds, iterations, spawn points, formulas, etc.
- **Performance Optimized**: Efficient algorithms for fast rendering

## Installation

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd fractal-toolkit

# Build the project
cargo build --release
```

The executables will be available in `target/release/`:
- `ftk-mandel` - Mandelbrot generator
- `ftk-julia` - Julia set generator
- `ftk-buddha` - Buddhabrot generator
- `ftk-buddhaj` - Buddhabrot Julia generator

### Automated Installation

The project includes an installation script that can build and install the binaries system-wide:

```bash
# Make the script executable
chmod +x install.sh

# Install to default location (/usr/local/bin)
sudo ./install.sh

# Install to custom location
sudo ./install.sh --prefix=/opt

# Install debug version
sudo ./install.sh --debug

# Uninstall the toolkit
sudo ./install.sh --uninstall
```

The installation script will:
- Check for Rust and Cargo dependencies
- Build the project in release mode (or debug mode if specified)
- Install all four binaries to the system
- Verify the installation

## Usage

### Mandelbrot Generator

Generate classic Mandelbrot set fractals:

```bash
ftk-mandel --bounds -2.0,2.0,-2.0,2.0 --dimensions 1024,1024 --output mandel.png
```

#### Mandelbrot Options

- `--bounds <x_min,x_max,y_min,y_max>`: Viewport bounds in the complex plane
- `--dimensions <width,height>`: Output image dimensions in pixels
- `--max-iterations <N>`: Maximum iterations before assuming point is in the set (default: 64)
- `--spawn <real,imag>`: Spawn point for Julia sets (ignored for Mandelbrot)
- `--formula <formula>`: Fractal formula (default: "z^2 + c")
- `--bailout <value>`: Escape radius threshold (default: 4.0)
- `--output <filename>`: Output filename (default: mandel_output.png)

### Julia Set Generator

Generate Julia set fractals with customizable spawn points:

```bash
ftk-julia --bounds -2.0,2.0,-2.0,2.0 --dimensions 1024,1024 --spawn 0.285,0.01 --output julia.png
```

#### Julia Set Options

- `--bounds <x_min,x_max,y_min,y_max>`: Viewport bounds in the complex plane
- `--dimensions <width,height>`: Output image dimensions in pixels
- `--max-iterations <N>`: Maximum iterations before assuming point is in the set (default: 64)
- `--spawn <real,imag>`: Constant c value for Julia set formula z² + c (default: 0.0,0.0)
- `--formula <formula>`: Fractal formula (default: "z^2 + c")
- `--bailout <value>`: Escape radius threshold (default: 4.0)
- `--output <filename>`: Output filename (default: julia_output.png)

### Buddhabrot Generator

Generate Buddhabrot fractals with RGB channel control:

```bash
ftk-buddha --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --samples 100000 \
  --red-channel 10,50,50000 --green-channel 51,100,25000 --blue-channel 101,200,10000 \
  --output buddha.png
```

#### Buddhabrot Options

- `--bounds <x_min,x_max,y_min,y_max>`: Viewport bounds in the complex plane
- `--dimensions <width,height>`: Output image dimensions in pixels
- `--min-iterations <N>`: Minimum iterations for points to be considered (default: 10)
- `--max-iterations <N>`: Maximum iterations to check (default: 100)
- `--samples <N>`: Number of random samples to take (default: 1000000)
- `--bailout <value>`: Escape radius threshold (default: 4.0)
- `--formula <formula>`: Fractal formula (default: "z^2 + c")
- `--red-channel <min_iter,max_iter,samples>`: Red channel configuration
- `--green-channel <min_iter,max_iter,samples>`: Green channel configuration
- `--blue-channel <min_iter,max_iter,samples>`: Blue channel configuration
- `--output <filename>`: Output filename (default: buddha_output.png)

### Buddhabrot Julia Generator

Generate Buddhabrot Julia fractals with RGB channel control:

```bash
ftk-buddhaj --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --spawn 0.285,0.01 --samples 100000 \
  --red-channel 10,50,50000 --green-channel 51,100,25000 --blue-channel 101,200,10000 \
  --output buddhaj.png
```

#### Buddhabrot Julia Options

- `--bounds <x_min,x_max,y_min,y_max>`: Viewport bounds in the complex plane
- `--dimensions <width,height>`: Output image dimensions in pixels
- `--min-iterations <N>`: Minimum iterations for points to be considered (default: 10)
- `--max-iterations <N>`: Maximum iterations to check (default: 100)
- `--samples <N>`: Number of random samples to take (default: 1000000)
- `--bailout <value>`: Escape radius threshold (default: 4.0)
- `--formula <formula>`: Fractal formula (default: "z^2 + c")
- `--spawn <real,imag>`: Constant c value for Julia set (default: 0.0,0.0)
- `--red-channel <min_iter,max_iter,samples>`: Red channel configuration
- `--green-channel <min_iter,max_iter,samples>`: Green channel configuration
- `--blue-channel <min_iter,max_iter,samples>`: Blue channel configuration
- `--output <filename>`: Output filename (default: buddhaj_output.png)

## Parameters Explained

### Bounds Parameter

The `--bounds` parameter defines the rectangular region of the complex plane to render:
- Format: `x_min,x_max,y_min,y_max`
- Example: `-2.0,2.0,-2.0,2.0` renders from -2-2i to 2+2i
- The complex plane has real values on x-axis and imaginary values on y-axis

### Dimensions Parameter

The `--dimensions` parameter sets the output image size:
- Format: `width,height`
- Example: `1024,1024` creates a 1024×1024 pixel image
- Larger images take more time to render

### Iterations

Iterations control the level of detail in fractals:
- Higher values reveal more fine structure but take longer to compute
- For Mandelbrot/Julia: maximum iterations before declaring a point as "in the set"
- For Buddhabrot: minimum and maximum iteration ranges for different channels

### RGB Channels (Buddhabrot Variants)

Each channel (red, green, blue) can be configured independently:
- Format: `min_iteration,max_iteration,sample_count`
- Different iteration ranges highlight different structures
- More samples = higher quality but longer render time
- Example: `10,50,50000` considers points with 10-50 iterations, using 50,000 samples

### Spawn Points

For Julia sets and Buddhabrot Julia, spawn points define the constant c in z² + c:
- Format: `real,imaginary`
- Different spawn points create dramatically different fractal structures
- Example: `0.285,0.01` creates a classic Julia set pattern

## Advanced Usage

### Custom Formulas

The Fractal Toolkit now supports complex mathematical expressions including:

- **Complex Powers**: `z^2.5`, `z^(-2+3.5*i)`
- **Complex Coefficients**: `3*i`, `(2+1.5*i)`
- **Nested Functions**: `sin(cos(z))`, `exp(sin(z) * cos(z))`
- **Complex Exponents**: `z^(2+3*i)`
- **Tetration**: `z^^n` (iterated exponentiation)
- **Special Functions**: `gamma(z)`, `zeta(z)`

#### Examples of Complex Formulas

```bash
# Sine of complex power
ftk-mandel --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --formula "sin(z^2.5)" --output mandel_sin_complex_power.png

# Complex base and exponent
ftk-julia --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --spawn 0.5,0.5 --formula "3*i + z^(-2+3.5*i)" --output julia_complex_base_exp.png

# Nested trigonometric functions
ftk-mandel --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --formula "exp(sin(z) * cos(z))" --output mandel_nested_trig.png

# Complex polynomial
ftk-julia --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --spawn 1.0,0.5 --formula "(2+1.5*i)*z^3 + (1-0.5*i)*z^2 + z + (0.5+1*i)" --output julia_complex_poly.png

# Tetration (iterated exponentiation)
ftk-mandel --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --formula "z^^2" --output mandel_tetration2.png

# Special functions
ftk-julia --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --spawn 0.5,0.5 --formula "gamma(z)" --output julia_gamma.png
```

### High-Resolution Rendering

For high-resolution images, consider:
- Using fewer samples per channel initially to test parameters
- Rendering in sections and stitching together
- Increasing system resources (RAM) for large images

### Performance Tips

- Start with small images (256×256) to test parameters
- Reduce sample counts for Buddhabrot variants during testing
- Use lower iteration counts initially
- Consider rendering specific regions of interest rather than full views

### Example Commands

#### Standard Mandelbrot
```bash
ftk-mandel --bounds -2.0,2.0,-2.0,2.0 --dimensions 1024,1024 --max-iterations 100 --output mandel_full.png
```

#### Detailed Region
```bash
ftk-mandel --bounds -0.8,0.2,0.2,1.2 --dimensions 1024,1024 --max-iterations 200 --output mandel_detail.png
```

#### Classic Julia Set
```bash
ftk-julia --bounds -2.0,2.0,-2.0,2.0 --dimensions 1024,1024 --spawn 0.285,0.01 --max-iterations 100 --output julia_classic.png
```

#### Artistic Buddhabrot
```bash
ftk-buddha --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --samples 500000 \
  --red-channel 10,50,200000 --green-channel 51,100,100000 --blue-channel 101,200,50000 \
  --output buddha_artistic.png
```

## Interactive HTML Explorer

Each generated PNG file comes with a companion HTML file featuring:

- **Interactive Selection**: Click and drag to select a region of interest
- **Aspect Ratio Controls**: Radio buttons for common aspect ratios (1:1, 3:2, 2:3, 4:3, 3:4, 16:9, 9:16)
- **Resolution Selector**: Dropdown menu with common resolutions
- **Command Generation**: Automatically generates the command to render the selected region
- **Real-time Preview**: Visual feedback during selection

The HTML explorer enables iterative exploration of fractal regions without manual parameter calculation.

## Development

### Project Structure

```
fractal-toolkit/
├── Cargo.toml          # Project manifest and dependencies
├── README.md          # This file
├── src/
│   ├── lib.rs         # Core algorithms and utilities
│   ├── main.rs        # Main entry point (currently empty)
│   └── bin/           # Individual executables
│       ├── ftk-mandel.rs     # Mandelbrot generator
│       ├── ftk-julia.rs      # Julia set generator
│       ├── ftk-buddha.rs     # Buddhabrot generator
│       └── ftk-buddhaj.rs    # Buddhabrot Julia generator
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Architecture

The toolkit follows a modular design:

- **Core Library** (`lib.rs`): Shared algorithms, data structures, and HTML generation
- **Executable Binaries**: Specialized interfaces for each fractal type
- **Parameter Structures**: Typed parameter containers for type safety
- **HTML Generation**: Dynamic HTML creation with embedded JavaScript

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

### License

MIT License