use std::fs;
use std::path::Path;
mod mandelbrot;
mod renderer;

fn main() {
    let width = 800;
    let height = 800;
    let max_iterations = 255;

    let x_min = -2.0;
    let x_max = 1.0;
    let y_min = -1.5;
    let y_max = 1.5;

    let grid = mandelbrot::calculations::compute(
        width,
        height,
        x_min,
        x_max,
        y_min,
        y_max,
        max_iterations,
    );

    let output_folder = "output";
    let output_path = format!("{}/mandelbrot.png", output_folder);
    if !Path::new(output_folder).exists() {
        match fs::create_dir_all(output_folder) {
            Ok(_) => println!("Created output directory: {}", output_folder),
            Err(e) => {
                eprintln!("Failed to create output directory: {}", e);
                return;
            }
        }
    }

    let img = renderer::image::render(width, height, &grid);
    match img.save(&output_path) {
        Ok(_) => println!("Mandelbrot image saved to {}", output_path),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
