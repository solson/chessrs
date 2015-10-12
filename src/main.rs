#[macro_use] extern crate glium;
extern crate time;

use glium::glutin;
use glium::backend::glutin_backend::GlutinFacade;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

struct GameState {
    display: GlutinFacade,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    program: glium::Program,

    last_frame_time: u64,
    triangle_angle: f32,
}

impl GameState {
    fn handle_input(&mut self) {
        for event in self.display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }

    fn update(&mut self) {
        let new_frame_time = time::precise_time_ns();
        let time_diff_ns = new_frame_time - self.last_frame_time;
        self.last_frame_time = new_frame_time;

        self.triangle_angle += time_diff_ns as f32 / 1e9;
    }

    fn render(&mut self) {
        use glium::Surface;

        let t = self.triangle_angle;
        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [ 0.0,     0.0,     1.0, 0.0],
                [ 0.0,     0.0,     0.0, 1.0],
            ]
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }
}

fn open_window() -> Result<GlutinFacade, glium::GliumCreationError<glutin::CreationError>> {
    use glium::DisplayBuild;

    glutin::WindowBuilder::new()
        .with_title(String::from("Chessrs"))
        .with_vsync()
        .build_glium()
}

const VERTEX_SHADER_SOURCE: &'static str = r#"
    #version 140

    in vec2 position;

    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

fn main() {
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let display = open_window().unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, VERTEX_SHADER_SOURCE,
                                              FRAGMENT_SHADER_SOURCE, None).unwrap();

    let mut game = GameState {
        display: display,
        vertex_buffer: vertex_buffer,
        indices: indices,
        program: program,

        last_frame_time: time::precise_time_ns(),
        triangle_angle: 0.0,
    };

    // let mut frame_count = 0;
    // let mut frame_start = time_start_ns;

    loop {
        game.handle_input();
        game.update();
        game.render();

        // frame_count += 1;
        // let frame_seconds = (time_now_ns - frame_start) as f64 / 1e9;
        // if frame_seconds > 5.0 {
        //     println!("FPS: {}", frame_count as f64 / frame_seconds);
        //     frame_count = 0;
        //     frame_start = time_now_ns;
        // }
    }
}
