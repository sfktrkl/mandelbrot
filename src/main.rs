mod mandelbrot;
mod window_manager;

use mandelbrot::Mandelbrot;
use window_manager::WindowManager;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const MAX_ITERATIONS: usize = 1000;

    let mandelbrot = Mandelbrot::new(WIDTH, HEIGHT, MAX_ITERATIONS);
    let mut manager = WindowManager::new(mandelbrot, WIDTH, HEIGHT);
    manager.run();
}
