use crate::game::render::Renderer;
// use crate::world::World;
use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct PcRenderer {
    canvas: Canvas<Window>,
    sdl_context: Sdl,
}

impl PcRenderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();
        let window: Window = video_subsystem
            .window("PC Renderer", 640, 480)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self {
            canvas,
            sdl_context,
        }
    }
}

impl Renderer for PcRenderer {
    fn init(&mut self) {
        self.canvas.clear();
    }

    fn commit(&mut self) {
        self.canvas.present(); // Directly call the inherent method to present the frame
    }
}
