use event_loop::{LoopState, HasEventLoop};
use render::Render;
use stage::{StageContainer, Stage};
use glutin::Event;
use glium::*;

pub struct Lab <'a, V, U, T: Stage<V, U>> {
    pub render: &'a Render,
    pub stage: StageContainer<'a, V, U, T>,
    pub timestamp: f64,
}

impl <'a, V, U, T: Stage<V, U>> Lab <'a, V, U, T> {
    pub fn new(render: &'a Render, obj: T) -> Self {
        let stage = StageContainer::new(obj);
        Lab {
            render: &render,
            stage: stage,
            timestamp: 0 as f64,
        }
    }
}

impl <'a, V: vertex::Vertex, U: uniforms::Uniforms + Copy, T: Stage<V, U>> HasEventLoop for Lab<'a, V, U, T> {
    fn render(&mut self, dt: f32) {
        let mut frame = self.render.begin();
        self.render.draw(&mut frame, &self.stage, dt);
        frame.finish();
    }

    #[allow(unused_variables)]
    fn update(&mut self, dt: f32) {
        self.stage.update(dt);
    }

    fn poll(&mut self) -> Option<LoopState> {
        for event in self.render.poll_events() {
            match event {
                Event::Closed => return Some(LoopState::Break),
                _ => return Some(LoopState::Continue),
            }
        }

        None
    }
}
