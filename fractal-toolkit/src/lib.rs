//! # Fractal Toolkit Library
//!
//! A comprehensive library for generating various types of fractals including Mandelbrot sets,
//! Julia sets, and Buddhabrot variants. This library provides the core algorithms and
//! utilities for the fractal toolkit executables with advanced support for custom imaginary units.
//!
//! ## Overview
//!
//! This library contains:
//! - Core fractal algorithms for Mandelbrot, Julia, and Buddhabrot sets
//! - Data structures for fractal parameters with support for custom imaginary units
//! - Image generation utilities
//! - Interactive HTML explorer generation
//! - Advanced mathematical expression evaluation with custom complex number systems
//!
//! ## Key Features
//!
//! - **Custom Imaginary Units**: Support for alternative complex number systems where i² can equal any complex number value
//!   - Standard complex numbers: i² = -1 (default behavior)
//!   - Split complex numbers: i² = 1 (hyperbolic numbers)
//!   - Alternative systems: i² = any complex value (enabling exploration of novel number systems)
//! - **Hyperoperation Support**: Full support for tetration (z^^w), pentation (z^^^w), and hexation (z^^^^w) operations
//! - **Advanced Formula Evaluation**: Sophisticated expression parser supporting complex mathematical functions
//! - **Orbit Debugging**: Built-in orbit tracing functionality to visualize iteration paths
//! - **High Performance**: Optimized multi-threaded rendering with rayon
//!
//! ## Mathematical Systems
//!
//! The library implements alternative complex number systems where the fundamental arithmetic operations
//! respect the custom imaginary unit value. When i² = custom_value, multiplication is defined as:
//! (a + bi) * (c + di) = ac + (ad + bc)*i + bd*(custom_value)
//!
//! This enables exploration of different mathematical properties and creates visually distinct fractals.

use num_complex::Complex;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use chrono::Local;
use image::{ImageBuffer, Rgba};

// Re-export modules for backward compatibility
pub use complex_numbers::{CustomComplex};
pub use formulas::{MathEvaluator};
pub use params::{FractalParams};
pub use orbits::{OrbitDebugger, OrbitStorage, OrbitStats};

// Export submodules
pub mod complex_numbers;
pub mod formulas;
pub mod expressions;
pub mod params;
pub mod orbits;
pub mod parsers;
pub mod hyperops;
pub mod math;
pub mod rendering;
pub mod utils;