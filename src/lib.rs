extern crate bit_set;
extern crate cgmath;
#[macro_use] extern crate glium;
extern crate time;

mod board;
mod camera;
pub mod units;

use bit_set::BitSet;
use cgmath::{Point2, Vector2, EuclideanVector, Vector};
use glium::glutin::{self, VirtualKeyCode};
use glium::backend::glutin_backend::GlutinFacade;

use board::Board;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

/// Actions to take from the game loop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    None,
    Stop,
}

const VERTEX_SHADER_SOURCE: &'static str = r#"
    #version 140
    in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"
    #version 140
    out vec4 color;
    uniform float shade;
    void main() {
        color = vec4(shade, shade, shade, 1.0);
    }
"#;

pub struct GameState {
    display: GlutinFacade,
    shader_program: glium::Program,

    /// Frames-per-second dependent scaling factor, in units of seconds per frame. For an example
    /// of its use, an object moving across the screen at `n` board cells per second should move
    /// `n * time_factor` board cells per frame.
    time_factor: f32,

    /// Units: nanoseconds.
    time_last_frame: u64,

    board: Board<bool>,
    camera: camera::Camera,
    held_keys: BitSet,
}

impl GameState {
    pub fn new() -> Self {
        let display = open_window().unwrap();
        let shader_program = glium::Program::from_source(
            &display, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None).unwrap();

        GameState {
            display: display,
            shader_program: shader_program,

            // HACK: Assumes 60 fps. On the other hand, it's only for the first frame.
            time_factor: 1.0 / 60.0,
            time_last_frame: time::precise_time_ns(),

            board: Board::new_test_board(),

            camera: camera::Camera {
                center: Point2::new(4.0, 2.0),
                zoom: camera::ZOOM_DEFAULT,
            },

            held_keys: BitSet::new(),
        }
    }

    pub fn handle_input(&mut self) -> Action {
        use glium::glutin::ElementState::*;
        use glium::glutin::Event::*;
        use glium::glutin::MouseScrollDelta::*;

        for event in self.display.poll_events() {
            match event {
                Closed => return Action::Stop,

                KeyboardInput(Pressed, _, Some(key)) => {
                    self.held_keys.insert(key as usize);
                }

                KeyboardInput(Released, _, Some(key)) => {
                    self.held_keys.remove(&(key as usize));
                }

                MouseWheel(LineDelta(_, scroll_amount)) => {
                    self.camera.zoom_steps(scroll_amount);
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
            self.camera.center = self.camera.center + camera_direction.normalize_to(frame_step);
        }
    }

    // FIXME: Many magic numbers.
    pub fn render(&mut self) {
        use glium::Surface;

        let mut target = self.display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);
        let radius = 0.47;

        for i in 0..self.board.width() {
            for j in 0..self.board.height() {
                if self.board[j as usize][i as usize] {
                    let x = i as f32 - self.camera.center.x;
                    let y = j as f32 - self.camera.center.y;
                    self.draw_quad(&mut target, x, y, radius, 1.0, 1.0);
                }
            }
        }

        self.draw_quad(&mut target, 0.0, 0.0, radius, 0.1, 0.5);
        target.finish().unwrap();
    }

    fn draw_quad(&self, target: &mut glium::Frame, x: f32, y: f32, radius: f32, mut zoom: f32,
                 shade: f32) {
        use glium::Surface;

        zoom *= self.camera.zoom_factor();

        // Top/bottom, left/right.
        let tl = Vertex { position: [(x - radius) * zoom, (y - radius) * zoom] };
        let tr = Vertex { position: [(x + radius) * zoom, (y - radius) * zoom] };
        let br = Vertex { position: [(x + radius) * zoom, (y + radius) * zoom] };
        let bl = Vertex { position: [(x - radius) * zoom, (y + radius) * zoom] };
        let vertices = [tl, br, tr, tl, bl, br];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let uniforms = uniform! { shade: shade };

        target.draw(&vertex_buffer, &indices, &self.shader_program, &uniforms,
                    &Default::default()).unwrap();
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

fn open_window() -> Result<GlutinFacade, glium::GliumCreationError<glutin::CreationError>> {
    use glium::DisplayBuild;

    glutin::WindowBuilder::new()
        .with_dimensions(800, 800)
        .with_title(String::from("Chessrs"))
        .with_vsync()
        .build_glium()
}
