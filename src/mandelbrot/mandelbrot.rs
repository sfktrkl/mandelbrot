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

    pub fn compute(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Vec<u8> {
        let mut grid = Vec::with_capacity(self.width * self.height);

        for py in 0..self.height {
            for px in 0..self.width {
                let x = x_min + (px as f64 / self.width as f64) * (x_max - x_min);
                let y = y_min + (py as f64 / self.height as f64) * (y_max - y_min);
                grid.push(self.mandelbrot(x, y));
            }
        }

        grid
    }

    fn mandelbrot(&self, x: f64, y: f64) -> u8 {
        let mut real = x;
        let mut imag = y;
        let mut iteration = 0;

        while real * real + imag * imag <= 4.0 && iteration < self.max_iterations {
            let temp_real = real * real - imag * imag + x;
            imag = 2.0 * real * imag + y;
            real = temp_real;

            iteration += 1;
        }

        (iteration * 255 / self.max_iterations) as u8
    }
}
