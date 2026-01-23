# Fractal Toolkit Project Summary

## Project Overview
The Fractal Toolkit is a comprehensive Rust library for generating various types of fractals including Mandelbrot sets, Julia sets, and Buddhabrot variants. It provides advanced support for custom imaginary units, hyperoperations, and complex mathematical expressions.

## Main Features
- **Custom Imaginary Units**: Support for alternative complex number systems where i² can equal any complex number value
- **Hyperoperation Support**: Full support for tetration (z^^w), pentation (z^^^w), and hexation (z^^^^w) operations
- **Advanced Formula Evaluation**: Sophisticated expression parser supporting complex mathematical functions
- **Orbit Debugging**: Built-in orbit tracing functionality to visualize iteration paths
- **High Performance**: Optimized multi-threaded rendering with rayon

## Project Structure
- `/fractal-toolkit` - Main project directory
  - `/src` - Source code
    - `/bin` - Executable binaries (ftk-mandel, ftk-julia, ftk-buddha, ftk-buddhaj, ftk-dca, ftk-calc)
    - `/parsers` - Expression parsing functionality
    - `/complex_math` - Complex number mathematics
    - `/fractals` - Fractal algorithm implementations
    - `/hyperops` - Hyperoperation implementations
    - `/math` - Mathematical utilities
    - `/rendering` - Image rendering utilities
    - `/utils` - Utility functions
  - `/docs` - Documentation files
  - `/tests` - Test files
  - `/test_files` - Test data
  - `/nya_elyria` - Additional project components
  - `Cargo.toml` - Project dependencies and configuration
  - `README.md` - Project documentation

## Key Files
- `src/lib.rs` - Main library file (5,321 lines) - Contains core fractal algorithms, custom complex number system, and mathematical expression evaluator
- `src/lib_old.rs` - Backup/copy of main library (5,321 lines)
- `src/parsers/expression_parser.rs` - Expression parsing functionality (785 lines)
- `src/math.rs` - Mathematical utilities (351 lines)
- `src/bin/ftk-mandel.rs` - Mandelbrot fractal generator (309 lines)
- `src/bin/ftk-buddhaj.rs` - Buddhabrot Julia fractal generator (315 lines)
- `src/hyperops.rs` - Hyperoperation implementations (255 lines)

## Executables
- `ftk-mandel` - Mandelbrot-style fractal generator with custom formulas
- `ftk-julia` - Julia set fractal generator
- `ftk-buddha` - Buddhabrot fractal generator
- `ftk-buddhaj` - Buddhabrot Julia fractal generator
- `ftk-dca` - Density-based coloring algorithm fractal generator
- `ftk-calc` - Calculator utility for complex number operations

## Dependencies
- clap - Command-line argument parsing
- image - Image processing and generation
- num-complex - Complex number operations
- rand - Random number generation
- serde - Serialization/deserialization
- num - Numerical traits and functions
- special - Special mathematical functions
- chrono - Date and time handling
- rayon - Parallel computing
- num_cpus - CPU core detection

## Current Issues
- Large monolithic files: `lib.rs` and `lib_old.rs` are both 5,321 lines long and should be split into smaller, more manageable modules
- Some directories have .rs files but no corresponding implementation files (e.g., math/, rendering/, utils/)
- Need for better modularization of the complex mathematical operations

## Architecture
The project uses a modular architecture with separate concerns for parsing, complex mathematics, fractal algorithms, hyperoperations, and rendering. The core innovation is the `CustomComplex` struct that allows for alternative complex number systems where i² can equal any complex value, enabling exploration of different mathematical properties.