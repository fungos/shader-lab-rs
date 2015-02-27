use glium::*;
use image;

pub struct Render {
    pub context: Display,
}

impl Render {
    pub fn new(context: Display) -> Render {
        Render {
            context: context,
        }
    }

    #[inline]
    pub fn load_texture(&self, path: &Path) -> Texture2d {
        Texture2d::new(&self.context, image::open(path).unwrap())
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
