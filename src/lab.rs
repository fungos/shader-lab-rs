use event_loop::{LoopState, HasEventLoop};
use stage::{StageContainer, Stage};
use glutin::Event;
use glium::*;

pub struct Lab <V, U, T: Stage<V, U>> {
    pub stage: StageContainer<V, U, T>,
    pub timestamp: f64,
}

impl <V: vertex::Vertex, U: uniforms::Uniforms + Copy, T: Stage<V, U>> Lab <V, U, T> {
    pub fn new(obj: T) -> Self {
        let stage = StageContainer::new(obj);
        Lab {
            stage: stage,
            timestamp: 0 as f64,
        }
    }
}

impl <V: vertex::Vertex, U: uniforms::Uniforms + Copy, T: Stage<V, U>> HasEventLoop for Lab<V, U, T> {
    fn render(&mut self, dt: f32) {
        self.stage.draw(dt);
        self.timestamp += dt as f64;
    }

    #[allow(unused_variables)]
    fn update(&mut self, dt: f32) {
        self.stage.update(dt);
    }

    fn poll(&mut self) -> Option<LoopState> {
        for event in self.stage.poll_events() {
            //println!("{:?}", event);
            match event {
                Event::Closed => return Some(LoopState::Break),
                Event::MouseMoved((x, y)) => {
                    //println!("Mouse moved {},{}!", x, y);
                    return Some(LoopState::Continue)
                },
                _ => return Some(LoopState::Continue),
            }
        }

        None
    }
}
