/**
 * This recipe uses the vertex shader to create a ripple effect on a planar
 * mesh.
 **/

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate glCookbook;
extern crate nalgebra;

use glutin::{Event};
use glium::{DisplayBuild, Surface, Display};
use glCookbook::{BaseVertex, Grid, FreeCamera};

// Program entry point
fn main() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1366, 768)
        .with_multisampling(4)
        .with_vsync()
        .with_title("tetra".to_string())
        .build_glium()
        .unwrap();

    let grid: Grid<Vertex> = Grid::new(20.0, 20.0, 60, 60);
    let mut camera         = FreeCamera::new(1.0, 75.0, 1.0, 500.0);
    camera.pos.y = 2.0;

    let program    = create_shader_program(&display);
    let vertex_buf = glium::VertexBuffer::new(&display, grid.vertices);
    let indices    = glium::index::IndexBuffer::new(
        &display, glium::index::TrianglesList(grid.indices)
    );

    implement_vertex!(Vertex, position);

    let mut controller = Controller::new();
    controller.rot_speed = 1.0/10.0;

    'mainLoop : loop {
        controller.update(&mut camera, &display);

        let uniforms = uniform!(
            MVP  : camera.projection.to_mat() * camera.get_view_matrix()
        );

        let mut target = display.draw();
        target.clear_color(0.02, 0.02, 0.05, 1.0);
        target.draw(
            &vertex_buf, &indices, &program, &uniforms,
            &std::default::Default::default()
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
    position : [f32; 3]
}

impl BaseVertex for Vertex {
    fn from_position(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { position : [x, y, z] }
    }
}
