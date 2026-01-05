use num_complex::Complex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPoint {
    pub color: String,  // Hex color like "#FF0000"
    pub position: f64,  // Position in gradient from 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub struct FractalParams {
    pub bounds: [f64; 4],           // [x_min, x_max, y_min, y_max]
    pub max_iterations: u32,
    pub spawn: Complex<f64>,        // For Julia sets
    pub bailout: f64,
    pub formula: String,
}

impl FractalParams {
    pub fn new(bounds: [f64; 4], max_iterations: u32, spawn: [f64; 2], bailout: f64, formula: String) -> Self {
        Self {
            bounds,
            max_iterations,
            spawn: Complex::new(spawn[0], spawn[1]),
            bailout,
            formula,
        }
    }
}

/// Generate HTML file with interactive features for the fractal image
pub fn generate_html_file(
    image_path: &str,
    bounds: [f64; 4],
    dimensions: [u32; 2],
    command_template: &str,
) -> std::io::Result<()> {
    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Fractal Explorer</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 20px;
            background-color: #f0f0f0;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .image-container {{
            position: relative;
            display: inline-block;
            margin-bottom: 20px;
        }}
        #fractal-image {{
            max-width: 100%;
            height: auto;
            border: 1px solid #ccc;
        }}
        #selection-box {{
            position: absolute;
            border: 2px dashed red;
            background: rgba(255, 0, 0, 0.2);
            pointer-events: none;
            display: none;
        }}
        .controls {{
            margin-top: 20px;
        }}
        .aspect-ratio-controls {{
            margin-bottom: 15px;
        }}
        .aspect-ratio-controls label {{
            margin-right: 10px;
            display: inline-block;
        }}
        .resolution-controls {{
            margin-bottom: 15px;
        }}
        .resolution-controls select {{
            padding: 5px;
            margin-left: 10px;
        }}
        .command-output {{
            background: #fff;
            padding: 10px;
            border: 1px solid #ccc;
            border-radius: 4px;
            font-family: monospace;
            white-space: pre-wrap;
            word-break: break-all;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Fractal Explorer</h1>
        <p>Click and drag on the image to select a region. The command to render that region will appear below.</p>

        <div class="image-container">
            <img id="fractal-image" src="{}" alt="Fractal Image">
            <div id="selection-box"></div>
        </div>

        <div class="controls">
            <div class="aspect-ratio-controls">
                <label><input type="radio" name="aspect-ratio" value="1:1" checked> 1:1 (Square)</label>
                <label><input type="radio" name="aspect-ratio" value="3:2"> 3:2</label>
                <label><input type="radio" name="aspect-ratio" value="2:3"> 2:3</label>
                <label><input type="radio" name="aspect-ratio" value="4:3"> 4:3</label>
                <label><input type="radio" name="aspect-ratio" value="3:4"> 3:4</label>
                <label><input type="radio" name="aspect-ratio" value="16:9"> 16:9</label>
                <label><input type="radio" name="aspect-ratio" value="9:16"> 9:16</label>
            </div>

            <div class="resolution-controls">
                <label>Resolution:</label>
                <select id="resolution-select">
                    <option value="640x480">640x480</option>
                    <option value="800x600">800x600</option>
                    <option value="1024x768">1024x768</option>
                    <option value="1280x720" selected>1280x720</option>
                    <option value="1920x1080">1920x1080</option>
                    <option value="2560x1440">2560x1440</option>
                    <option value="3840x2160">3840x2160</option>
                </select>
            </div>

            <h3>Command to render selected region:</h3>
            <div id="command-output" class="command-output">{}</div>
        </div>
    </div>

    <script>
        const img = document.getElementById('fractal-image');
        const selectionBox = document.getElementById('selection-box');
        let isSelecting = false;
        let startX, startY, currentX, currentY;

        // Get image dimensions
        const imgWidth = {};
        const imgHeight = {};
        const bounds = [{}, {}, {}, {}]; // [x_min, x_max, y_min, y_max]

        img.addEventListener('mousedown', startSelection);
        document.addEventListener('mousemove', updateSelection);
        document.addEventListener('mouseup', endSelection);

        function startSelection(e) {{
            isSelecting = true;

            // Get the position of the image relative to the viewport
            const rect = img.getBoundingClientRect();

            startX = e.clientX - rect.left;
            startY = e.clientY - rect.top;

            selectionBox.style.left = startX + 'px';
            selectionBox.style.top = startY + 'px';
            selectionBox.style.width = '0px';
            selectionBox.style.height = '0px';
            selectionBox.style.display = 'block';
        }}

        function updateSelection(e) {{
            if (!isSelecting) return;

            const rect = img.getBoundingClientRect();
            currentX = e.clientX - rect.left;
            currentY = e.clientY - rect.top;

            const width = Math.abs(currentX - startX);
            const height = Math.abs(currentY - startY);

            const left = Math.min(startX, currentX);
            const top = Math.min(startY, currentY);

            selectionBox.style.left = left + 'px';
            selectionBox.style.top = top + 'px';
            selectionBox.style.width = width + 'px';
            selectionBox.style.height = height + 'px';
        }}

        function endSelection() {{
            if (!isSelecting) return;
            isSelecting = false;

            // Calculate the selected region in complex plane coordinates
            const selectedXMin = bounds[0] + (startX / imgWidth) * (bounds[1] - bounds[0]);
            const selectedXMax = bounds[0] + (currentX / imgWidth) * (bounds[1] - bounds[0]);
            const selectedYMin = bounds[2] + (startY / imgHeight) * (bounds[3] - bounds[2]);
            const selectedYMax = bounds[2] + (currentY / imgHeight) * (bounds[3] - bounds[2]);

            // Ensure correct order
            const xMin = Math.min(selectedXMin, selectedXMax);
            const xMax = Math.max(selectedXMin, selectedXMax);
            const yMin = Math.min(selectedYMin, selectedYMax);
            const yMax = Math.max(selectedYMin, selectedYMax);

            // Get selected aspect ratio
            const selectedRatio = document.querySelector('input[name="aspect-ratio"]:checked').value;
            const [ratioX, ratioY] = selectedRatio.split(':').map(Number);

            // Get selected resolution
            const resolutionSelect = document.getElementById('resolution-select');
            const [width, height] = resolutionSelect.value.split('x').map(Number);

            // Generate the command
            const command = `{}`.replace('{{bounds}}', `[${{xMin}}, ${{xMax}}, ${{yMin}}, ${{yMax}}]`)
                                    .replace('{{dimensions}}', `[${{width}}, ${{height}}]`);

            document.getElementById('command-output').textContent = command;
        }}

        // Update command when resolution or aspect ratio changes
        document.getElementById('resolution-select').addEventListener('change', updateCommand);
        document.querySelectorAll('input[name="aspect-ratio"]').forEach(radio => {{
            radio.addEventListener('change', updateCommand);
        }});

        function updateCommand() {{
            // This would update the command based on the current selection and settings
            // For now, we just show a placeholder
            if (startX !== undefined && currentX !== undefined) {{
                endSelection(); // Recalculate with new settings
            }}
        }}
    </script>
</body>
</html>"#,
        image_path,
        command_template,
        dimensions[0],
        dimensions[1],
        bounds[0],
        bounds[1],
        bounds[2],
        bounds[3],
        command_template
    );

    let html_path = std::path::Path::new(image_path)
        .with_extension("html");

    std::fs::write(html_path, html_content)
}

