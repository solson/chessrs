extern crate cgmath;
#[macro_use] extern crate glium;
extern crate time;

use cgmath::Point2;
use glium::glutin;
use glium::backend::glutin_backend::GlutinFacade;

mod board;
use board::Board;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

/// Actions to take from the game loop.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
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

struct GameState {
    display: GlutinFacade,
    shader_program: glium::Program,

    /// Frames-per-second dependent scaling factor, in units of seconds per frame. For an example
    /// of its use, an object moving across the screen at `n` board cells per second should move
    /// `n * time_factor` board cells per frame.
    time_factor: f32,

    /// Units: nanoseconds.
    time_last_frame: u64,

    board: Board<bool>,
    camera_center: Point2<f32>,
    camera_zoom: f32,
}

const DEFAULT_ZOOM: f32 = 1.0 / 7.5;

impl GameState {
    fn new() -> Self {
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
            camera_center: Point2::new(4.0, 2.0),
            camera_zoom: DEFAULT_ZOOM,
        }
    }

    fn handle_input(&mut self) -> Action {
        use glium::glutin::ElementState::*;
        use glium::glutin::Event::*;
        use glium::glutin::MouseScrollDelta::*;
        use glium::glutin::VirtualKeyCode::*;

        // Units: board cells / second
        let camera_speed = 5.0;

        for event in self.display.poll_events() {
            match event {
                Closed => return Action::Stop,

                // FIXME: This camera movement code is utterly dumb.
                KeyboardInput(Pressed, _, Some(key_code)) => match key_code {
                    Up => {
                        self.camera_center.y += camera_speed * self.time_factor;
                    }
                    Down => {
                        self.camera_center.y -= camera_speed * self.time_factor;
                    }
                    Left => {
                        self.camera_center.x -= camera_speed * self.time_factor;
                    }
                    Right => {
                        self.camera_center.x += camera_speed * self.time_factor;
                    }
                    _ => {}
                },

                MouseWheel(LineDelta(_, scroll_amount)) => {
                    // FIXME: Magic numbers.
                    self.camera_zoom *= 1.1 * scroll_amount;
                }

                _ => {},
            }
        }

        Action::None
    }

    fn update(&mut self) {
        let time = time::precise_time_ns();

        // Nanoseconds to seconds.
        self.time_factor = (time - self.time_last_frame) as f32 / 1e9;
        self.time_last_frame = time;
    }

    fn render(&mut self) {
        use glium::Surface;

        let mut target = self.display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);
        let radius = 0.47;

        for i in 0..self.board.width() {
            for j in 0..self.board.height() {
                if self.board[j as usize][i as usize] {
                    let x = i as f32 - self.camera_center.x;
                    let y = j as f32 - self.camera_center.y;
                    self.draw_quad(&mut target, x, y, radius, self.camera_zoom, 1.0);
                }
            }
        }

        self.draw_quad(&mut target, 0.0, 0.0, radius, self.camera_zoom / 10.0, 0.5);
        target.finish().unwrap();
    }

    fn draw_quad(&self, target: &mut glium::Frame, x: f32, y: f32, radius: f32, zoom: f32,
                 shade: f32) {
        use glium::Surface;

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
}

fn open_window() -> Result<GlutinFacade, glium::GliumCreationError<glutin::CreationError>> {
    use glium::DisplayBuild;

    glutin::WindowBuilder::new()
        .with_dimensions(800, 800)
        .with_title(String::from("Chessrs"))
        .with_vsync()
        .build_glium()
}

fn main() {
    let mut game = GameState::new();

    loop {
        match game.handle_input() {
            Action::Stop => break,
            Action::None => {},
        }

        game.update();
        game.render();
    }
}
