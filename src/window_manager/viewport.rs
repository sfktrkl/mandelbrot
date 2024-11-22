#[derive(Clone, Copy)]
pub struct Viewport {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub width: usize,
    pub height: usize,
}

impl Viewport {
    pub fn zoom(&mut self, factor: f64, mouse_x: f64, mouse_y: f64) {
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
