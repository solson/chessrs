use glium;
use glium::glutin;
use glium::backend::glutin_backend::GlutinFacade;

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

pub struct Display {
    pub backend: GlutinFacade,
    shader_program: glium::Program,
}

impl Display {
    pub fn new_window() -> Self {
        use glium::DisplayBuild;

        let backend = glutin::WindowBuilder::new()
            .with_dimensions(800, 800)
            .with_title(String::from("Chessrs"))
            .with_vsync()
            .build_glium()
            .unwrap();

        let shader_program = glium::Program::from_source(
            &backend, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None).unwrap();

        Display {
            backend: backend,
            shader_program: shader_program,
        }
    }

    pub fn draw_quad(&self, target: &mut glium::Frame, x: f32, y: f32, radius: f32, zoom: f32,
                     shade: f32) {
        use glium::Surface;

        // Top/bottom, left/right.
        let tl = Vertex { position: [(x - radius) * zoom, (y - radius) * zoom] };
        let tr = Vertex { position: [(x + radius) * zoom, (y - radius) * zoom] };
        let br = Vertex { position: [(x + radius) * zoom, (y + radius) * zoom] };
        let bl = Vertex { position: [(x - radius) * zoom, (y + radius) * zoom] };
        let vertices = [tl, br, tr, tl, bl, br];

        let vertex_buffer = glium::VertexBuffer::new(&self.backend, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let uniforms = uniform! { shade: shade };

        target.draw(&vertex_buffer, &indices, &self.shader_program, &uniforms,
                    &Default::default()).unwrap();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
