/**
 * This recipe uses the vertex shader to create a ripple effect on a planar
 * mesh.
 **/

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate glCookbook;
extern crate nalgebra;
extern crate num;

use std::default::{Default};
use glutin::{Event, VirtualKeyCode};
use glium::{DisplayBuild, Surface, Display};
use glCookbook::{BaseVertex, Grid, FreeCamera};
use nalgebra::{Vec3, Norm};
use num::Float;

// Program entry point
fn main() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1366, 768)
        .with_multisampling(4)
        .with_vsync()
        .with_title("tetra".to_string())
        .build_glium()
        .unwrap();

    let ball: IsoSphere = IsoSphere::new();
    let grid: Grid<Vertex> = Grid::new(20.0, 20.0, 20, 20);
    let mut camera         = FreeCamera::new(1.0, 75.0, 1.0, 500.0);
    camera.pos.y = 2.0;

    let normal_program = create_normal_renderer_program(&display);
    let program        = create_shader_program(&display);

    let vertex_buf = glium::VertexBuffer::new(&display, grid.vertices);
    let indices    = glium::index::IndexBuffer::new(
        &display, glium::index::TrianglesList(grid.indices)
    );

    let ball_v_buf   =
        glium::VertexBuffer::new(&display, ball.faces_to_vertex_array::<Vertex>());
    let ball_indices =
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let ball_norm_indices =
        glium::index::NoIndices(glium::index::PrimitiveType::Points);
    let ball_model = nalgebra::Mat4::new(
        1.0f32, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 2.0,
        0.0, 0.0, 1.0, -20.0,
        0.0, 0.0, 0.0, 1.0
    );

    implement_vertex!(Vertex, position, normal);

    let mut controller = Controller::new();
    controller.rot_speed = 1.0/40.0;
    controller.move_speed = 0.2;

    let mut draw_params = glium::DrawParameters::default();
    draw_params.polygon_mode = glium::PolygonMode::Fill;

    'mainLoop : loop {
        let mv = camera.projection.to_mat() * camera.get_view_matrix();


        let mut target = display.draw();
        target.clear_color(0.02, 0.02, 0.05, 1.0);

        let mut uniforms = uniform!(
            MVP  : mv
        );
        target.draw(
            &vertex_buf, &indices, &program, &uniforms, &draw_params
        ).unwrap();

        uniforms = uniform!{
            MVP : mv * ball_model
        };
        target.draw(
            &ball_v_buf, &ball_norm_indices, &normal_program, &uniforms, &draw_params
        ).unwrap();

        target.draw(
            &ball_v_buf, &ball_indices, &program, &uniforms, &draw_params
        ).unwrap();

        target.finish();

        for event in display.poll_events() {
            match event {
                Event::Closed => break 'mainLoop,
                Event::Resized(w, h) => {
                    camera.projection.set_aspect((w as f32)/(h as f32));
                },
                _ => ()
            }
            controller.process_event(&event);
        }
        controller.update(&mut camera, &display);
    }
}

