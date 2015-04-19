use ::{
    NormalVertex, Renderable, RenderableVertex, RenderableIndices
};
use ::nalgebra::{Vec3, Norm};
use ::glium::{VertexBuffer, Display};
use ::glium::index::{PrimitiveType};
use ::num::Float;

struct Face {
    pub v1 : Vec3<f32>,
    pub v2 : Vec3<f32>,
    pub v3 : Vec3<f32>
}

impl Face {
    fn from_vec3(v1: Vec3<f32>, v2: Vec3<f32>, v3: Vec3<f32>) -> Face {
        Face {
            v1: v1,
            v2: v2,
            v3: v3
        }
    }
}

pub struct IsoSphere {
    faces        : Vec<Face>
}

impl IsoSphere {
    pub fn new(subdivide_count: u8) -> IsoSphere {
        let mut sphere = IsoSphere { faces : vec![] };

        sphere.generate_icosahedron();
        for _ in 0..subdivide_count {
            sphere.subdivide_faces();
        }
        sphere
    }

    pub fn faces_to_vertex_array<T: NormalVertex>(&self) -> Vec<T> {
        let mut vertices = vec![];
        for face in &self.faces {
            vertices.push(IsoSphere::vertex_from_vec(face.v1));
            vertices.push(IsoSphere::vertex_from_vec(face.v2));
            vertices.push(IsoSphere::vertex_from_vec(face.v3));
        }
        vertices
    }

    fn vertex_from_vec<T: NormalVertex>(vec: Vec3<f32>) -> T {
        let mut vert = T::from_position(vec.x, vec.y, vec.z);
        vert.set_normal(vec.x, vec.y, vec.z);
        vert
    }

    fn subdivide_faces(&mut self) {
        let mut new_faces = vec![];

        for face in &self.faces {
            let Face { v1, v2, v3 } = *face;
            let a = ((v1 + v2) * 1.0/2.0).normalize();
            let b = ((v2 + v3) * 1.0/2.0).normalize();
            let c = ((v1 + v3) * 1.0/2.0).normalize();

            new_faces.push(Face::from_vec3(a, b, c));
            new_faces.push(Face::from_vec3(v1, a, c));
            new_faces.push(Face::from_vec3(a, b, v2));
            new_faces.push(Face::from_vec3(c, b, v3));
        }

        self.faces = new_faces;
    }

    fn generate_icosahedron(&mut self) {
        let t = (1.0 + 5.0.sqrt())/2.0;
        let p0  = Vec3::new( 0.0,  t,  1.0).normalize();
        let p1  = Vec3::new( 0.0,  t, -1.0).normalize();
        let p2  = Vec3::new( 0.0, -t,  1.0).normalize();
        let p3  = Vec3::new( 0.0, -t, -1.0).normalize();

        let p4  = Vec3::new(-1.0, 0.0,  -t).normalize();
        let p5  = Vec3::new( 1.0, 0.0,  -t).normalize();
        let p6  = Vec3::new( 1.0, 0.0,   t).normalize();
        let p7  = Vec3::new(-1.0, 0.0,   t).normalize();

        let p8  = Vec3::new(-t,  1.0,  0.0).normalize();
        let p9  = Vec3::new(-t, -1.0,  0.0).normalize();
        let p10 = Vec3::new( t,  1.0,  0.0).normalize();
        let p11 = Vec3::new( t, -1.0,  0.0).normalize();

        self.faces = vec![
            Face::from_vec3(p1, p10, p5),
            Face::from_vec3(p1, p5, p4),
            Face::from_vec3(p1, p8, p4),
            Face::from_vec3(p1, p8, p0),
            Face::from_vec3(p1, p0, p10),

            Face::from_vec3(p7, p0, p6),
            Face::from_vec3(p7, p6, p2),
            Face::from_vec3(p7, p2, p9),
            Face::from_vec3(p7, p9, p8),
            Face::from_vec3(p7, p8, p0),

            Face::from_vec3(p11, p10, p5),
            Face::from_vec3(p11, p5, p3),
            Face::from_vec3(p11, p3, p2),
            Face::from_vec3(p11, p2, p6),
            Face::from_vec3(p11, p6, p10),

            Face::from_vec3(p0, p6, p10),
            Face::from_vec3(p8, p4, p9),
            Face::from_vec3(p9, p4, p3),
            Face::from_vec3(p9, p3, p2),
            Face::from_vec3(p4, p3, p5)
        ];
    }
}

impl Renderable for IsoSphere {
    fn get_vertex_array<T: RenderableVertex>(
        &self, display: &Display
    ) -> VertexBuffer<T> {
        VertexBuffer::new(display, self.faces_to_vertex_array::<T>())
    }

    fn get_indices(&self, _: &Display) ->  RenderableIndices {
        RenderableIndices::None(PrimitiveType::TrianglesList)
    }
}

