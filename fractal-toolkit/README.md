# Fractal Toolkit

A Rust-based command-line tool for generating beautiful fractal images, including Mandelbrot and Julia sets.

## Features

- Generate Mandelbrot and Julia set fractals
- Highly customizable parameters (bounds, iterations, spawn points, formulas, etc.)
- High-quality PNG output
- Interactive HTML explorer with:
  - Click-and-drag selection box
  - Aspect ratio controls (1:1, 3:2, 2:3, 4:3, 3:4, 16:9, 9:16)
  - Resolution selector
  - Command generation for selected regions

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd fractal-toolkit

# Build the project
cargo build --release
```

## Usage

### Mandelbrot Generator

```bash
ftk-mandel --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --max-iterations 64 --output mandel.png
```

### Julia Set Generator

```bash
ftk-julia --bounds -2.0,2.0,-2.0,2.0 --dimensions 512,512 --spawn 0.285,0.01 --max-iterations 64 --output julia.png
```

## Parameters

- `--bounds`: Fractal bounds as [x_min, x_max, y_min, y_max] (comma-separated)
- `--dimensions`: Output image dimensions as [width, height] (comma-separated)
- `--max-iterations`: Maximum number of iterations (default: 64)
- `--spawn`: Spawn point for Julia sets as [real, imag] (comma-separated)
- `--formula`: Fractal formula (default: "z^2 + c")
- `--bailout`: Bailout value (default: 4.0)
- `--output`: Output file name (default: mandel_output.png or julia_output.png)
- `--color-pallette`: Color palette (not yet implemented)

## Example Commands

Generate a standard Mandelbrot set:
```bash
ftk-mandel --bounds -2.0,2.0,-2.0,2.0 --dimensions 1024,1024 --output mandel.png
```

Generate a Julia set with a specific spawn point:
```bash
ftk-julia --bounds -2.0,2.0,-2.0,2.0 --dimensions 1024,1024 --spawn -0.7,0.27015 --output julia.png
```

Generate a cubic Mandelbrot variant:
```bash
ftk-mandel --bounds -1.5,1.5,-1.5,1.5 --dimensions 512,512 --formula "z^3 + c" --output cubic_mandel.png
```

## Interactive HTML Explorer

Each generated PNG file comes with a companion HTML file that allows you to:

1. Click and drag on the fractal image to select a region
2. Choose from multiple aspect ratios using radio buttons
3. Select different output resolutions
4. Get the command to generate the selected region

The HTML file will be automatically created with the same name as the PNG but with a `.html` extension.

## Development

To run tests:
```bash
cargo test
```

To build in development mode:
```bash
cargo build
```

To build in release mode:
```bash
cargo build --release
```

## License

MIT License