fn create_shader_program(display: &Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 330
        in vec3 position;

        uniform mat4 MVP;

        void main() {
            gl_Position = MVP * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330
        out vec4 vFragColor;
        void main() {
            vFragColor = vec4(1.0);
        }
    "#;

    glium::Program::from_source(
        display, vertex_shader_src, fragment_shader_src, None
    ).unwrap()
}

fn create_normal_renderer_program(display: &Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 330
        in vec3 position;
        in vec3 normal;
        out vec3 g_normal;

        void main() {
            g_normal = normal;
            gl_Position = vec4(position, 1.0);
        }
    "#;

    let geometry_shader_src = r#"
        #version 330
        layout(points) in;

        layout(line_strip, max_vertices = 2) out;

        uniform mat4 MVP;

        in vec3 g_normal[];

        void main() {
            vec4 v0 = gl_in[0].gl_Position;
            gl_Position = MVP * v0;
            EmitVertex();

            vec4 v1 = v0 + vec4(g_normal[0] * 0.5, 0);
            gl_Position = MVP * v1;
            EmitVertex();

            EndPrimitive();
        }
    "#;

    let fragment_shader_src = r#"
        #version 330
        out vec4 frag_color;
        void main() {
            frag_color = vec4(0.0, 0.0, 0.7, 1.0);
        }
    "#;

    glium::Program::from_source(
        display, vertex_shader_src, fragment_shader_src, Some(geometry_shader_src)
    ).unwrap()
}

struct Face {
    pub v1 : Vec3<f32>,
    pub v2 : Vec3<f32>,
    pub v3 : Vec3<f32>
}

impl Face {
    fn from_vec3(v1: Vec3<f32>, v2: Vec3<f32>, v3: Vec3<f32>) -> Face {
        Face {
            v1: v1,
            v2: v2,
            v3: v3
        }
    }
}

struct IsoSphere {
    faces        : Vec<Face>
}

impl IsoSphere {
    fn new() -> IsoSphere {
        let mut sphere = IsoSphere { faces : vec![] };

        sphere.generate_icosahedron();
        sphere
    }

    fn faces_to_vertex_array<T: NormalVertex>(&self) -> Vec<T> {
        let mut vertices = vec![];
        for face in &self.faces {
            vertices.push(IsoSphere::vertex_from_vec(face.v1));
            vertices.push(IsoSphere::vertex_from_vec(face.v2));
            vertices.push(IsoSphere::vertex_from_vec(face.v3));
        }
        vertices
    }

    fn vertex_from_vec<T: NormalVertex>(vec: Vec3<f32>) -> T {
        let mut vert = T::from_position(vec.x, vec.y, vec.z);
        vert.set_normal(vec.x, vec.y, vec.z);
        vert
    }

    fn subdivide_faces(&mut self) {
        let mut new_faces = vec![];

        for face in &self.faces {
            let Face { v1, v2, v3 } = *face;
            let a = ((v1 + v2) * 1.0/2.0).normalize();
            let b = ((v2 + v3) * 1.0/2.0).normalize();
            let c = ((v1 + v3) * 1.0/2.0).normalize();

            new_faces.push(Face::from_vec3(v1, a, v3));
            new_faces.push(Face::from_vec3(a, b, c));
            new_faces.push(Face::from_vec3(a, v2, b));
            new_faces.push(Face::from_vec3(c, b, v3));
        }

        self.faces = new_faces;
    }

    fn generate_icosahedron(&mut self) {
        let t = (1.0 + 5.0.sqrt())/2.0;
        let p0  = Vec3::new( 0.0,  t,  1.0).normalize();
        let p1  = Vec3::new( 0.0,  t, -1.0).normalize();
        let p2  = Vec3::new( 0.0, -t,  1.0).normalize();
        let p3  = Vec3::new( 0.0, -t, -1.0).normalize();

        let p4  = Vec3::new(-1.0, 0.0,  -t).normalize();
        let p5  = Vec3::new( 1.0, 0.0,  -t).normalize();
        let p6  = Vec3::new( 1.0, 0.0,   t).normalize();
        let p7  = Vec3::new(-1.0, 0.0,   t).normalize();

        let p8  = Vec3::new(-t,  1.0,  0.0).normalize();
        let p9  = Vec3::new(-t, -1.0,  0.0).normalize();
        let p10 = Vec3::new( t,  1.0,  0.0).normalize();
        let p11 = Vec3::new( t, -1.0,  0.0).normalize();

        self.faces = vec![
            Face::from_vec3(p1, p10, p5),
            Face::from_vec3(p1, p5, p4),
            Face::from_vec3(p1, p8, p4),
            Face::from_vec3(p1, p8, p0),
            Face::from_vec3(p1, p0, p10),

            Face::from_vec3(p7, p0, p6),
            Face::from_vec3(p7, p6, p2),
            Face::from_vec3(p7, p2, p9),
            Face::from_vec3(p7, p9, p8),
            Face::from_vec3(p7, p8, p0),

            Face::from_vec3(p11, p10, p5),
            Face::from_vec3(p11, p5, p3),
            Face::from_vec3(p11, p3, p2),
            Face::from_vec3(p11, p2, p6),
            Face::from_vec3(p11, p6, p10),

            Face::from_vec3(p0, p6, p10),
            Face::from_vec3(p8, p4, p9),
            Face::from_vec3(p9, p4, p3),
            Face::from_vec3(p9, p3, p2),
            Face::from_vec3(p4, p3, p5)
        ];
    }
}

struct Controller {
    pub rx         : f32,
    pub ry         : f32,
    pub front      : bool,
    pub back       : bool,
    pub left       : bool,
    pub right      : bool,
    pub move_speed : f32,
    pub rot_speed  : f32,
    center_x       : i32,
    center_y       : i32
}

impl Controller {
    fn new() -> Controller {
        Controller {
            rx         : 0.0,
            ry         : 0.0,
            front      : false,
            back       : false,
            left       : false,
            right      : false,
            move_speed : 1.0,
            rot_speed  : 1.0,
            center_x   : 0,
            center_y   : 0
        }
    }

    fn process_event(&mut self, event: &Event) {
        match *event {
            Event::Resized(w, h) => {
                self.center_x = (w / 2) as i32;
                self.center_y = (h / 2) as i32;
            },
            Event::MouseMoved((x, y)) => {
                if !(x == self.center_x && y == self.center_y) {
                    self.rx = (x - self.center_x) as f32;
                    self.ry = (self.center_y - y) as f32;
                }
                else {
                    self.rx = 0.0;
                    self.ry = 0.0;
                }
            },
            Event::KeyboardInput(state, _, Some(k)) => {
                let pressed = state == glutin::ElementState::Pressed;
                match k {
                    VirtualKeyCode::Comma => self.front = pressed,
                    VirtualKeyCode::O     => self.back  = pressed,
                    VirtualKeyCode::A     => self.left  = pressed,
                    VirtualKeyCode::E     => self.right = pressed,
                    _ => ()
                }
            }
            _ => ()
        }
    }

    fn update(&self, camera: &mut FreeCamera, display: &glium::Display) {
        if self.front {
            camera.advance(self.move_speed);
        }
        if self.back {
            camera.advance(-self.move_speed);
        }
        if self.right {
            camera.strafe(self.move_speed);
        }
        if self.left {
            camera.strafe(-self.move_speed);
        }
        camera.rotate_up(self.ry * self.rot_speed);
        camera.rotate_left(-self.rx * self.rot_speed);

        // snap mouse to the center of the screen
        let _ = (*display.get_window().unwrap())
            .set_cursor_position(self.center_x, self.center_y);
    }
}

#[derive(Clone, Copy)]
struct Vertex {
    position : [f32; 3],
    normal   : [f32; 3]
}

impl BaseVertex for Vertex {
    fn from_position(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { position : [x, y, z], normal : [0.0, 0.0, 0.0] }
    }
}

trait NormalVertex : BaseVertex {
    fn set_normal(&mut self, x: f32, y: f32, z: f32) -> ();
}

impl NormalVertex for Vertex {
    fn set_normal(&mut self, x: f32, y: f32, z: f32) {
        self.normal = [x, y, z];
    }
}







