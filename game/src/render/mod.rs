// render/mod.rs
pub trait Renderer {
    fn init(&mut self);
    fn render_frame(&mut self, world: &mut crate::world::World);
}

#[cfg(feature = "gba")]
pub mod gba;

#[cfg(feature = "pc")]
pub mod pc;
