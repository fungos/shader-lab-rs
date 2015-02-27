use glium::*;
use render::Render;
use std::marker;

pub trait Stage<T, U> {
    fn update(&mut self, dt: f32);
    fn render(&mut self, dt: f32);
    fn get_vertex_buffer(&self) -> VertexBuffer<T>;
    fn get_index_buffer(&self) -> IndexBuffer;
    fn get_program(&self) -> Program;
    fn get_uniforms(&self) -> U;
    fn get_draw_params(&self) -> DrawParameters;
    fn get_render(&self) -> &Render;
}

pub struct StageContainer<V, U, T: Stage<V, U>> {
    pub obj: T,
    marker1: marker::PhantomData<U>,
    marker2: marker::PhantomData<V>,
}

impl <V: vertex::Vertex, U: uniforms::Uniforms + Copy, T: Stage<V, U>> StageContainer <V, U, T> {
    pub fn new(obj: T) -> StageContainer<V, U, T> {
        StageContainer {
            obj: obj,
            marker1: marker::PhantomData,
            marker2: marker::PhantomData,
        }
    }

    #[allow(unused_variables)]
    pub fn update(&mut self, dt: f32) {
        self.obj.update(dt);
    }

    pub fn draw(&mut self, dt: f32) {
        let obj = &mut self.obj;
        let mut frame = obj.get_render().begin();

        obj.render(dt);
        frame.draw(
            &obj.get_vertex_buffer(),
            &obj.get_index_buffer(),
            &obj.get_program(),
            &obj.get_uniforms(),
            &obj.get_draw_params()
        ).unwrap();

        frame.finish();
    }

    pub fn poll_events(&self) -> PollEventsIter {
       self.obj.get_render().poll_events()
    }
}
