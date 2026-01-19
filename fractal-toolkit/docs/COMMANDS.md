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
- `--i-sqrt-value='complex_value'`: Custom imaginary unit value (i = sqrt of this value), defaults to -1 if unspecified (default: -1)
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
- `z^(2.7+0.3i) + c` with `--i-sqrt-value='-i'` - Complex exponent with custom imaginary unit where i² = -i
- `z^(2.7+0.3i) + c` with `--i-sqrt-value='1-i'` - Complex exponent with custom imaginary unit where i² = 1-i
- `z^(2.7+0.3i) + c` with `--i-sqrt-value='0.5+0.5i'` - Complex exponent with custom imaginary unit where i² = 0.5+0.5i
- `z^2 + c` with `--i-sqrt-value='0'` - Edge case where i² = 0 (collapses imaginary component)
- `z^2 + c` with `--i-sqrt-value='1'` - Split-complex numbers where i² = 1
- `z^^z + c` - Tetration-based fractal
- `z^^^z + c` - Pentation-based fractal
- `sqrt(z) + c` - Square root fractal
- `sin(z) + c` - Sine-based fractal

## Custom Imaginary Unit (--i-sqrt-value)

The `--i-sqrt-value` parameter allows users to define the value that the imaginary unit i is the square root of. By default, i² = -1 (standard complex numbers), but users can now set i² to other values like -i, 1-i, or any complex number.

### Mathematical Implications
- When `--i-sqrt-value='-1'` (default): Standard complex numbers where i² = -1
- When `--i-sqrt-value='-i'`: Alternative number system where i² = -i
- When `--i-sqrt-value='1-i'`: Alternative number system where i² = 1-i
- When `--i-sqrt-value='0.5+0.5i'`: Alternative number system where i² = 0.5+0.5i
- When `--i-sqrt-value='0'`: Degenerate case where i² = 0 (imaginary component collapses)
- When `--i-sqrt-value='1'`: Split-complex numbers where i² = 1

This creates entirely new classes of fractals with different mathematical properties and visual characteristics.

## Examples

### Basic Mandelbrot
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2 + c'
```

### Real Non-Integer Exponent
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2.5 + c'
```

### Complex Exponent with Standard Imaginary Unit
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c'
```

### Complex Exponent with Custom Imaginary Unit (i² = -i)
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c' --i-sqrt-value='-i'
```

### Complex Exponent with Custom Imaginary Unit (i² = 1-i)
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c' --i-sqrt-value='1-i'
```

### Complex Exponent with Custom Imaginary Unit (i² = 0.5+0.5i)
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c' --i-sqrt-value='0.5+0.5i'
```

### Split-Complex Numbers (i² = 1)
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2 + c' --i-sqrt-value='1'
```

### Tetration with Custom Imaginary Unit
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=256,256 --formula='z^^z + c' --i-sqrt-value='0.5+0.5i' --max-iterations=16
```