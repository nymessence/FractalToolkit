# Fractal Toolkit

A powerful and flexible fractal generation toolkit that supports complex mathematical operations, hyperoperations, and customizable rendering parameters.

## Features

### Advanced Mathematical Operations
- **Complex Exponents**: Support for formulas like `z^(2.7+0.3i) + c`
- **Real Non-Integer Exponents**: Handle expressions like `z^2.5 + c`
- **Hyperoperations**: 
  - Tetration (`z^^w`) - iterated exponentiation
  - Pentation (`z^^^w`) - iterated tetration
  - Hexation (`z^^^^w`) - iterated pentation
- **Trigonometric Functions**: sin, cos, tan, asin, acos, atan
- **Hyperbolic Functions**: sinh, cosh, tanh
- **Root Functions**: sqrt, cbrt
- **Logarithmic Functions**: ln, log

### Flexible Formula System
- Custom formulas with variables `z` and `c`
- Support for complex number literals like `2.7+0.3i`
- Parentheses for grouping operations
- Standard arithmetic operators (+, -, *, /, ^)

### High-Quality Rendering
- Configurable dimensions and bounds
- Custom color palettes with gradient support
- Adjustable iteration counts and bailout thresholds
- PNG output with HTML explorer

## Installation

```bash
# Clone the repository
git clone https://github.com/username/fractal-toolkit.git
cd fractal-toolkit

# Build the project
cargo build --release

# Install dependencies
chmod +x install.sh
./install.sh
```

## Usage

### Basic Mandelbrot
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2 + c'
```

### Complex Exponent Fractal
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c'
```

### Tetration-Based Fractal
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=256,256 --formula='z^^z + c' --max-iterations=32
```

### Custom Color Palette
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^3 + c' --color-pallette='[(#000000,0.0),(#FF0000,0.33),(#00FF00,0.66),(#FFFFFF,1.0)]'
```

## Available Commands

### ftk-mandel
Main command for generating Mandelbrot-style fractals with custom formulas.

#### Options
- `--bounds=x_min,x_max,y_min,y_max`: Set coordinate bounds (default: -2,2,-2,2)
- `--dimensions=width,height`: Set output dimensions (default: 512,512)
- `--max-iterations=n`: Maximum iterations per pixel (default: 100)
- `--spawn=real,imag`: Starting point (default: 0,0)
- `--color-pallette='[(hex_color,position),...]`: Color palette definition
- `--bailout=value`: Escape threshold (default: 4)
- `--formula='expression'`: Custom formula (default: 'z^2 + c')
- `--output='filename.png'`: Output filename

## Examples

### Real Non-Integer Exponents
```bash
# 2.5 exponent
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --formula='z^2.5 + c'

# 3.7 exponent
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --formula='z^3.7 + c'
```

### Complex Exponents
```bash
# Complex exponent
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --formula='z^(3.2-1.4i) + c'

# Pure imaginary exponent
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --formula='z^(0+2.5i) + c'
```

### Trigonometric Functions
```bash
# Sine-based fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --formula='sin(z) + c'

# Hyperbolic cosine fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --formula='cosh(z) + c'
```

### Hyperoperations
```bash
# Pentation-based fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=128,128 --formula='z^^^z + c' --max-iterations=16
```

## Documentation

For detailed documentation, see the files in the `docs/` directory:
- `COMMANDS.md` - Detailed command reference
- `EXAMPLES.md` - Practical examples and use cases
- `Monetizing_Fractal_Renders.md` - Commercial applications

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.