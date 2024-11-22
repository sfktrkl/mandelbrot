use rayon::prelude::*;

pub struct Mandelbrot {
    width: usize,
    height: usize,
    max_iterations: usize,
    palette: Vec<(u8, u8, u8)>,
}

impl Mandelbrot {
    pub fn new(width: usize, height: usize, max_iterations: usize) -> Self {
        Self {
            width,
            height,
            max_iterations,
            palette: vec![
                (0, 0, 50),      // Very dark blue
                (0, 0, 100),     // Dark blue
                (0, 50, 150),    // Deep blue
                (0, 100, 200),   // Medium blue
                (100, 150, 255), // Lighter blue
                (200, 200, 255), // Very light blue
                (255, 150, 100), // Light orange
                (255, 100, 50),  // Orange
                (255, 50, 0),    // Bright orange
                (255, 255, 255), // White
            ],
        }
    }

    pub fn compute(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Vec<(u8, u8, u8)> {
        (0..self.height)
            .into_par_iter()
            .flat_map(|py| {
                let y = y_min + (py as f64 / self.height as f64) * (y_max - y_min);
                (0..self.width)
                    .map(|px| {
                        let x = x_min + (px as f64 / self.width as f64) * (x_max - x_min);
                        self.mandelbrot(x, y)
                    })
                    .collect::<Vec<(u8, u8, u8)>>()
            })
            .collect()
    }

    fn mandelbrot(&self, x0: f64, y0: f64) -> (u8, u8, u8) {
        let mut real = 0.0;
        let mut imag = 0.0;
        let mut iteration = 0;

        while real * real + imag * imag <= 4.0 && iteration < self.max_iterations {
            let temp_real = real * real - imag * imag + x0;
            imag = 2.0 * real * imag + y0;
            real = temp_real;

            iteration += 1;
        }

        self.color_map(iteration as f64)
    }

    fn color_map(&self, iteration: f64) -> (u8, u8, u8) {
        let idx = (iteration as usize) % self.palette.len();
        let next_idx = if idx + 1 < self.palette.len() {
            idx + 1
        } else {
            idx
        };

        let c1 = self.palette[idx];
        let c2 = self.palette[next_idx];

        let factor = iteration - iteration.floor();
        let r = (c1.0 as f64 * (1.0 - factor) + c2.0 as f64 * factor) as u8;
        let g = (c1.1 as f64 * (1.0 - factor) + c2.1 as f64 * factor) as u8;
        let b = (c1.2 as f64 * (1.0 - factor) + c2.2 as f64 * factor) as u8;

        (r, g, b)
    }
}
