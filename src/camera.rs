use cgmath::Point2;

/// Units: board cells / second
pub const CAMERA_SPEED: f32 = 4.0;

/// The numerator is the basic side-length of the screen (`2.0` since OpenGL ranges from `-1.0` to
/// `1.0`). The denominator denotes how many board cells will fit across the screen.
const ZOOM_MIN: f32 = 2.0 / 15.0;
const ZOOM_MAX: f32 = 2.0 / 2.5;

/// Zoom ranges from `0.0` (fully zoomed out) to `1.0` (fully zoomed in).
pub const ZOOM_DEFAULT: f32 = 0.3;
const ZOOM_STEP: f32 = 0.1;

pub struct Camera {
    pub center: Point2<f32>,
    pub zoom: f32,
}

impl Camera {
    /// Zoom the camera in (or out, if negative) by the given number of zoom steps.
    pub fn zoom_steps(&mut self, num_steps: f32) {
        self.zoom = clamp(self.zoom + num_steps * ZOOM_STEP, 0.0, 1.0);
    }

    pub fn zoom_factor(&self) -> f32 {
        interpolate_linear(ZOOM_MIN, ZOOM_MAX, self.zoom)
    }
}

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

fn interpolate_linear(start: f32, end: f32, amount: f32) -> f32 {
    start + (end - start) * amount
}
