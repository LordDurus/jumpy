use crate::render::Renderer;
use crate::world::World;
use gba::Color;
use gba::mmio::*;

pub struct GbaRenderer;

impl GbaRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for GbaRenderer {
    fn init(&mut self) {
        // Set display mode and enable sprites
        DISPCNT.write(DISPCNT::MODE::Mode0 + DISPCNT::OBJ_ON);
    }

    fn commit(&mut self) {
        gba::wait_for_vblank(); // Wait for VBlank to sync rendering
    }
}
