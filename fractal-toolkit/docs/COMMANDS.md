# Fractal Toolkit Commands Documentation

The Fractal Toolkit provides four main commands for generating different types of fractal images:

- `ftk-mandel` - Generates Mandelbrot fractal images
- `ftk-julia` - Generates Julia set fractal images
- `ftk-buddha` - Generates Buddhabrot fractal images
- `ftk-buddhaj` - Generates Buddhabrot Julia fractal images

## Common Concepts

### Bounds Format
All commands accept bounds in the format `[x_min, x_max, y_min, y_max]` representing the region of the complex plane to render:
- `--bounds=-2.0,2.0,-2.0,2.0` (from -2.0 to 2.0 on both real and imaginary axes)

### Dimensions Format
All commands accept dimensions in the format `[width, height]` representing the output image size:
- `--dimensions=512,512` (512x512 pixel image)

### Spawn Format
For Julia sets and Buddhabrot Julia, spawn points are specified as `[real, imag]`:
- `--spawn=0.285,0.01` (complex number 0.285 + 0.01i)

## ftk-mandel - Mandelbrot Set Generator

Generates Mandelbrot fractal images.

### Options

- `--bounds <BOUNDS>...` - Bounds of the fractal [x_min, x_max, y_min, y_max]
  - Example: `--bounds=-2.0,2.0,-2.0,2.0`

- `--max-iterations <MAX_ITERATIONS>` - Maximum number of iterations (default: 64)
  - Example: `--max-iterations=128`

- `--dimensions <DIMENSIONS>...` - Dimensions of the output image [width, height]
  - Example: `--dimensions=1024,1024`

- `--spawn <SPAWN>...` - Spawn point for the fractal [real, imag] (default: 0.0,0.0)
  - Example: `--spawn=0.0,0.0`

- `--color-pallette <COLOR_PALLETTE>` - Color palette [(hex_color, position), ...]
  - Example: `--color-pallette="[(#FF0000,0.0),(#00FF00,0.5),(#0000FF,1.0)]"`

- `--formula <FORMULA>` - Formula for the fractal (default: "z^2 + c")
  - Example: `--formula="z^2 + c"` or `--formula="z^3 + c"`

- `--bailout <BAILOUT>` - Bailout value (default: 4.0)
  - Example: `--bailout=4.0`

- `--output <OUTPUT>` - Output file name (default: "mandel_output.png")
  - Example: `--output="mandel_test.png"`

### Supported Formulas

The Mandelbrot generator supports various mathematical formulas:
- `"z^2 + c"` - Standard Mandelbrot
- `"z^3 + c"` - Cubic Mandelbrot
- `"z^4 + c"` - Quartic Mandelbrot
- `"sin(z) + c"` - Sine Mandelbrot
- `"cos(z) + c"` - Cosine Mandelbrot
- `"tan(z) + c"` - Tangent Mandelbrot
- `"exp(z) + c"` - Exponential Mandelbrot
- `"log(z) + c"` - Logarithmic Mandelbrot
- And many more complex combinations

### Example Usage

```bash
# Basic Mandelbrot set
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.0,0.0 --formula="z^2 + c" --bailout=4.0 --output="mandel_basic.png"

# Mandelbrot with custom color palette
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=128 --dimensions=1024,1024 --color-pallette="[(#FF0000,0.0),(#00FF00,0.5),(#0000FF,1.0)]" --formula="z^2 + c" --bailout=4.0 --output="mandel_colored.png"

# Mandelbrot with different formula
ftk-mandel --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=100 --dimensions=512,512 --formula="sin(z) + c" --bailout=4.0 --output="mandel_sine.png"
```

## ftk-julia - Julia Set Generator

Generates Julia set fractal images.

### Options

- `--bounds <BOUNDS>...` - Bounds of the fractal [x_min, x_max, y_min, y_max]
  - Example: `--bounds=-2.0,2.0,-2.0,2.0`

- `--max-iterations <MAX_ITERATIONS>` - Maximum number of iterations (default: 64)
  - Example: `--max-iterations=128`

- `--dimensions <DIMENSIONS>...` - Dimensions of the output image [width, height]
  - Example: `--dimensions=512,512`

