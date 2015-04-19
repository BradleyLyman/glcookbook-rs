extern crate nalgebra;
extern crate glium;
extern crate glutin;
extern crate num;

use glium::{
    IndexBuffer, VertexBuffer, Display
};
use glium::index::{
    PrimitiveType
};
use glutin::{Event, VirtualKeyCode};

mod grid;
mod camera;
mod iso_sphere;

pub use grid::Grid;
pub use camera::FreeCamera;
pub use iso_sphere::IsoSphere;



pub trait BaseVertex: Copy + Clone {
    fn from_position(x: f32, y: f32, z: f32) -> Self;
}

pub trait NormalVertex: BaseVertex {
    fn set_normal(&mut self, x: f32, y: f32, z: f32);
}

pub enum RenderableIndices {
    None(PrimitiveType),
    Buffer(IndexBuffer)
}

pub trait RenderableVertex:
    'static + NormalVertex + glium::vertex::Vertex + std::marker::Send {}

pub trait Renderable {
    fn get_vertex_array<T: RenderableVertex>(
        &self, display: &Display
    ) -> VertexBuffer<T>;

    fn get_indices(&self, display: &Display) -> RenderableIndices;
}

pub struct Controller {
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
    pub fn new() -> Controller {
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

    pub fn process_event(&mut self, event: &Event) {
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

    pub fn update(&self, camera: &mut FreeCamera, display: &Display) {
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
