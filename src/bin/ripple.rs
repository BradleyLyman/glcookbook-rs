/**
 * This recipe uses the vertex shader to create a ripple effect on a planar
 * mesh.
 **/

#[macro_use]
extern crate glium;
extern crate glutin;

use glutin::{Event};
use glium::{DisplayBuild, Surface};


// Program entry point
fn main() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1366, 768)
        .with_multisampling(4)
        .with_vsync()
        .with_title("tetra".to_string())
        .build_glium()
        .unwrap();

    'mainLoop : loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish();

        for event in display.poll_events() {
            match event {
                Event::Closed => break 'mainLoop,
                _ => ()
            }
        }
    }
}


#[derive(Clone, Copy)]
struct Vertex {
    position : [f32; 3]
}
