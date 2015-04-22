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
use glium::{DisplayBuild, Surface, Display, VertexBuffer, PolygonMode, Program, DepthTest, DrawParameters, Frame};
use glium::index::{NoIndices, PrimitiveType, IndexBuffer};
use glium::texture::{
    Texture2d, UncompressedFloatFormat
};
use glCookbook::{
    Vertex, Grid, RenderableIndices, RenderableObj,
    BuildRenderable,
    Controller, FreeCamera, LightingRenderer,
    NormalRenderer
};
use nalgebra::{Vec3, Mat4, Iso3, Transformation, to_homogeneous, Inv};


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
    let normal_renderer      = NormalRenderer::new(&display);
    let mut terrain_renderer = TerrainRenderer::new(&display);
    let mut draw_normals     = false;
    let mut camera           = FreeCamera::new(1.0, 75.0, 1.0, 500.0);

    let heightmap = Texture2d::empty_with_format(
        &display, UncompressedFloatFormat::F32, false, 16, 16
    ).unwrap();

    camera.pos.y = 2.0;

    let mut controller = Controller::new();
    controller.rot_speed = 1.0/40.0;
    controller.move_speed = 0.2;

    let generator = HeightmapGenerator::new(&display);
    generator.generate(&heightmap);

    'mainLoop : loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.02, 0.02, 0.05, 1.0), 1.0);
        terrain_renderer.level = 1;
        terrain_renderer.draw(
            &mut target, &grid, &camera.projection.to_mat(),
            &camera.get_view_transform(), &Iso3::new(nalgebra::zero(), nalgebra::zero()), &heightmap
        );

        terrain_renderer.draw(
            &mut target, &ring, &camera.projection.to_mat(),
            &camera.get_view_transform(), &Iso3::new(nalgebra::zero(), nalgebra::zero()), &heightmap
        );

        for level in 2..2 {
            terrain_renderer.level = level;
            terrain_renderer.draw(
                &mut target, &ring, &camera.projection.to_mat(),
                &camera.get_view_transform(), &Iso3::new(nalgebra::zero(), nalgebra::zero()), &heightmap
            );
        }

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
                    terrain_renderer.wire = !terrain_renderer.wire;
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

pub struct HeightmapGenerator {
    pub program : glium::Program,
    pub fs_quad : VertexBuffer<Vertex>
}

impl HeightmapGenerator {
    fn new(display: &Display) -> HeightmapGenerator {
        HeightmapGenerator {
            program : HeightmapGenerator::create_shader_program(&display),
            fs_quad : HeightmapGenerator::create_fullscreen_quad(&display)
        }
    }

    fn generate(&self, heightmap: &Texture2d) {
        let mut surface = heightmap.as_surface();

        surface.clear(Some((0.0, 0.0, 0.0, 0.0)), None, None);
        surface.draw(
            &self.fs_quad, &NoIndices(PrimitiveType::TriangleStrip),
            &self.program, &uniform!(), &std::default::Default::default()
        ).unwrap();
    }

    fn create_fullscreen_quad(display: &Display) -> VertexBuffer<Vertex> {
        VertexBuffer::new(display, vec![
            Vertex::from_position(-1.0, -1.0, 0.0),
            Vertex::from_position( 1.0, -1.0, 0.0),
            Vertex::from_position(-1.0,  1.0, 0.0),
            Vertex::from_position( 1.0,  1.0, 0.0)
        ])
    }

    fn create_shader_program(display: &Display) -> glium::Program {
        let vertex_shader_src = r#"
            #version 330

            in vec3 position;
            out vec2 pos;
            void main() {
                pos = position.xy;
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330

            in vec2 pos;
            out vec4 frag_color;
            void main() {
                frag_color = vec4(3*sin(pos.x*3.14159));
            }
        "#;

        Program::from_source(
            display, vertex_shader_src, fragment_shader_src, None
        ).unwrap()
    }
}

pub struct TerrainRenderer {
    pub program : glium::Program,
    pub wire    : bool,
    pub level   : i32
}

impl TerrainRenderer {
    fn new(display: &Display) -> TerrainRenderer {
        TerrainRenderer {
            program : TerrainRenderer::create_shader_program(&display),
            wire    : false,
            level   : 1
        }
    }

    fn draw(
        &self, frame: &mut Frame,
        obj: &RenderableObj, proj: &Mat4<f32>, view: &Iso3<f32>, model: &Iso3<f32>, heightmap: &Texture2d
    ) {
        let v = to_homogeneous(&nalgebra::inv(view).unwrap());
        let mvp = *proj;

        let params = DrawParameters {
            depth_test   : DepthTest::IfLess,
            depth_write  : true,
            polygon_mode : if self.wire == true { PolygonMode::Line } else { PolygonMode::Fill },
            .. ::std::default::Default::default()
        };

        let uniforms = uniform!(
            MVP       : mvp,
            V         : v,
            level     : self.level,
            heightmap : heightmap
        );

        match obj.indices {
            RenderableIndices::None(primitive) => {
                frame.draw(
                    &obj.vertices,
                    &NoIndices(primitive),
                    &self.program, &uniforms,
                    &params
                ).unwrap();
            },
            RenderableIndices::Buffer(ref buffer) => {
                frame.draw(
                    &obj.vertices,
                    buffer,
                    &self.program, &uniforms,
                    &params
                ).unwrap();
            }
        }
    }

    fn create_shader_program(display: &Display) -> glium::Program {
        let vertex_shader_src = r#"
            #version 330

            in vec3 position;
            out float height;

            uniform vec3 camera_offset;
            uniform mat4 MVP;
            uniform mat4 V;
            uniform int level;
            uniform sampler2D heightmap;

            void main() {
                float level_exp = pow(2, level);
                vec3 actual_pos =
                    level_exp * position + vec3(level_exp, 0.0, level_exp);

                vec4 world_pos = V * vec4(actual_pos, 1);

                actual_pos.y = texture(heightmap, vec2(world_pos.xz / 16));
                height = actual_pos.y;
                gl_Position = MVP * vec4(actual_pos, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330
            in float height;
            out vec4 frag_color;
            void main() {
                frag_color = vec4(1.0, 1.0, height/10.0, 1.0);
            }
        "#;

        Program::from_source(
            display, vertex_shader_src, fragment_shader_src, None
        ).unwrap()
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













