/**
 * This recipe uses the vertex shader to create a ripple effect on a planar
 * mesh.
 **/

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate nalgebra;

use glutin::{Event};
use glium::{DisplayBuild, Surface, Display};
use nalgebra::{PerspMat3, Iso3, Vec3, Mat4, ToHomogeneous, Rotation};

static GRID_WIDTH: f32 = 20f32;
static GRID_DEPTH: f32 = 20f32;
static X_COUNT: u16 = 60;
static Z_COUNT: u16 = 60;

// Program entry point
fn main() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1366, 768)
        .with_multisampling(4)
        .with_vsync()
        .with_title("tetra".to_string())
        .build_glium()
        .unwrap();

    let program    = create_shader_program(&display);
    let vertices   = build_mesh_vertices();
    let vertex_buf = glium::VertexBuffer::new(&display, vertices);
    let indices    = glium::index::IndexBuffer::new(
        &display, glium::index::TrianglesList(build_mesh_indices())
    );

    implement_vertex!(Vertex, position);

    let mut projection = create_projection();
    let mut time       = 0.0f32;
    let mut model      = Iso3::new(nalgebra::zero(), nalgebra::zero());
    let view           = Iso3::new(Vec3::new(0.0, 0.0, 30.0), nalgebra::zero()).to_homogeneous();

    let mut draw_params: glium::DrawParameters = std::default::Default::default();
    draw_params.polygon_mode = glium::PolygonMode::Line;

    let (mut center_x, mut center_y): (i32, i32) = (0, 0);
    let (mut rx, mut ry): (f32, f32) = (0.0, 0.0);

    'mainLoop : loop {
        time += 0.01f32;

        model.rotation = model.rotation.append_rotation(&(Vec3::y() * rx));
        model.rotation = model.rotation.append_rotation(&(Vec3::x() * ry));
        let mvp = *projection.as_mat() * view * model.to_homogeneous();

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
                    projection.set_aspect((w as f32)/(h as f32));
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

fn create_projection() -> PerspMat3<f32> {
    PerspMat3::new(1.0, 75.0 * 3.14159 / 180.0, 1.0, 200.0)
}

fn create_shader_program(display: &Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 330
        in vec3 position;

        uniform mat4 MVP;
        uniform float time;

        const float amplitude = 15.0;
        const float frequency = 0.5;
        const float PI = 3.14159;

        void main() {
            float distance = max(length(position), 0.0000001);
            float y = 1.0 / distance * amplitude*sin(-PI*distance*frequency+time);
            gl_Position = MVP * vec4(position.x, position.z, y, 1.0);
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

fn build_mesh_vertices() -> Vec<Vertex> {
    let mut vertices = vec![];
    for j in 0..Z_COUNT {
        // scaled_j is a value between -1 and 1
        let scaled_j = ((j as f32)/(Z_COUNT as f32 - 1.0)) * 2.0 - 1.0;

        for i in 0..X_COUNT {
            let scaled_i = ((i as f32)/(X_COUNT as f32 - 1.0)) * 2.0 - 1.0;

            vertices.push(Vertex { position : [
                scaled_i * GRID_WIDTH, -1.0, scaled_j * GRID_DEPTH
            ] });
        }
    }

    vertices
}

fn build_mesh_indices() -> Vec<u16> {
    let mut indices = vec![];
    let mut count = 0;

    for row in 0..Z_COUNT-1 {
        for col in 0..X_COUNT-1 {
            let tl = row * X_COUNT + col;
            let bl = tl + 1;
            let tr = tl + X_COUNT;
            let br = tr + 1;

            if count % 2 == 0 {
                indices.push(tl);
                indices.push(bl);
                indices.push(br);

                indices.push(tl);
                indices.push(br);
                indices.push(tr);
            }
            else {
                indices.push(tl);
                indices.push(bl);
                indices.push(tr);

                indices.push(tr);
                indices.push(bl);
                indices.push(br);
            }
            count += 1;
        }
    }
    indices
}


#[derive(Clone, Copy)]
struct Vertex {
    position : [f32; 3]
}
