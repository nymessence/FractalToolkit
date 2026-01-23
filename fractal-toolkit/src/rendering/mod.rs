use image::{ImageBuffer, Rgba, Pixel};
use num_complex::Complex;
use std::path::Path;

/// Utilities for rendering fractals to images
pub struct FractalRenderer;

impl FractalRenderer {
    /// Create a new fractal renderer
    pub fn new() -> Self {
        Self {}
    }

    /// Render a fractal to an image buffer
    pub fn render_to_image<P>(
        &self,
        width: u32,
        height: u32,
        calculate_pixel: impl Fn(u32, u32) -> Rgba<u8>
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut imgbuf = ImageBuffer::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = calculate_pixel(x, y);
        }

        imgbuf
    }

    /// Save an image buffer to a file
    pub fn save_image<P: AsRef<Path>>(
        &self,
        imgbuf: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        path: P
    ) -> Result<(), image::ImageError> {
        imgbuf.save(path)
    }

    /// Generate a color from iteration count and max iterations
    pub fn color_from_iteration(iter: u32, max_iter: u32) -> Rgba<u8> {
        if iter == max_iter {
            // Inside the set - black
            Rgba([0, 0, 0, 255])
        } else {
            // Outside the set - color based on iteration count
            let t = iter as f64 / max_iter as f64;
            
            // Create a smooth color gradient
            let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
            let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
            let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
            
            Rgba([r, g, b, 255])
        }
    }

    /// Map a complex value to a color
    pub fn complex_to_color(z: Complex<f64>) -> Rgba<u8> {
        // Normalize the complex value to RGB
        let norm = z.norm();
        let normalized_norm = (norm % 1.0) * 255.0;
        
        let r = ((z.re + 1.0).abs() * 127.0) as u8;
        let g = ((z.im + 1.0).abs() * 127.0) as u8;
        let b = (normalized_norm % 255.0) as u8;
        
        Rgba([r, g, b, 255])
    }
}

/// Color palette for fractal rendering
pub struct ColorPalette {
    pub colors: Vec<(f64, [u8; 3])>, // (position, [r, g, b])
}

impl ColorPalette {
    /// Create a default rainbow palette
    pub fn default_rainbow() -> Self {
        Self {
            colors: vec![
                (0.0, [0, 0, 0]),       // Black
                (0.1, [0, 0, 128]),     // Dark Blue
                (0.2, [0, 0, 255]),     // Blue
                (0.3, [0, 128, 0]),     // Dark Green
                (0.4, [0, 255, 0]),     // Green
                (0.5, [255, 255, 0]),   // Yellow
                (0.6, [255, 128, 0]),   // Orange
                (0.7, [255, 0, 0]),     // Red
                (0.8, [128, 0, 128]),   // Purple
                (0.9, [255, 0, 255]),   // Magenta
                (1.0, [255, 255, 255]), // White
            ],
        }
    }

    /// Get a color for a given position in the palette
    pub fn get_color(&self, pos: f64) -> [u8; 3] {
        let clamped_pos = pos.max(0.0).min(1.0);
        
        // Find the two closest color stops
        let mut lower_idx = 0;
        let mut upper_idx = self.colors.len() - 1;
        
        for i in 0..self.colors.len() {
            if self.colors[i].0 <= clamped_pos {
                lower_idx = i;
            }
            if self.colors[i].0 >= clamped_pos && i > 0 {
                upper_idx = i;
                break;
            }
        }
        
        if lower_idx == upper_idx {
            return self.colors[lower_idx].1;
        }
        
        // Interpolate between the two colors
        let lower_pos = self.colors[lower_idx].0;
        let upper_pos = self.colors[upper_idx].0;
        let ratio = (clamped_pos - lower_pos) / (upper_pos - lower_pos);
        
        let lower_color = self.colors[lower_idx].1;
        let upper_color = self.colors[upper_idx].1;
        
        [
            (lower_color[0] as f64 + (upper_color[0] as f64 - lower_color[0] as f64) * ratio) as u8,
            (lower_color[1] as f64 + (upper_color[1] as f64 - lower_color[1] as f64) * ratio) as u8,
            (lower_color[2] as f64 + (upper_color[2] as f64 - lower_color[2] as f64) * ratio) as u8,
        ]
    }
}