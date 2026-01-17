# Fractal Toolkit Examples

This document provides practical examples of how to use the Fractal Toolkit to generate various types of fractals with different formulas and parameters.

## Basic Examples

### Classic Mandelbrot Set
Generate the classic Mandelbrot set with default parameters:
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2 + c'
```

### Adjusting Resolution
Generate a higher resolution fractal:
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=1024,1024 --formula='z^2 + c' --output='high_res_mandelbrot.png'
```

### Custom Color Palette
Apply a custom color palette to your fractal:
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2 + c' --color-pallette='[(#000000,0.0),(#FF0000,0.33),(#00FF00,0.66),(#FFFFFF,1.0)]'
```

## Advanced Formula Examples

### Real Non-Integer Exponents
Generate fractals with real non-integer exponents:
```bash
# Cubic root-like fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2.5 + c'

# Higher real exponent
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^3.7 + c'

# Fractional exponent
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^1.5 + c'
```

### Complex Exponents
Generate fractals with complex exponents:
```bash
# Complex exponent fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c'

# Another complex exponent
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(3.2-1.4i) + c'

# Pure imaginary exponent
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(0+2.5i) + c'
```

### Trigonometric Functions
Use trigonometric functions in your formulas:
```bash
# Sine-based fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='sin(z) + c'

# Cosine-based fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='cos(z) + c'

# Tangent-based fractal (may require adjusted bailout)
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='tan(z) + c' --bailout=16
```

### Hyperbolic Functions
Use hyperbolic functions in your formulas:
```bash
# Hyperbolic sine fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='sinh(z) + c'

# Hyperbolic cosine fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='cosh(z) + c'

# Hyperbolic tangent fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='tanh(z) + c'
```

### Root Functions
Use root functions in your formulas:
```bash
# Square root fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='sqrt(z) + c'

# Cube root fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='cbrt(z) + c'
```

### Inverse Trigonometric Functions
Use inverse trigonometric functions:
```bash
# Arc sine fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='asin(z) + c'

# Arc cosine fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='acos(z) + c'

# Arc tangent fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='atan(z) + c'
```

### Hyperoperations
Use higher hyperoperations in your formulas:
```bash
# Tetration-based fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=256,256 --formula='z^^z + c' --max-iterations=32

# Pentation-based fractal (more complex, may require adjustment)
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=128,128 --formula='z^^^z + c' --max-iterations=16

# Hexation-based fractal (very complex)
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=64,64 --formula='z^^^^z + c' --max-iterations=8
```

## Parameter Tuning Examples

### Adjusting Iterations
For more detailed fractals, increase iterations:
```bash
# More detailed fractal
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^2.5 + c' --max-iterations=200
```

### Adjusting Bailout Value
Change the bailout threshold for different effects:
```bash
# Lower bailout for tighter fractals
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^3 + c' --bailout=2

# Higher bailout for looser fractals
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c' --bailout=32
```

### Zooming In
Focus on specific regions of the fractal:
```bash
# Zoom into the spiral area of Mandelbrot
cargo run --bin ftk-mandel -- --bounds=-0.1,0.1,-0.1,0.1 --dimensions=512,512 --formula='z^2 + c' --max-iterations=500

# Zoom into a complex exponent fractal
cargo run --bin ftk-mandel -- --bounds=-0.5,0.5,-0.5,0.5 --dimensions=512,512 --formula='z^(2.7+0.3i) + c' --max-iterations=200
```

## Combining Features

### Complex Formula with Custom Palette
Combine multiple features in one command:
```bash
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=1024,1024 --formula='z^(3.2-1.4i) + c' --max-iterations=150 --bailout=16 --color-pallette='[(#000000,0.0),(#FF0000,0.25),(#FFFF00,0.5),(#00FF00,0.75),(#FFFFFF,1.0)]' --output='complex_fractal.png'
```

### Testing New Formulas
Quick test with low resolution:
```bash
# Quick test of a new formula
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=64,64 --formula='sinh(z) + sqrt(z) + c' --max-iterations=32
```

## Troubleshooting Examples

### Handling Black Images
If you get a black image with complex exponents, try:
```bash
# Increase bailout value
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c' --bailout=64

# Reduce iterations
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=512,512 --formula='z^(2.7+0.3i) + c' --max-iterations=16
```

### Performance Optimization
For faster testing:
```bash
# Low resolution test
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=32,32 --formula='z^^z + c' --max-iterations=8

# Then scale up once satisfied
cargo run --bin ftk-mandel -- --bounds=-2,2,-2,2 --dimensions=256,256 --formula='z^^z + c' --max-iterations=32
```