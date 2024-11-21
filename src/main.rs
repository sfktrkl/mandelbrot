mod mandelbrot;
mod window_manager;

fn main() {
    let mut manager = window_manager::WindowManager::new(800, 600);
    manager.run();
}
