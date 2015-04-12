extern crate nalgebra;

pub trait BaseVertex: Copy + Clone {
    fn from_position(x: f32, y: f32, z: f32) -> Self;
}

pub use grid::Grid;

mod grid;
