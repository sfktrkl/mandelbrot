use rayon::prelude::*;

pub struct Mandelbrot {
    width: usize,
    height: usize,
    max_iterations: usize,
}

impl Mandelbrot {
    pub fn new(width: usize, height: usize, max_iterations: usize) -> Self {
        Self {
            width,
            height,
            max_iterations,
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

    fn mandelbrot(&self, x: f64, y: f64) -> (u8, u8, u8) {
        let mut real = x;
        let mut imag = y;
        let mut iteration = 0;

        while real * real + imag * imag <= 4.0 && iteration < self.max_iterations {
            let temp_real = real * real - imag * imag + x;
            imag = 2.0 * real * imag + y;
            real = temp_real;

            iteration += 1;
        }

        if iteration == self.max_iterations {
            return (0, 0, 0);
        }
        self.color_map(iteration)
    }

    fn color_map(&self, iteration: usize) -> (u8, u8, u8) {
        let factor = iteration as f32 / self.max_iterations as f32;

        let r = (factor * 255.0) as u8;
        let g = (factor * 255.0) as u8;
        let b = ((1.0 - factor) * 255.0) as u8;

        (r, g, b)
    }
}
