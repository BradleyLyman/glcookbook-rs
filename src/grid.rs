use ::{
    RenderableVertex, RenderableIndices, BaseVertex
};
use ::glium::{
    IndexBuffer, VertexBuffer, Display
};
use ::glium::index::{TrianglesList};

pub struct Grid{
    pub indices : Vec<u16>,
    vertices    : Vec<[f32; 3]>
}

impl Grid {
    pub fn new(depth: f32, width: f32, x_count: u16, z_count: u16) -> Grid {
        let mut grid = Grid { vertices : vec![], indices  : vec![] };

        grid.build_vertices(depth, width, x_count, z_count);
        grid.build_indices(x_count, z_count);
        grid
    }

    pub fn get_vertices<T: BaseVertex>(&self) -> Vec<T> {
        let mut verts = vec![];

        for pos in &self.vertices {
            verts.push(T::from_position(pos[0], pos[1], pos[2]));
        }
        verts
    }

    fn build_vertices(
        &mut self, depth: f32, width: f32, x_count: u16, z_count: u16
    ) {
        for j in 0..z_count {
            let scaled_j = ((j as f32)/(z_count as f32 -1.0)) * 2.0 - 1.0;
            for i in 0..x_count {
                let scaled_i = ((i as f32)/(x_count as f32 - 1.0)) * 2.0 - 1.0;

                self.vertices.push([scaled_i * width, 0.0, scaled_j * depth]);
            }
        }
    }

    fn build_indices(&mut self, x_count: u16, z_count: u16) {
        let mut count = 0;
        for row in 0..z_count-1 {
            for col in 0..x_count-1 {
                let tl = row * x_count + col;
                let bl = tl + 1;
                let tr = tl + x_count;
                let br = tr + 1;

                if count % 2 == 0 {
                    self.indices.push(tl);
                    self.indices.push(bl);
                    self.indices.push(br);

                    self.indices.push(tl);
                    self.indices.push(br);
                    self.indices.push(tr);
                }
                else {
                    self.indices.push(tl);
                    self.indices.push(bl);
                    self.indices.push(tr);

                    self.indices.push(tr);
                    self.indices.push(bl);
                    self.indices.push(br);
                }
                count += 1;
            }
        }
    }
}

impl ::Renderable for Grid {
    fn get_vertex_array<T: RenderableVertex>(&self, display: &Display)
        -> VertexBuffer<T> {
        let mut vertices = self.get_vertices::<T>();

        for vertex in &mut vertices {
            vertex.set_normal(0.0, 1.0, 0.0);
        }

        VertexBuffer::new(display, vertices)
    }

    fn get_indices(&self, display: &Display) -> RenderableIndices {
        RenderableIndices::Buffer(
            IndexBuffer::new(
                display, TrianglesList(self.indices.clone())
            )
        )
    }
}












