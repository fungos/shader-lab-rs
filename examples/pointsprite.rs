#![feature(plugin)]
#![feature(path)]
#![plugin(glium_macros)]

extern crate cgmath;
extern crate glium;
extern crate "shader-lab-rs" as shader;

use cgmath::*;
use glium::*;

use shader::stage::Stage;
use shader::render::Render;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 600;

static VERTEX_SHADER: &'static str = "
        #version 150

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        in vec2 pos;
        in vec2 uv;
        in vec4 color;

        out Vertex {
            vec2 uv;
            vec4 color;
        } vertex;

        void main() {
            mat4 mv = model * view;
            vec4 p = mv * vec4(pos, 0.0, 1.0);

            gl_Position = projection * p;
            vertex.uv = uv;
            vertex.color = color;
        }
    ";

static GEOMETRY_SHADER: &'static str = "
        #version 150

        layout (points) in;
        layout (triangle_strip, max_vertices = 4) out;

        uniform mat4 projection;
        uniform vec4 width_height_offset;

        in Vertex {
            vec2 uv;
            vec4 color;
        } vertex[];

        out vec2 coords;
        out vec4 color;

        void main() {
            vec4 p = gl_in[0].gl_Position;
            float hw = width_height_offset.x;
            float hh = width_height_offset.y;
            vec4 offset = vec4(width_height_offset.z, width_height_offset.w, 0.0, 1.0);
            vec2 uv = vertex[0].uv;

            // vertex a - left bottom (-1, -1)
            vec2 va = p.xy + vec2(-hw, -hh);
            gl_Position = projection * (offset + vec4(va, p.zw));
            coords = uv;
            color = vertex[0].color;
            EmitVertex();

            // vertex b - left top (-1, 1)
            vec2 vb = p.xy + vec2(-hw, hh);
            gl_Position = projection * (offset + vec4(vb, p.zw));
            coords = uv + vec2(0.0, 1.0);
            color = vertex[0].color;
            EmitVertex();

            // vertex d - right bottom (1, -1)
            vec2 vd = p.xy + vec2(hw, -hh);
            gl_Position = projection * (offset + vec4(vd, p.zw));
            coords = uv + vec2(1.0, 0.0);
            color = vertex[0].color;
            EmitVertex();

            // vertex c - right top (1, 1)
            vec2 vc = p.xy + vec2(hw, hh);
            gl_Position = projection * (offset + vec4(vc, p.zw));
            coords = uv + vec2(1.0, 1.0);
            color = vertex[0].color;
            EmitVertex();

            EndPrimitive();
        }
    ";


static FRAGMENT_SHADER: &'static str = "
        #version 130

        uniform sampler2D texture;

        in vec2 coords;
        in vec4 color;

        void main() {
            gl_FragColor = texture2D(texture, coords);// * color;
        }
    ";


#[vertex_format]
#[derive(Copy)]
pub struct VertexData {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

#[uniforms]
pub struct Uniforms<'a> {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    width_height_offset: [f32; 4],
    texture: &'a Texture2d,
}

pub struct Scene<'a> {
    render: &'a Render,
    model_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    proj_matrix: Matrix4<f32>,
    width_height_offset: Vector4<f32>,
    texture: &'a Texture2d,
}

impl<'a> Scene<'a> {
    pub fn new(render: &'a Render, tex: &'a Texture2d) -> Scene<'a> {
        let w = tex.get_width();
        let h = tex.get_height().unwrap();

        let mtx = Matrix4::<f32>::new(
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 1.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
            );

        let inv_w = w as f32 / WIDTH as f32;
        let inv_h = h as f32 / HEIGHT as f32;

        let right = Vector4::new(inv_w, 0.0, 0.0, 0.0);
        let up = Vector4::new(0.0, inv_h, 0.0, 0.0);
        let offset = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let width_height_offset = right.add_v(&up).add_v(&offset);

        Scene {
            render: &render,
            model_matrix: mtx,
            view_matrix: mtx,
            proj_matrix: mtx,
            width_height_offset: width_height_offset,
            texture: &tex,
        }
    }
}

impl<'a> Stage<VertexData, Uniforms<'a>> for Scene<'a> {
    #[allow(unused_variables)]
    fn update(&mut self, dt: f32) {
    }

    fn get_vertex_buffer(&self) -> VertexBuffer<VertexData> {
        VertexBuffer::new(&self.render.context, vec![
            VertexData { pos: [0.0, 0.0], uv: [0.5, 0.5], color: [1.0, 1.0, 1.0, 1.0] },
        ])
    }

    fn get_index_buffer(&self) -> IndexBuffer {
        IndexBuffer::new(&self.render.context, index::PointsList(vec![1 as u16]))
    }

    fn get_program(&self) -> Program {
        Program::from_source(&self.render.context,
            VERTEX_SHADER, FRAGMENT_SHADER, Some(GEOMETRY_SHADER)
        ).unwrap()
    }

    fn get_uniforms(&self) -> Uniforms<'a> {
        Uniforms {
            model: *self.model_matrix.as_fixed(),
            view: *self.view_matrix.as_fixed(),
            projection: *self.proj_matrix.as_fixed(),
            width_height_offset: *self.width_height_offset.as_fixed(),
            texture: self.texture,
        }
    }

    fn get_draw_params(&self) -> DrawParameters {
        DrawParameters {
            blending_function: Some(BlendingFunction::Addition {
                source: LinearBlendingFactor::SourceAlpha,
                destination: LinearBlendingFactor::OneMinusSourceAlpha,
            }),
            .. std::default::Default::default()
        }
    }
}

fn main() {
    let render = shader::init(WIDTH, HEIGHT);
    let tex = Render::load_texture(&render, &Path::new("./examples/data/image.png"));
    let scene = Scene::new(&render, &tex);
    shader::run::<VertexData, Uniforms, Scene>(scene, &render);
}
