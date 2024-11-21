pub fn compute(
    width: u32,
    height: u32,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iterations: u32,
) -> Vec<u8> {
    let mut grid = Vec::with_capacity((width * height) as usize);

    for py in 0..height {
        for px in 0..width {
            // Map pixel to complex plane
            let x = x_min + (px as f64 / width as f64) * (x_max - x_min);
            let y = y_min + (py as f64 / height as f64) * (y_max - y_min);

            // Compute Mandelbrot escape value
            let value = mandelbrot(x, y, max_iterations);
            grid.push(value);
        }
    }

    grid
}

fn mandelbrot(x: f64, y: f64, max_iterations: u32) -> u8 {
    let mut real = x;
    let mut imag = y;
    let mut iteration = 0;

    while real * real + imag * imag <= 4.0 && iteration < max_iterations {
        let temp_real = real * real - imag * imag + x;
        imag = 2.0 * real * imag + y;
        real = temp_real;

        iteration += 1;
    }

    (iteration * 255 / max_iterations) as u8
}
