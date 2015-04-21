use ::nalgebra::{Vec3, Mat4, Iso3, to_homogeneous, Transformation, RotationMatrix};
use ::{RenderableObj, RenderableIndices};
use ::glium::{Program, Display, DrawParameters, DepthTest, Frame, Surface, PolygonMode};
use ::glium::index::{NoIndices, PrimitiveType};


pub struct LightingRenderer {
    pub program        : Program,
    pub light_position : Vec3<f32>,
    pub diffuse_color  : Vec3<f32>,
    pub specular_color : Vec3<f32>,
    pub shininess      : f32,
    pub wire           : bool
}

impl LightingRenderer {
    pub fn new(display: &Display) -> LightingRenderer {
        LightingRenderer {
            program        : LightingRenderer::create_shader_program(&display),
            light_position : Vec3::new(0.0, 0.0, 0.0),
            diffuse_color  : Vec3::new(1.0, 1.0, 1.0),
            specular_color : Vec3::new(1.0, 1.0, 1.0),
            shininess      : 128.0,
            wire           : false
        }
    }

    pub fn draw(
        &self, frame: &mut Frame,
        obj: &RenderableObj, proj: &Mat4<f32>, view: &Iso3<f32>, model: &Iso3<f32>
    )  {

        let mv  = view.prepend_transformation(model);
        let mvp = *proj * to_homogeneous(&mv);
        let n   = *mv.to_rot_mat().submat();

        let params = DrawParameters {
            depth_test   : DepthTest::IfLess,
            depth_write  : true,
            polygon_mode : if self.wire == true { PolygonMode::Line } else { PolygonMode::Fill },
            .. ::std::default::Default::default()
        };

        let uniforms = uniform!(
            MVP            : mvp,
            MV             : to_homogeneous(&mv),
            N              : n,
            light_position : self.light_position,
            diffuse_color  : self.diffuse_color,
            specular_color : self.specular_color,
            shininess      : self.shininess
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

    fn create_shader_program(display: &Display) -> Program {
        let vertex_shader_src = r#"
            #version 330
            in vec3 position;
            in vec3 normal;
            smooth out vec3 eye_space_normal;
            smooth out vec3 eye_space_position;

            uniform mat4 MVP;
            uniform mat4 MV;
            uniform mat3 N;

            void main() {
                eye_space_normal = N*normal;
                eye_space_position = (MV*vec4(position, 1)).xyz;
                gl_Position = MVP * vec4(position , 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330

            smooth in vec3 eye_space_normal;
            smooth in vec3 eye_space_position;
            out vec4 vFragColor;

            uniform vec3 light_position;
            uniform vec3 diffuse_color;
            uniform vec3 specular_color;
            uniform mat4 MV;
            uniform float shininess;

            const vec3 eye_space_camera_pos = vec3(0,0,0);

            void main() {
                vec3 eye_space_light_pos = (MV * vec4(light_position, 1)).xyz;
                vec3 norm                = normalize(eye_space_normal);

                vec3 L = normalize(eye_space_light_pos - eye_space_position);
                vec3 V = normalize(eye_space_camera_pos - eye_space_position);
                vec3 H = normalize(L + V);
                float diffuse  = max(0, dot(norm, L));
                float specular = max(0, pow(dot(eye_space_normal, H), shininess));

                vFragColor = specular*vec4(specular_color, 1) + diffuse*vec4(diffuse_color, 1);

            }
        "#;

        Program::from_source(
            display, vertex_shader_src, fragment_shader_src, None
        ).unwrap()
    }
}



pub struct NormalRenderer {
    pub program : Program
}

impl NormalRenderer {
    pub fn new(display: &Display) -> NormalRenderer {
        NormalRenderer {
            program : NormalRenderer::create_normal_renderer_program(&display)
        }
    }

    pub fn draw(
        &self, frame: &mut Frame, obj: &RenderableObj,
        proj: &Mat4<f32>, view: &Iso3<f32>, model: &Iso3<f32>
    ) {
        let mv = view.prepend_transformation(model);
        let mvp = *proj * to_homogeneous(&mv);

        let uniforms = uniform!(
            MVP: mvp
        );

        let params = DrawParameters {
            depth_test: DepthTest::IfLess,
            depth_write: true,
            .. ::std::default::Default::default()
        };

        frame.draw(
            &obj.vertices,
            &NoIndices(PrimitiveType::Points),
            &self.program, &uniforms,
            &params
        ).unwrap();
    }

    fn create_normal_renderer_program(display: &Display) -> Program {
        let vertex_shader_src = r#"
            #version 330
            in vec3 position;
            in vec3 normal;
            out vec3 g_normal;

            void main() {
                g_normal = normal;
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let geometry_shader_src = r#"
            #version 330
            layout(points) in;

            layout(line_strip, max_vertices = 2) out;

            uniform mat4 MVP;

            in vec3 g_normal[];

            void main() {
                vec4 v0 = gl_in[0].gl_Position;
                gl_Position = MVP * v0;
                EmitVertex();

                vec4 v1 = v0 + vec4(g_normal[0] * 0.5, 0);
                gl_Position = MVP * v1;
                EmitVertex();

                EndPrimitive();
            }
        "#;

        let fragment_shader_src = r#"
            #version 330
            out vec4 frag_color;
            void main() {
                frag_color = vec4(0.0, 0.0, 0.7, 1.0);
            }
        "#;

        Program::from_source(
            display, vertex_shader_src, fragment_shader_src, Some(geometry_shader_src)
        ).unwrap()
    }
}