- `--spawn <SPAWN>...` - Spawn point for the fractal [real, imag] (default: 0.0,0.0)
  - Example: `--spawn=0.285,0.01`

- `--color-pallette <COLOR_PALLETTE>` - Color palette [(hex_color, position), ...]
  - Example: `--color-pallette="[(#FF0000,0.0),(#00FF00,0.5),(#0000FF,1.0)]"`

- `--formula <FORMULA>` - Formula for the fractal (default: "z^2 + c")
  - Example: `--formula="z^2 + c"`

- `--bailout <BAILOUT>` - Bailout value (default: 4.0)
  - Example: `--bailout=4.0`

- `--output <OUTPUT>` - Output file name (default: "julia_output.png")
  - Example: `--output="julia_test.png"`

### Example Usage

```bash
# Basic Julia set with default parameters
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=64 --dimensions=512,512 --spawn=0.285,0.01 --formula="z^2 + c" --bailout=4.0 --output="julia_basic.png"

# Julia set with custom parameters
ftk-julia --bounds=-1.5,1.5,-1.5,1.5 --max-iterations=100 --dimensions=1024,1024 --spawn=-0.7,0.27015 --formula="z^2 + c" --bailout=4.0 --output="julia_custom.png"

# Julia set with different formula
ftk-julia --bounds=-2.0,2.0,-2.0,2.0 --max-iterations=80 --dimensions=512,512 --spawn=0.0,1.0 --formula="z^3 + c" --bailout=4.0 --output="julia_cubic.png"
```

## ftk-buddha - Buddhabrot Generator

Generates Buddhabrot fractal images.

### Options

- `--bounds <BOUNDS>...` - Bounds of the fractal [x_min, x_max, y_min, y_max]
  - Example: `--bounds=-2.0,2.0,-2.0,2.0`

- `--dimensions <DIMENSIONS>...` - Dimensions of the output image [width, height]
  - Example: `--dimensions=512,512`

- `--min-iterations <MIN_ITERATIONS>` - Minimum iterations for points to be considered (default: 10)
  - Example: `--min-iterations=5`

- `--max-iterations <MAX_ITERATIONS>` - Maximum iterations to check (default: 100)
  - Example: `--max-iterations=200`

- `--samples <SAMPLES>` - Number of random samples to take (default: 1000000)
  - Example: `--samples=500000`

- `--bailout <BAILOUT>` - Bailout value (default: 4.0)
  - Example: `--bailout=4.0`

- `--formula <FORMULA>` - Formula for the fractal (default: "z^2 + c")
  - Example: `--formula="z^2 + c"`

- `--red-channel <RED_CHANNEL>...` - Red channel: min_iter,max_iter,samples
  - Example: `--red-channel=10,50,50000`

- `--green-channel <GREEN_CHANNEL>...` - Green channel: min_iter,max_iter,samples
  - Example: `--green-channel=50,75,30000`

- `--blue-channel <BLUE_CHANNEL>...` - Blue channel: min_iter,max_iter,samples
  - Example: `--blue-channel=75,100,20000`

- `--output <OUTPUT>` - Output file name (default: "buddha_output.png")
  - Example: `--output="buddha_test.png"`

### Example Usage

```bash
# Basic Buddhabrot with default parameters
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=512,512 --min-iterations=10 --max-iterations=100 --samples=100000 --bailout=4.0 --formula="z^2 + c" --red-channel=10,50,50000 --green-channel=50,75,30000 --blue-channel=75,100,20000 --output="buddha_basic.png"

# Buddhabrot with higher resolution and more samples
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=1024,1024 --min-iterations=20 --max-iterations=200 --samples=500000 --bailout=4.0 --formula="z^2 + c" --red-channel=20,100,200000 --green-channel=100,150,150000 --blue-channel=150,200,100000 --output="buddha_high_res.png"

# Buddhabrot with different formula
ftk-buddha --bounds=-2.0,2.0,-2.0,2.0 --dimensions=512,512 --min-iterations=10 --max-iterations=100 --samples=100000 --bailout=4.0 --formula="z^3 + c" --red-channel=10,50,50000 --green-channel=50,75,30000 --blue-channel=75,100,20000 --output="buddha_cubic.png"
```

