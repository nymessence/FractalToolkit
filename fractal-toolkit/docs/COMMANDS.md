# Fractal Toolkit Commands Documentation

## Overview
The Fractal Toolkit provides a comprehensive suite of commands for generating and exploring fractals with customizable formulas and parameters.

## Available Commands

### ftk-mandel
Generates Mandelbrot-style fractals with custom formulas.

#### Usage
```bash
cargo run --bin ftk-mandel -- [OPTIONS]
```

#### Options
- `--bounds=x_min,x_max,y_min,y_max`: Set the coordinate bounds for the fractal view (default: -2,2,-2,2)
- `--dimensions=width,height`: Set the dimensions of the output image in pixels (default: 512,512)
- `--max-iterations=n`: Maximum number of iterations per pixel (default: 100)
- `--spawn=real,imag`: Starting point for the fractal calculation (default: 0,0)
- `--color-pallette='[(hex_color,position),...]`: Define the color palette for the fractal
- `--bailout=value`: Threshold for escaping iteration (default: 4)
- `--formula='expression'`: Custom formula for the fractal (default: 'z^2 + c')
- `--output='filename.png'`: Output filename for the generated image

#### Supported Operators
- `+` - Addition
- `-` - Subtraction
- `*` - Multiplication
- `/` - Division
- `^` - Power (exponentiation)
- `^^` - Tetration (hyper-4 operation)
- `^^^` - Pentation (hyper-5 operation)
- `^^^^` - Hexation (hyper-6 operation)

#### Supported Functions
- `abs(x)` - Absolute value
- `sqrt(z)` - Square root
- `cbrt(z)` - Cube root
- `sin(z)` - Sine
- `cos(z)` - Cosine
- `tan(z)` - Tangent
- `asin(z)` - Arc sine
- `acos(z)` - Arc cosine
- `atan(z)` - Arc tangent
- `sinh(z)` - Hyperbolic sine
- `cosh(z)` - Hyperbolic cosine
- `tanh(z)` - Hyperbolic tangent
- `exp(z)` - Exponential function
- `ln(z)` - Natural logarithm
- `log(base, z)` - Logarithm with specified base
- `conjugate(z)` - Complex conjugate

#### Formula Examples
- `z^2 + c` - Classic Mandelbrot set
- `z^2.5 + c` - Real non-integer exponent
- `z^(2.7+0.3i) + c` - Complex exponent
- `z^3 + c` - Cubic Mandelbrot
- `z^^z + c` - Tetration-based fractal
- `z^^^z + c` - Pentation-based fractal
- `sqrt(z) + c` - Square root fractal
- `sin(z) + c` - Sine-based fractal

## Mathematical Operations

### Power Operations
- Standard power: `z^n` where n can be integer, real, or complex
- Complex exponents: `z^(a+bi)` where a and b are real numbers
- Real non-integer exponents: `z^2.7`, `z^3.14`, etc.

### Hyperoperations
- Tetration: `z^^w` - iterated exponentiation
- Pentation: `z^^^w` - iterated tetration
- Hexation: `z^^^^w` - iterated pentation

### Complex Functions
- Trigonometric: `sin(z)`, `cos(z)`, `tan(z)`
- Inverse trigonometric: `asin(z)`, `acos(z)`, `atan(z)`
- Hyperbolic: `sinh(z)`, `cosh(z)`, `tanh(z)`
- Roots: `sqrt(z)`, `cbrt(z)`
- Logarithmic: `ln(z)`, `log(base, z)`
- Exponential: `exp(z)`

## Color Palettes
Color palettes are defined as a list of tuples in the format `(hex_color, position)`, where:
- `hex_color` is a color in hexadecimal format (e.g., #FF0000 for red)
- `position` is a float between 0.0 and 1.0 indicating the position in the gradient

Example:
```
--color-pallette='[(#000000,0.0),(#00FF00,0.5),(#FFFFFF,1.0)]'
```

## Troubleshooting

### Common Issues
- **Black Images**: May occur with certain complex exponents that cause immediate escape. Try adjusting bailout value or max iterations.
- **Performance**: High iteration counts or complex formulas may take longer to compute.
- **Invalid Syntax**: Ensure formulas follow proper mathematical syntax with balanced parentheses.

### Tips
- Start with lower resolution (e.g., 64x64) for testing new formulas
- Use moderate max-iteration values initially (e.g., 32-100)
- Complex exponents may require higher bailout values to visualize properly
- Non-integer and complex exponents often produce interesting and unique fractal patterns

## Examples

### Basic Mandelbrot
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2 + c'
```

### Real Non-Integer Exponent
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2.5 + c'
```

### Complex Exponent
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c'
```

### Tetration-Based Fractal
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=256,256 --formula='z^^z + c' --max-iterations=16
```

### Custom Color Palette
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --color-pallette='[(#000000,0.0),(#FF0000,0.33),(#00FF00,0.66),(#FFFFFF,1.0)]' --formula='z^3 + c'
```