use cgmath::{Matrix4, Point, Point2};
use glium::{self, glutin};

use camera::{self, Camera};

const VERTEX_SHADER_SOURCE: &'static str = r#"
    #version 140
    in vec2 position;
    uniform mat4 projection;
    void main() {
        gl_Position = projection * vec4(position, 0.0, 1.0);
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
    pub backend: glium::Display,
    shader_program: glium::Program,
    width: u32,
    height: u32,
    pub camera: Camera,
}

impl Display {
    pub fn new_window() -> Self {
        use glium::DisplayBuild;

        let monitor = glutin::get_primary_monitor();
        let (width, height) = monitor.get_dimensions();

        let backend = glutin::WindowBuilder::new()
            // .with_dimensions(width, height)
            .with_fullscreen(monitor)
            .with_title(String::from("Chessrs"))
            .with_vsync()
            .build_glium()
            .unwrap();

        let shader_program = glium::Program::from_source(
            &backend, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None).unwrap();

        Display {
            backend: backend,
            shader_program: shader_program,
            width: width,
            height: height,
            camera: Camera {
                center: Point2::origin(),
                zoom: camera::ZOOM_DEFAULT,
            },
        }
    }

    pub fn draw_quad(&self, target: &mut glium::Frame, x: f32, y: f32, radius: f32, shade: f32) {
        use glium::Surface;

        let zoom = self.camera.zoom_factor();

        // Top/bottom, left/right.
        let tl = Vertex { position: [(x - radius) * zoom, (y - radius) * zoom] };
        let tr = Vertex { position: [(x + radius) * zoom, (y - radius) * zoom] };
        let br = Vertex { position: [(x + radius) * zoom, (y + radius) * zoom] };
        let bl = Vertex { position: [(x - radius) * zoom, (y + radius) * zoom] };
        let vertices = [tl, br, tr, tl, bl, br];

        let vertex_buffer = glium::VertexBuffer::new(&self.backend, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let uniforms = uniform! {
            projection: self.scale_aspect_ratio(),
            shade: shade,
        };

        target.draw(&vertex_buffer, &indices, &self.shader_program, &uniforms,
                    &Default::default()).unwrap();
    }

    /// Create a transformation matrix to correct for stretching due to non-square aspect ratios.
    fn scale_aspect_ratio(&self) -> [[f32; 4]; 4] {
        let inv_aspect_ratio = self.height as f32 / self.width as f32;
        Matrix4::from_nonuniform_scale(inv_aspect_ratio, 1.0, 1.0).into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
