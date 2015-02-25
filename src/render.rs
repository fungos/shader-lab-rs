use glium::*;
use image;

pub struct Render {
    pub context: Display,
}

pub trait Draw {
    fn draw(&self, frame: &mut Frame, dt: f32);
}

impl Render {
    pub fn new(context: Display) -> Render {
        Render {
            context: context,
        }
    }

    #[inline]
    pub fn draw<T: Draw>(&self, frame: &mut Frame, obj: &T, dt: f32) {
        obj.draw(frame, dt);
    }

    #[inline]
    pub fn load_texture(render: &Render, path: &Path) -> Texture2d {
        Texture2d::new(&render.context, image::open(path).unwrap())
    }

    #[inline]
    pub fn begin(&self) -> Frame {
        let mut frame = self.context.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);

        frame
    }

    #[inline]
    pub fn poll_events(&self) -> PollEventsIter {
        self.context.poll_events()
    }
}
