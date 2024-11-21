use crate::mandelbrot::Mandelbrot;
use minifb::{Key, Window, WindowOptions};

#[derive(Clone, Copy)]
struct Viewport {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
}

impl Viewport {
    fn zoom(&mut self, factor: f64, mouse_x: f64, mouse_y: f64) {
        let width: f64 = self.x_max - self.x_min;
        let height: f64 = self.y_max - self.y_min;

        let new_width: f64 = width * factor;
        let new_height: f64 = height * factor;

        let rel_mouse_x: f64 = (mouse_x / self.width as f64) * width + self.x_min;
        let rel_mouse_y: f64 = (mouse_y / self.height as f64) * height + self.y_min;

        self.x_min = rel_mouse_x - (rel_mouse_x - self.x_min) * factor;
        self.x_max = self.x_min + new_width;
        self.y_min = rel_mouse_y - (rel_mouse_y - self.y_min) * factor;
        self.y_max = self.y_min + new_height;
    }
}

pub struct WindowManager {
    mandelbrot: Mandelbrot,
    viewport: Viewport,
    window: Window,
}

impl WindowManager {
    pub fn new(mandelbrot: Mandelbrot, width: usize, height: usize) -> Self {
        let window = Window::new("Mandelbrot Set", width, height, WindowOptions::default())
            .expect("Failed to create window");

        let viewport = Viewport {
            x_min: -2.0,
            x_max: 1.0,
            y_min: -1.5,
            y_max: 1.5,
            width,
            height,
        };

        Self {
            mandelbrot,
            window,
            viewport,
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.render_frame();

            if let Some(scroll) = self.window.get_scroll_wheel() {
                let factor: f32 = if scroll.1 > 0.0 { 0.9 } else { 1.1 };

                if let Some((mouse_x, mouse_y)) =
                    self.window.get_mouse_pos(minifb::MouseMode::Discard)
                {
                    self.viewport
                        .zoom(factor as f64, mouse_x as f64, mouse_y as f64);
                }
            }
        }
    }

    fn render_frame(&mut self) {
        let grid = self.mandelbrot.compute(
            self.viewport.x_min,
            self.viewport.x_max,
            self.viewport.y_min,
            self.viewport.y_max,
        );

        let buffer = self.render(&grid);

        self.window
            .update_with_buffer(&buffer, self.viewport.width, self.viewport.height)
            .expect("Failed to update window");
    }

    fn render(&self, grid: &[u8]) -> Vec<u32> {
        let mut buffer = Vec::with_capacity(self.viewport.width * self.viewport.height);

        for &value in grid {
            let rgba = u32::from_be_bytes([0, value, value, value]);
            buffer.push(rgba);
        }

        buffer
    }
}
