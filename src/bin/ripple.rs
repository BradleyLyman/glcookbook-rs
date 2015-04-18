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
use nalgebra::{Iso3, Vec3, ToHomogeneous, Rotation};
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

    let grid       = Grid::new(20.0, 20.0, 60, 60);
    let vertex_buf = glium::VertexBuffer::new(&display, grid.get_vertices::<Vertex>());
    let indices    = glium::index::IndexBuffer::new(
        &display, glium::index::TrianglesList(grid.indices)
    );

    let program = create_shader_program(&display);

    implement_vertex!(Vertex, position);

    let mut time       = 0.0f32;
    let mut model      = Iso3::new(nalgebra::zero(), nalgebra::zero());
    let mut camera = FreeCamera::new(1.0, 75.0, 1.0, 500.0);
    camera.pos.z = 30.0;

    let mut draw_params: glium::DrawParameters = std::default::Default::default();
    draw_params.polygon_mode = glium::PolygonMode::Line;

    let (mut center_x, mut center_y): (i32, i32) = (0, 0);
    let (mut rx, mut ry): (f32, f32) = (0.0, 0.0);

    'mainLoop : loop {
        time += 0.01f32;

        model.rotation = model.rotation.append_rotation(&(Vec3::y() * rx));
        model.rotation = model.rotation.append_rotation(&(Vec3::x() * ry));
        let mvp = camera.projection.to_mat() * camera.get_view_matrix() * model.to_homogeneous();

        let uniforms = uniform!(
            MVP  : mvp,
            time : time
        );

        let mut target = display.draw();
        target.clear_color(0.02, 0.02, 0.05, 1.0);
        target.draw(
            &vertex_buf, &indices, &program, &uniforms,
            &draw_params
        ).unwrap();
        target.finish();

        for event in display.poll_events() {
            match event {
                Event::Closed => break 'mainLoop,
                Event::Resized(w, h) => {
                    camera.projection.set_aspect((w as f32)/(h as f32));
                    center_x = (w as i32) / 2;
                    center_y = (h as i32) / 2;
                },
                Event::MouseMoved((x, y)) => {
                    if !(x == center_x && y == center_y) {
                        let sx = x - center_x;
                        let sy = center_y - y;
                        rx = (sx as f32) / 20.0 * 3.1415 / 180.0;
                        ry = (sy as f32) / 20.0 * 3.1415 / 180.0;
                    }
                    else {
                        rx = 0.0;
                        ry = 0.0;
                    }
                }
                _ => ()
            }
        }
        let _ = (*display.get_window().unwrap())
            .set_cursor_position(center_x, center_y);
    }
}

fn create_shader_program(display: &Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 330
        in vec3 position;

        uniform mat4 MVP;
        uniform float time;

        const float amplitude = 2.0;
        const float frequency = 0.5;
        const float PI = 3.14159;

        void main() {
            float distance = length(position * frequency);
            float y = amplitude*sin(-PI*distance+time);
            gl_Position = MVP * vec4(position.x, y, position.z, 1.0);
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

#[derive(Clone, Copy)]
struct Vertex {
    position : [f32; 3]
}

impl BaseVertex for Vertex {
    fn from_position(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { position : [x, y, z] }
    }
}
