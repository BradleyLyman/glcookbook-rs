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

use glutin::{Event};
use glium::{DisplayBuild, Surface, Display};
use glium::index::{NoIndices, PrimitiveType};
use glCookbook::{
    Vertex, RenderableIndices, Renderable,
    Controller, FreeCamera, LightingRenderer
};
use nalgebra::{Vec3, Mat4, Iso3, Transformation};

// Program entry point
fn main() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1366, 768)
        .with_multisampling(4)
        .with_depth_buffer(24)
        .with_vsync()
        .with_title("tetra".to_string())
        .build_glium()
        .unwrap();

    let mut lighting_renderer = LightingRenderer::new(&display);
    let mut camera            = FreeCamera::new(1.0, 75.0, 1.0, 500.0);

    camera.pos.y = 2.0;
    lighting_renderer.light_position = Vec3::new(0.0, 10.0, 0.0);
    lighting_renderer.diffuse_color = Vec3::new(0.2, 0.2, 0.8);
    lighting_renderer.specular_color = Vec3::new(0.8, 0.8, 0.8);
    lighting_renderer.shininess = 256.0;


    let mut controller = Controller::new();
    controller.rot_speed = 1.0/40.0;
    controller.move_speed = 0.2;

    'mainLoop : loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.02, 0.02, 0.05, 1.0), 1.0);

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

struct ResourceCache {
    vertices: glium::VertexBuffer<Vertex>,
    indices : RenderableIndices
}


