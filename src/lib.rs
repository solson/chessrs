extern crate bit_set;
extern crate cgmath;
#[macro_use] extern crate glium;
extern crate time;

mod board;
mod camera;
mod render;
pub mod units;

use bit_set::BitSet;
use cgmath::{EuclideanVector, Point, Point2, SquareMatrix, Vector, Vector2, Vector4};
use glium::glutin::VirtualKeyCode;

use board::Board;
use render::Display;

/// Actions to take from the game loop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    None,
    Stop,
}

pub struct GameState {
    display: Display,
    held_keys: BitSet,
    board: Board<bool>,
    mouse_position: Point2<f32>,

    /// Set to the current time in nanoseconds at the beginning of each frame's `update` step.
    time_last_frame: u64,

    /// Frames-per-second dependent scaling factor, in units of seconds per frame. For an example
    /// of its use, an object moving across the screen at `n` board cells per second should move
    /// `n * time_factor` board cells per frame.
    time_factor: f32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            display: Display::new_window(),
            board: Board::new_test_board(),
            held_keys: BitSet::new(),
            mouse_position: Point2::origin(),

            // HACK: Assumes 60 fps. On the other hand, it's only for the first frame.
            time_factor: 1.0 / 60.0,
            time_last_frame: time::precise_time_ns(),
        }
    }

    pub fn handle_input(&mut self) -> Action {
        use glium::glutin::ElementState::*;
        use glium::glutin::Event::*;
        use glium::glutin::MouseScrollDelta::*;

        for event in self.display.backend.poll_events() {
            match event {
                Closed => return Action::Stop,

                KeyboardInput(Pressed, _, Some(key)) => {
                    self.held_keys.insert(key as usize);
                }

                KeyboardInput(Released, _, Some(key)) => {
                    self.held_keys.remove(&(key as usize));
                }

                MouseWheel(LineDelta(_, scroll_amount)) => {
                    self.display.camera.zoom_steps(scroll_amount);
                }

                MouseMoved((x_pixel, y_pixel)) => {
                    // Convert from pixel indices ranging from `0..width` and `0..height` to OpenGL
                    // screen coordinates ranging from `-1.0..1.0`.
                    let x_screen = 2.0 * x_pixel as f32 / self.display.width as f32 - 1.0;
                    let y_screen = -2.0 * y_pixel as f32 / self.display.height as f32 + 1.0;

                    // Convert from OpenGL screen coordinates to board coordinates using the
                    // inverse of the view transformation matrix.
                    let inv_view = self.display.view_transform().invert().unwrap();
                    let screen_vec = Vector4::new(x_screen, y_screen, 0.0, 1.0);
                    let board_vec = inv_view * screen_vec;

                    // FIXME: Record mouse position in raw screen coordinates to update the derived
                    // board coordinates when panning and zooming while the mouse is stationary.
                    self.mouse_position = Point2::new(board_vec.x, board_vec.y);
                }

                _ => {},
            }
        }

        Action::None
    }

    pub fn update(&mut self) {
        use glium::glutin::VirtualKeyCode as Key;

        let time = time::precise_time_ns();
        self.time_factor = (time - self.time_last_frame) as f32 * units::NS_TO_S;
        self.time_last_frame = time;

        let camera_direction = Vector2 {
            x: self.get_key_direction(Key::Right, Key::Left),
            y: self.get_key_direction(Key::Up, Key::Down),
        };

        if camera_direction != Vector2::zero() {
            let frame_step = camera::CAMERA_SPEED * self.time_factor;
            self.display.camera.center = self.display.camera.center
                + camera_direction.normalize_to(frame_step);
        }
    }

    // FIXME: Many magic numbers.
    pub fn render(&mut self) {
        use glium::Surface;

        let mut target = self.display.backend.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);
        let radius = 0.47;

        for x in 0..self.board.width() {
            for y in 0..self.board.height() {
                if self.board[y as usize][x as usize] {
                    let point = Point2::new(x as f32, y as f32);
                    let shade = if (x as f32 - self.mouse_position.x).abs() <= radius &&
                                   (y as f32 - self.mouse_position.y).abs() <= radius {
                        0.7
                    } else {
                        1.0
                    };
                    self.display.draw_quad(&mut target, point, radius, shade);
                }
            }
        }

        self.display.draw_quad(&mut target, self.display.camera.center, 0.1 * radius, 0.5);
        target.finish().unwrap();
    }

    /// Returns whether the key is currently being held down by the user.
    fn is_key_held(&self, key: VirtualKeyCode) -> bool {
        self.held_keys.contains(&(key as usize))
    }

    /// Returns `1.0` if `positive` is held, `-1.0` if `negative` is held, and `0.0` if both or
    /// neither are held.
    fn get_key_direction(&self, positive: VirtualKeyCode, negative: VirtualKeyCode) -> f32 {
        match (self.is_key_held(positive), self.is_key_held(negative)) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        }
    }
}
