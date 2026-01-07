# Fractal Toolkit Usage Examples

This document provides comprehensive usage examples for the Fractal Toolkit, demonstrating various ways to generate different types of fractals with different parameters.

## Basic Examples

### Mandelbrot Set Examples

#### Basic Mandelbrot
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.0,0.0 --formula="z^2 + c" --bailout=4.0 --output="mandel_basic.png"
```

#### High-Resolution Mandelbrot
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=128 --dimensions=1024,1024 --spawn=0.0,0.0 --formula="z^2 + c" --bailout=4.0 --output="mandel_high_res.png"
```

#### Mandelbrot with Custom Color Palette
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=100 --dimensions=512,512 --color-pallette="[(#FF0000,0.0),(#00FF00,0.5),(#0000FF,1.0)]" --formula="z^2 + c" --bailout=4.0 --output="mandel_colored.png"
```

#### Mandelbrot with Different Formula
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=80 --dimensions=512,512 --formula="sin(z) + c" --bailout=4.0 --output="mandel_sine.png"
```

#### Zoomed Mandelbrot (Seahorse Valley)
```bash
ftk-mandel --bounds=-0.75,-0.73,-0.1,-0.08 --max-iterations=256 --dimensions=800,600 --formula="z^2 + c" --bailout=4.0 --output="mandel_seahorse.png"
```

#### Zoomed Mandelbrot (Spiral Region)
```bash
ftk-mandel --bounds=-0.16,-0.14,1.025,1.045 --max-iterations=512 --dimensions=800,600 --formula="z^2 + c" --bailout=4.0 --output="mandel_spiral.png"
```

### Julia Set Examples

#### Basic Julia Set
```bash
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.285,0.01 --formula="z^2 + c" --bailout=4.0 --output="julia_basic.png"
```

#### Classic Julia Set (Rabbit)
```bash
ftk-julia --bounds=-1.5,1.5,-1.5,1.5 --max-iterations=100 --dimensions=1024,1024 --spawn=-0.12,0.74 --formula="z^2 + c" --bailout=4.0 --output="julia_rabbit.png"
```

#### Classic Julia Set (Dendrite)
```bash
ftk-julia --bounds=-1.5,1.5,-1.5,1.5 --max-iterations=100 --dimensions=1024,1024 --spawn=0.0,1.0 --formula="z^2 + c" --bailout=4.0 --output="julia_dendrite.png"
```

#### Julia Set with Different Formula
```bash
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=80 --dimensions=512,512 --spawn=0.0,1.0 --formula="z^3 + c" --bailout=4.0 --output="julia_cubic.png"
```

#### Julia Set with Custom Color Palette
```bash
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=128 --dimensions=1024,1024 --spawn=0.285,0.01 --color-pallette="[(#FF0000,0.0),(#FFFF00,0.25),(#00FF00,0.5),(#00FFFF,0.75),(#0000FF,1.0)]" --formula="z^2 + c" --bailout=4.0 --output="julia_rainbow.png"
```

### Buddhabrot Examples

#### Basic Buddhabrot
```bash
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=512,512 --min-iterations=10 --max-iterations=100 --samples=100000 --bailout=4.0 --formula="z^2 + c" --red-channel=10,50,50000 --green-channel=50,75,30000 --blue-channel=75,100,20000 --output="buddha_basic.png"
```

#### High-Resolution Buddhabrot
```bash
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=1024,1024 --min-iterations=20 --max-iterations=200 --samples=500000 --bailout=4.0 --formula="z^2 + c" --red-channel=20,100,200000 --green-channel=100,150,150000 --blue-channel=150,200,100000 --output="buddha_high_res.png"
```

#### Buddhabrot with Different Formula
```bash
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=512,512 --min-iterations=10 --max-iterations=100 --samples=100000 --bailout=4.0 --formula="z^3 + c" --red-channel=10,50,50000 --green-channel=50,75,30000 --blue-channel=75,100,20000 --output="buddha_cubic.png"
```

### Buddhabrot Julia Examples

#### Basic Buddhabrot Julia
```bash
ftk-buddhaj --bounds=-2.0,2.0,-2.0,2.0 --dimensions=512,512 --min-iterations=10 --max-iterations=100 --samples=100000 --bailout=4.0 --spawn=0.285,0.01 --formula="z^2 + c" --red-channel=10,50,50000 --green-channel=50,75,30000 --blue-channel=75,100,20000 --output="buddhaj_basic.png"
```

