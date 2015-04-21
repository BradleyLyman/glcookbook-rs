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

use glutin::{Event, ElementState, VirtualKeyCode};
use glium::{DisplayBuild, Surface, Display, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType, IndexBuffer};
use glCookbook::{
    Vertex, Grid, RenderableIndices, RenderableObj,
    BuildRenderable,
    Controller, FreeCamera, LightingRenderer,
    NormalRenderer
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

    let grid = RenderableObj::new(&TerrainMeshCenter, &display);
    let ring = RenderableObj::new(&TerrainRing, &display);
    let mut lighting_renderer = LightingRenderer::new(&display);
    let normal_renderer       = NormalRenderer::new(&display);
    let mut draw_normals      = false;
    let mut camera            = FreeCamera::new(1.0, 75.0, 1.0, 500.0);

    camera.pos.y = 2.0;
    lighting_renderer.light_position = Vec3::new(0.0, 5.0, 0.0);
    lighting_renderer.diffuse_color  = Vec3::new(0.8, 0.8, 0.8);
    lighting_renderer.specular_color = Vec3::new(0.5, 0.6, 0.5);
    lighting_renderer.shininess      = 128.0;
    lighting_renderer.wire           = false;


    let mut controller = Controller::new();
    controller.rot_speed = 1.0/40.0;
    controller.move_speed = 0.2;

    'mainLoop : loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.02, 0.02, 0.05, 1.0), 1.0);
        lighting_renderer.draw(
            &mut target, &grid, &camera.projection.to_mat(),
            &camera.get_view_transform(), &Iso3::new(nalgebra::zero(), nalgebra::zero())
        );

        lighting_renderer.draw(
            &mut target, &ring, &camera.projection.to_mat(),
            &camera.get_view_transform(), &Iso3::new(nalgebra::zero(), nalgebra::zero())
        );

        if draw_normals {
            normal_renderer.draw(
                &mut target, &grid, &camera.projection.to_mat(),
                &camera.get_view_transform(), &Iso3::new(nalgebra::zero(), nalgebra::zero())
            );

            normal_renderer.draw(
                &mut target, &ring, &camera.projection.to_mat(),
                &camera.get_view_transform(), &Iso3::new(nalgebra::zero(), nalgebra::zero())
            );
        }

        target.finish();

        for event in display.poll_events() {
            match event {
                Event::Closed => break 'mainLoop,
                Event::Resized(w, h) => {
                    camera.projection.set_aspect((w as f32)/(h as f32));
                },
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Space)) => {
                    lighting_renderer.wire = !lighting_renderer.wire;
                },
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Return)) => {
                    draw_normals = !draw_normals;
                },
                _ => ()
            }
            controller.process_event(&event);
        }
        controller.update(&mut camera, &display);
    }
}

struct TerrainMeshCenter;

impl BuildRenderable for TerrainMeshCenter {
    fn get_vertex_array(&self, display: &Display) -> VertexBuffer<Vertex> {
        let mut vertices = vec![];

        for x in -4..4 {
            for z in -4..4 {
                let mut vertex = Vertex::from_position(x as f32, 0.0, z as f32);
                vertex.normal  = [0.0, 1.0, 0.0];
                vertices.push(vertex);
            }
        }

        VertexBuffer::new(display, vertices)
    }

    fn get_indices(&self, display: &Display) -> RenderableIndices {
        let mut ind = vec![];

        for row in 0..7 {
            for col in 0..7 {
                let tl = (row * 8 + col) as u16;
                let tr = tl + 8u16;
                let bl = tl + 1u16;
                let br = bl + 8u16;

                ind.push(tl);
                ind.push(bl);
                ind.push(tr);

                ind.push(tr);
                ind.push(bl);
                ind.push(br);
            }
        }

        RenderableIndices::Buffer(
            IndexBuffer::from_raw(display, ind, PrimitiveType::TrianglesList)
        )
    }
}

struct TerrainRing;

impl BuildRenderable for TerrainRing {
    fn get_vertex_array(&self, display: &Display) -> VertexBuffer<Vertex> {
        let mut vertices = vec![];

        // right side
        for x in 3..8 {
            for z in -7..8 {
                let mut vert = Vertex::from_position(x as f32, 0.0, z as f32);
                vert.normal = [0.0, 1.0, 0.0];
                vertices.push(vert);
            }
        }

        // left side
        for x in -7..-3 {
            for z in -7..8 {
                let mut vert = Vertex::from_position(x as f32, 0.0, z as f32);
                vert.normal = [0.0, 1.0, 0.0];
                vertices.push(vert);
            }
        }

        // top side
        for x in -4..4 {
            for z in 3..8 {
                let mut vert = Vertex::from_position(x as f32, 0.0, z as f32);
                vert.normal = [0.0, 1.0, 0.0];
                vertices.push(vert);
            }
        }

        // bottom side
        for x in -4..4 {
            for z in -7..-3 {
                let mut vert = Vertex::from_position(x as f32, 0.0, z as f32);
                vert.normal = [0.0, 1.0, 0.0];
                vertices.push(vert);
            }
        }

        VertexBuffer::new(display, vertices)
    }

    fn get_indices(&self, display: &Display) -> RenderableIndices {
        let mut ind = vec![];

        let mut offset = 0;
        // right side
        for row in 0..4 {
            for col in 0..14 {
                let tl = (row * 15 + col) as u16;
                let tr = tl + 15u16;
                let bl = tl + 1u16;
                let br = bl + 15u16;

                ind.push(tl);
                ind.push(bl);
                ind.push(tr);

                ind.push(tr);
                ind.push(bl);
                ind.push(br);
            }
        }
        offset = 75;

        for row in 0..3 {
            for col in 0..14 {
                let tl = (row * 15 + col + offset) as u16;
                let tr = tl + 15u16;
                let bl = tl + 1u16;
                let br = bl + 15u16;

                ind.push(tl);
                ind.push(bl);
                ind.push(tr);

                ind.push(tr);
                ind.push(bl);
                ind.push(br);
            }
        }
        offset = 75 + 60;

        for row in 0..7 {
            for col in 0..4 {
                let tl = (row * 5 + col + offset) as u16;
                let tr = tl + 5u16;
                let bl = tl + 1u16;
                let br = bl + 5u16;

                ind.push(tl);
                ind.push(bl);
                ind.push(tr);

                ind.push(tr);
                ind.push(bl);
                ind.push(br);
            }
        }
        offset = 75 + 60 + 40;

        for row in 0..7 {
            for col in 0..3 {
                let tl = (row * 4 + col + offset) as u16;
                let tr = tl + 4u16;
                let bl = tl + 1u16;
                let br = bl + 4u16;

                ind.push(tl);
                ind.push(bl);
                ind.push(tr);

                ind.push(tr);
                ind.push(bl);
                ind.push(br);
            }
        }


        RenderableIndices::Buffer(
            IndexBuffer::from_raw(display, ind, PrimitiveType::TrianglesList)
        )
    }
}













