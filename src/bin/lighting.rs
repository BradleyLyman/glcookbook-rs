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
    Grid, FreeCamera,
    RenderableObj,
    Controller, IsoSphere, LightingRenderer,
    NormalRenderer
};
use nalgebra::{Vec3, Mat4, Iso3, Transformation};
use num::Float;

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

    let ball = RenderableObj::new(&IsoSphere::new(3), &display);
    let grid = RenderableObj::new(&Grid::new(20.0, 20.0, 20, 20), &display);
    let ball_model =
        nalgebra::Iso3::new(Vec3::new(0.0, 2.0, 0.0), nalgebra::zero());

    let mut lighting_renderer = LightingRenderer::new(&display);
    let normal_renderer       = NormalRenderer::new(&display);
    let mut camera            = FreeCamera::new(1.0, 75.0, 1.0, 500.0);
    let mut time = 0.0f32;

    camera.pos.y = 2.0;
    lighting_renderer.light_position = Vec3::new(10.0, 2.0, 0.0);
    lighting_renderer.diffuse_color = Vec3::new(0.2, 0.2, 0.8);
    lighting_renderer.specular_color = Vec3::new(0.8, 0.8, 0.8);
    lighting_renderer.shininess = 256.0;


    let mut controller = Controller::new();
    controller.rot_speed = 1.0/40.0;
    controller.move_speed = 0.2;

    'mainLoop : loop {
        time += 0.02;
        let (x, z) = time.sin_cos();
        lighting_renderer.light_position = Vec3::new(x*10.0, 2.0, z*10.0);

        let mut target = display.draw();
        target.clear_color_and_depth((0.02, 0.02, 0.05, 1.0), 1.0);


        lighting_renderer.draw(
            &mut target, &grid, &camera.projection.to_mat(),
            &camera.get_view_transform(), &Iso3::new(nalgebra::zero(), nalgebra::zero())
        );

        lighting_renderer.draw(
            &mut target, &ball, &camera.projection.to_mat(),
            &camera.get_view_transform(), &ball_model
        );

        normal_renderer.draw(
            &mut target, &ball, &camera.projection.to_mat(),
            &camera.get_view_transform(), &ball_model
        );

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