#### Buddhabrot Julia with Different Spawn Point
```bash
ftk-buddhaj --bounds=-2.0,2.0,-2.0,2.0 --dimensions=1024,1024 --min-iterations=20 --max-iterations=200 --samples=500000 --bailout=4.0 --spawn=-0.7,0.27015 --formula="z^2 + c" --red-channel=20,100,200000 --green-channel=100,150,150000 --blue-channel=150,200,100000 --output="buddhaj_custom.png"
```

## Advanced Examples

### Complex Color Palettes

#### Rainbow Color Palette
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=128 --dimensions=1024,1024 --color-pallette="[(#FF0000,0.0),(#FF7F00,0.16),(#FFFF00,0.33),(#00FF00,0.5),(#0000FF,0.66),(#4B0082,0.83),(#9400D3,1.0)]" --formula="z^2 + c" --bailout=4.0 --output="mandel_rainbow.png"
```

#### Monochrome Palette
```bash
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=100 --dimensions=512,512 --spawn=0.285,0.01 --color-pallette="[(#000000,0.0),(#444444,0.3),(#888888,0.6),(#CCCCCC,0.8),(#FFFFFF,1.0)]" --formula="z^2 + c" --bailout=4.0 --output="julia_monochrome.png"
```

### Different Mathematical Formulas

#### Exponential Mandelbrot
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=80 --dimensions=512,512 --formula="exp(z) + c" --bailout=4.0 --output="mandel_exp.png"
```

#### Logarithmic Mandelbrot
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=80 --dimensions=512,512 --formula="log(z) + c" --bailout=4.0 --output="mandel_log.png"
```

#### Tangent Julia
```bash
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.0,1.0 --formula="tan(z) + c" --bailout=4.0 --output="julia_tan.png"
```

#### Complex Formula
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=100 --dimensions=512,512 --formula="z^2 + c*sin(z)" --bailout=4.0 --output="mandel_complex.png"
```

### High-Quality Renderings

#### Ultra-High Resolution Mandelbrot
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=256 --dimensions=2048,2048 --formula="z^2 + c" --bailout=4.0 --output="mandel_4k.png"
```

#### Ultra-High Resolution Buddhabrot
```bash
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=2048,2048 --min-iterations=50 --max-iterations=500 --samples=2000000 --bailout=4.0 --formula="z^2 + c" --red-channel=50,200,800000 --green-channel=200,350,700000 --blue-channel=350,500,500000 --output="buddha_4k.png"
```

### Time-Stamped Outputs

#### Timestamped Mandelbrot
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=128 --dimensions=1024,1024 --formula="z^2 + c" --bailout=4.0 --output="mandel_$(date +%Y%m%d_%H%M%S).png"
```

#### Timestamped Julia Set
```bash
ftk-julia --bounds=-1.5,1.5,-1.5,1.5 --max-iterations=100 --dimensions=1024,1024 --spawn=-0.12,0.74 --formula="z^2 + c" --bailout=4.0 --output="julia_$(date +%Y%m%d_%H%M%S).png"
```

## Complex Examples with Multiple Parameters

### Detailed Zoom Example
```bash
ftk-mandel --bounds=-0.745,-0.743,0.110,0.112 --max-iterations=1024 --dimensions=1200,1200 --formula="z^2 + c" --bailout=4.0 --output="mandel_deep_zoom.png"
```

### Complex Buddhabrot with Fine-Tuned Channels
```bash
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=1024,1024 --min-iterations=10 --max-iterations=1000 --samples=1000000 --bailout=4.0 --formula="z^2 + c" --red-channel=10,100,400000 --green-channel=100,500,350000 --blue-channel=500,1000,250000 --output="buddha_detailed.png"
```

### Ultra Resolution Complex Buddhabrot with Fine-Tuned Channels
```bash
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=8192,8192 --min-iterations=10 --max-iterations=10000 --samples=1000000000 --bailout=4.0 --formula="z^2 + c" --red-channel=100,100,400000000 --green-channel=1000,5000,350000000 --blue-channel=5000,10000,250000000 --output="buddha_ultra_detailed.png"
```

### Buddhabrot Julia with High Iterations
```bash
ftk-buddhaj --bounds=-2.0,2.0,-2.0,2.0 --dimensions=1024,1024 --min-iterations=50 --max-iterations=500 --samples=1000000 --bailout=4.0 --spawn=0.285,0.01 --formula="z^2 + c" --red-channel=50,200,400000 --green-channel=200,350,350000 --blue-channel=350,500,250000 --output="buddhaj_detailed.png"
```

