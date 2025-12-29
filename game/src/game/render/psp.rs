#![cfg(feature = "psp")]

use crate::render::Renderer;
use crate::world::World;
use psp::sys::{
    SCE_DISPLAY_MODE_LCD, SCE_GU_CLEAR_ALL, SCE_GU_SYNC_WAIT, SceDisplaySetMode, sceDisplaySetMode,
    sceGuFinish, sceGuStart, sceGuSync,
};

pub struct PspRenderer;

impl PspRenderer {
    pub fn new() -> Self {
        unsafe {
            sceDisplaySetMode(SCE_DISPLAY_MODE_LCD, 480, 272); // Set display mode
        }
        Self
    }
}

impl Renderer for PspRenderer {
    fn init(&mut self) {
        unsafe {
            sceGuStart(0, std::ptr::null_mut()); // Start GU (Graphics Unit) rendering
        }
    }

    fn render_frame(&mut self, _world: &mut World) {
        unsafe {
            sceGuStart(0, std::ptr::null_mut()); // Begin rendering
            sceGuFinish(); // End rendering
            sceGuSync(SCE_GU_SYNC_WAIT, std::ptr::null_mut()); // Sync rendering
        }
        self.commit();
    }

    fn commit(&mut self) {
        unsafe {
            psp::sys::sceDisplayWaitVblankStart(); // Wait for VBlank to prevent tearing
        }
    }
}
