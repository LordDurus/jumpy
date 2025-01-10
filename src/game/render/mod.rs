pub trait Renderer {
    fn init(&mut self);
    fn commit(&mut self); // Commit the frame to the display
}

#[cfg(feature = "pc")]
pub mod pc;

#[cfg(feature = "gba")]
pub mod gba;

#[cfg(feature = "psp")]
pub mod psp;