## Performance Considerations

### Fast Preview Render
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=32 --dimensions=256,256 --formula="z^2 + c" --bailout=4.0 --output="mandel_preview.png"
```

### Medium Quality Render
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=128 --dimensions=512,512 --formula="z^2 + c" --bailout=4.0 --output="mandel_medium.png"
```

### High Quality Render
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=512 --dimensions=2048,2048 --formula="z^2 + c" --bailout=4.0 --output="mandel_high_quality.png"
```

## Troubleshooting Examples

### Low Iterations for Simple Structures
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=16 --dimensions=512,512 --formula="z^2 + c" --bailout=4.0 --output="mandel_low_iter.png"
```

### High Bailout for Complex Structures
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=128 --dimensions=512,512 --formula="z^2 + c" --bailout=16.0 --output="mandel_high_bailout.png"
```

## Complex Mathematical Formulas

The Fractal Toolkit now supports complex mathematical expressions including:

- Complex powers: `z^2.5`, `z^(-2+3.5*i)`
- Complex coefficients: `3*i`, `(2+1.5*i)`
- Nested functions: `sin(cos(z))`, `exp(sin(z) * cos(z))`
- Complex exponents: `z^(2+3*i)`

### Examples of Complex Formulas

#### Sine of Complex Power
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --formula="sin(z^2.5)" --bailout=4.0 --output="mandel_sin_complex_power.png"
```

#### Complex Base and Exponent
```bash
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.5,0.5 --formula="3*i + z^(-2+3.5*i)" --bailout=4.0 --output="julia_complex_base_exp.png"
```

#### Nested Trigonometric Functions
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --formula="exp(sin(z) * cos(z))" --bailout=4.0 --output="mandel_nested_trig.png"
```

#### Complex Polynomial
```bash
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=1.0,0.5 --formula="(2+1.5*i)*z^3 + (1-0.5*i)*z^2 + z + (0.5+1*i)" --bailout=4.0 --output="julia_complex_poly.png"
```

## Advanced Mathematical Functions

The Fractal Toolkit now supports advanced mathematical functions including:

- **Tetration**: `z^^n` (iterated exponentiation)
- **Special Functions**: `gamma(z)`, `zeta(z)`
- **Complex Combinations**: Functions can be combined with other operations

### Examples of Advanced Functions

#### Tetration (Iterated Exponentiation)
```bash
# Simple tetration: z^^2 = z^z
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --formula="z^^2" --bailout=4.0 --output="mandel_tetration2.png"

# Higher tetration: z^^3 = z^(z^z)
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.5,0.5 --formula="z^^3" --bailout=4.0 --output="julia_tetration3.png"
```

#### Special Functions
```bash
# Gamma function
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --formula="gamma(z)" --bailout=4.0 --output="mandel_gamma.png"

# Riemann zeta function
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.5,0.5 --formula="zeta(z)" --bailout=4.0 --output="julia_zeta.png"
```

#### Complex Combinations
```bash
# Tetration combined with trigonometric functions
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --formula="sin(z^^2)" --bailout=4.0 --output="mandel_sin_tetration.png"

# Special functions in complex expressions
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.5,0.5 --formula="gamma(z) + zeta(z)" --bailout=4.0 --output="julia_gamma_zeta.png"
```

## Complete Example from User Documentation

The original example command from the documentation:
```bash
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.0,0.0 --color-pallette="[(#FF0000,0.0),(#00FF00,0.5),(#0000FF,1.0)]" --formula="z^2 + c" --bailout=4.0 --output="mandel_$(date +%Y%m%d_%H%M%S).png"
```

## Ultra high resolution render of a minibrot for wall art:
ftk-mandel --bounds=-0.04290453181602061,-0.04282444273121655,-0.9897886081089382,-0.9897085190241341 --dimensions=8192,8192 --max-iterations=4096 --spawn=0,0 --color-pallette=--color-pallette=--color-pallette="[(#000000,0.0),(#000088,0.1111),(#0000FF,0.2222),(#0000FF,0.3333),(#00FFFF,0.4444),(#00FF00,0.5556),(#FFFF00,0.6667),(#FF0000,0.7778),(#880000,0.8889),(#000000,1.0)]" --bailout=1.0e100 --formula="z^2 + c" --output="mandel_zoom_$(date +%Y%m%d_%H%M%S).png"

Note: The original example used `--color-pallette` (with a typo), but the correct option is `--color-pallette` as implemented in the code.