## ftk-buddhaj - Buddhabrot Julia Generator

Generates Buddhabrot Julia fractal images.

### Options

- `--bounds <BOUNDS>...` - Bounds of the fractal [x_min, x_max, y_min, y_max]
  - Example: `--bounds=-2.0,2.0,-2.0,2.0`

- `--dimensions <DIMENSIONS>...` - Dimensions of the output image [width, height]
  - Example: `--dimensions=512,512`

- `--min-iterations <MIN_ITERATIONS>` - Minimum iterations for points to be considered (default: 10)
  - Example: `--min-iterations=5`

- `--max-iterations <MAX_ITERATIONS>` - Maximum iterations to check (default: 100)
  - Example: `--max-iterations=200`

- `--samples <SAMPLES>` - Number of random samples to take (default: 1000000)
  - Example: `--samples=500000`

- `--bailout <BAILOUT>` - Bailout value (default: 4.0)
  - Example: `--bailout=4.0`

- `--formula <FORMULA>` - Formula for the fractal (default: "z^2 + c")
  - Example: `--formula="z^2 + c"`

- `--spawn <SPAWN>...` - Spawn point for the fractal [real, imag] (default: 0.0,0.0)
  - Example: `--spawn=0.285,0.01`

- `--red-channel <RED_CHANNEL>...` - Red channel: min_iter,max_iter,samples
  - Example: `--red-channel=10,50,50000`

- `--green-channel <GREEN_CHANNEL>...` - Green channel: min_iter,max_iter,samples
  - Example: `--green-channel=50,75,30000`

- `--blue-channel <BLUE_CHANNEL>...` - Blue channel: min_iter,max_iter,samples
  - Example: `--blue-channel=75,100,20000`

- `--output <OUTPUT>` - Output file name (default: "buddhaj_output.png")
  - Example: `--output="buddhaj_test.png"`

### Example Usage

```bash
# Basic Buddhabrot Julia with default parameters
ftk-buddhaj --bounds=-2.0,2.0,-2.0,2.0 --dimensions=512,512 --min-iterations=10 --max-iterations=100 --samples=100000 --bailout=4.0 --spawn=0.285,0.01 --formula="z^2 + c" --red-channel=10,50,50000 --green-channel=50,75,30000 --blue-channel=75,100,20000 --output="buddhaj_basic.png"

# Buddhabrot Julia with different spawn point
ftk-buddhaj --bounds=-2.0,2.0,-2.0,2.0 --dimensions=1024,1024 --min-iterations=20 --max-iterations=200 --samples=500000 --bailout=4.0 --spawn=-0.7,0.27015 --formula="z^2 + c" --red-channel=20,100,200000 --green-channel=100,150,150000 --blue-channel=150,200,100000 --output="buddhaj_custom.png"

# Buddhabrot Julia with different formula
ftk-buddhaj --bounds=-2.0,2.0,-2.0,2.0 --dimensions=512,512 --min-iterations=10 --max-iterations=100 --samples=100000 --bailout=4.0 --spawn=0.0,1.0 --formula="z^3 + c" --red-channel=10,50,50000 --green-channel=50,75,30000 --blue-channel=75,100,20000 --output="buddhaj_cubic.png"
```

## Additional Features

### HTML Explorer Generation

All commands automatically generate an HTML explorer file alongside the PNG image. This HTML file allows you to:
- Click and drag on the fractal image to select a region
- See the command to render that specific region
- Choose different aspect ratios and resolutions
- Generate commands for zooming into specific areas

The HTML file has the same name as the PNG file but with a `.html` extension.

### Formula Support

The fractal toolkit supports a wide variety of mathematical formulas:
- Basic: `z^2 + c`, `z^3 + c`, `z^4 + c`
- Trigonometric: `sin(z) + c`, `cos(z) + c`, `tan(z) + c`
- Exponential: `exp(z) + c`, `log(z) + c`
- Complex combinations: `z*z + sin(c)`, `z^2 + c*sin(z)`, etc.

### Output File Naming

You can use shell variables in output file names:
- `--output="mandel_$(date +%Y%m%d_%H%M%S).png"` - Creates timestamped files
- `--output="fractal_${USER}.png"` - Uses environment variables