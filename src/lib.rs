extern crate nalgebra;
extern crate glium;

use glium::{
    IndexBuffer, VertexBuffer, Display
};
use glium::index::{
    PrimitiveType
};

mod grid;
mod camera;

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

pub use grid::Grid;
pub use camera::FreeCamera;
