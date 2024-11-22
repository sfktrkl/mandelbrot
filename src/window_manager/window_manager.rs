use super::viewport::Viewport;
use crate::mandelbrot::Mandelbrot;
use minifb::{Key, Window, WindowOptions};

pub struct WindowManager {
    buffer: Option<Vec<u32>>,
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
            buffer: None,
            mandelbrot,
            window,
            viewport,
        }
    }

    pub fn run(&mut self) {
        if self.buffer.is_none() {
            self.buffer = Some(self.render());
        }

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            if let Some(scroll) = self.window.get_scroll_wheel() {
                let factor: f32 = if scroll.1 > 0.0 { 0.5 } else { 2.0 };

                if let Some((mouse_x, mouse_y)) =
                    self.window.get_mouse_pos(minifb::MouseMode::Discard)
                {
                    self.viewport
                        .zoom(factor as f64, mouse_x as f64, mouse_y as f64);
                }

                self.buffer = Some(self.render());
            }

            self.window
                .update_with_buffer(
                    &self.buffer.as_ref().unwrap(),
                    self.viewport.width,
                    self.viewport.height,
                )
                .expect("Failed to update window");
        }
    }

    fn render(&self) -> Vec<u32> {
        let grid = self.mandelbrot.compute(
            self.viewport.x_min,
            self.viewport.x_max,
            self.viewport.y_min,
            self.viewport.y_max,
        );

        let mut buffer = Vec::with_capacity(self.viewport.width * self.viewport.height);

        for value in grid {
            let rgba = u32::from_be_bytes([0, value.0, value.1, value.2]);
            buffer.push(rgba);
        }

        buffer
    }
}
