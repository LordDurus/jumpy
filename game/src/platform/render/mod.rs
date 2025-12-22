use crate::platform::input;

pub mod backend;
pub mod common;
pub mod render;

pub use render::Renderer;

#[cfg(feature = "gba")]
pub mod gba;

#[cfg(feature = "pc")]
pub mod pc;

#[cfg(feature = "psp")]
pub mod psp;
