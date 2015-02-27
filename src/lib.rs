#![feature(plugin)]
#![plugin(glium_macros)]
#![cfg_attr(test, allow(warnings))]
#![feature(old_path)]

extern crate event_loop;
extern crate image;
extern crate cgmath;
extern crate clock_ticks;
extern crate glium;
extern crate glutin;

pub mod render;
pub mod lab;
pub mod stage;

use self::stage::Stage;
use self::render::Render;

pub fn init(w: u32, h: u32) -> render::Render {
    use glutin;
    use glium::DisplayBuild;

    let display = glutin::WindowBuilder::new()
        .with_dimensions(w, h)
        .with_title("Shader Lab".to_string())
        .with_vsync()
        .build_glium()
        .unwrap();

    Render::new(display)
}

pub fn run<V: glium::vertex::Vertex, U: glium::uniforms::Uniforms + Copy, T: Stage<V, U>>(obj: T) {
    use event_loop::EventLoop;
    use lab::Lab;

    let mut lab: Lab<V, U, T> = Lab::new(obj);
    let evt = EventLoop::new(120, 60);
    evt.run(&mut lab);
}
