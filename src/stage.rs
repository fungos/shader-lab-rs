use glium::*;
use render::Draw;

pub trait Stage<T, U> {
    fn update(&mut self, dt: f32);
    fn render(&mut self, dt: f32);
    fn get_vertex_buffer(&self) -> VertexBuffer<T>;
    fn get_index_buffer(&self) -> IndexBuffer;
    fn get_program(&self) -> Program;
    fn get_uniforms(&self) -> U;
    fn get_draw_params(&self) -> DrawParameters;
}

pub struct StageContainer<'a, V, U, T: Stage<V, U>> {
    pub obj: T,
}

impl <'a, V, U, T: Stage<V, U>> StageContainer <'a, V, U, T> {
    pub fn new(obj: T) -> StageContainer<'a, V, U, T> {
        StageContainer {
            obj: obj,
        }
    }

    #[allow(unused_variables)]
    pub fn update(&mut self, dt: f32) {
    // Here T must be mutable so we can do:
    // self.obj.update(dt);
    }
}

impl <'a, V: vertex::Vertex, U: uniforms::Uniforms + Copy, T: Stage<V, U>> Draw for StageContainer<'a, V, U, T> {
    #[inline]
    #[allow(unused_variables)]
    fn draw(&mut self, frame: &mut Frame, dt: f32) {
        let mut obj = &self.obj;
        obj.render(dt);
        frame.draw(
            &obj.get_vertex_buffer(),
            &obj.get_index_buffer(),
            &obj.get_program(),
            &obj.get_uniforms(),
            &obj.get_draw_params()
        ).unwrap();
    }
}