/// Calculate the number of iterations for a point in the Mandelbrot set
pub fn mandelbrot_iterations(c: Complex<f64>, params: &FractalParams) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let mut iter = 0;
    
    while iter < params.max_iterations {
        z = z * z + c;
        if z.norm_sqr() > params.bailout * params.bailout {
            break;
        }
        iter += 1;
    }
    
    iter
}

/// Calculate the number of iterations for a point in a Julia set
pub fn julia_iterations(z: Complex<f64>, params: &FractalParams) -> u32 {
    let c = params.spawn;  // Use spawn point as the constant for Julia set
    let mut z = z;
    let mut iter = 0;
    
    while iter < params.max_iterations {
        z = z * z + c;
        if z.norm_sqr() > params.bailout * params.bailout {
            break;
        }
        iter += 1;
    }
    
    iter
}

/// Convert pixel coordinates to complex plane coordinates
pub fn pixel_to_complex(x: u32, y: u32, width: u32, height: u32, bounds: [f64; 4]) -> Complex<f64> {
    let [x_min, x_max, y_min, y_max] = bounds;

    // Use (width-1) and (height-1) to ensure the last pixel maps to x_max/y_max
    let real = if width > 1 {
        x_min + (x as f64 / (width - 1) as f64) * (x_max - x_min)
    } else {
        x_min
    };
    let imag = if height > 1 {
        y_min + (y as f64 / (height - 1) as f64) * (y_max - y_min)
    } else {
        y_min
    };

    Complex::new(real, imag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_pixel_to_complex() {
        // Test conversion from pixel to complex coordinates
        let bounds = [-2.0, 2.0, -2.0, 2.0];  // 4x4 area
        let width = 4;
        let height = 4;

        // Test corner points
        let top_left = pixel_to_complex(0, 0, width, height, bounds);
        assert!((top_left.re - (-2.0)).abs() < 0.01);  // Should be x_min
        assert!((top_left.im - (-2.0)).abs() < 0.01);  // Should be y_min

        let bottom_right = pixel_to_complex(width - 1, height - 1, width, height, bounds);
        // For a 4x4 image, the last pixel is at index 3, so it maps to slightly less than x_max/y_max
        // due to 0-indexing: pixel 3 of 4 pixels maps to 3/3 = 1.0 of the range
        let expected_x = -2.0 + (3.0 / 3.0) * (2.0 - (-2.0));  // Should be 2.0
        let expected_y = -2.0 + (3.0 / 3.0) * (2.0 - (-2.0));  // Should be 2.0
        assert!((bottom_right.re - expected_x).abs() < 0.01);  // Should be close to x_max
        assert!((bottom_right.im - expected_y).abs() < 0.01);  // Should be close to y_max
    }

    #[test]
    fn test_mandelbrot_iterations_origin() {
        // The origin (0, 0) should be in the Mandelbrot set (high iterations)
        let params = FractalParams::new([-2.0, 2.0, -2.0, 2.0], 100, [0.0, 0.0], 4.0, "z^2 + c".to_string());
        let c = Complex::new(0.0, 0.0);
        let iterations = mandelbrot_iterations(c, &params);
        assert_eq!(iterations, 100);  // Should reach max iterations
    }

    #[test]
    fn test_mandelbrot_iterations_outside_set() {
        // A point far outside the set should escape quickly
        let params = FractalParams::new([-2.0, 2.0, -2.0, 2.0], 100, [0.0, 0.0], 4.0, "z^2 + c".to_string());
        let c = Complex::new(2.0, 2.0);  // This should escape quickly
        let iterations = mandelbrot_iterations(c, &params);
        assert!(iterations < 10);  // Should escape in few iterations
    }

    #[test]
    fn test_julia_iterations_origin() {
        // Test Julia set with a simple c value
        let params = FractalParams::new([-2.0, 2.0, -2.0, 2.0], 100, [0.0, 0.0], 4.0, "z^2 + c".to_string());
        let z = Complex::new(0.0, 0.0);
        let iterations = julia_iterations(z, &params);
        assert_eq!(iterations, 100);  // z=0, c=0 should stay bounded
    }

    #[test]
    fn test_complex_norm_sqr() {
        // Test that our complex number operations work correctly
        let z = Complex::new(3.0, 4.0);
        assert_eq!(z.norm_sqr(), 25.0);  // 3^2 + 4^2 = 25
    }
}