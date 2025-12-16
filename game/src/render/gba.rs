use crate::render::Renderer;
use crate::world::World;

pub struct GbaRenderer;

impl GbaRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for GbaRenderer {
    fn init(&mut self) {
        // Initialize GBA-specific rendering (e.g., mode, VRAM setup)
    }

    fn render_frame(&mut self, world: &mut World) {
        // Update OAM or VRAM based on the world state
        agb::wait_for_vblank(); // Wait for VBlank to sync rendering
    }
}
