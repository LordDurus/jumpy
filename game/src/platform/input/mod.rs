pub trait Input {}

#[cfg(feature = "gba")]
pub mod gba;

#[cfg(feature = "pc")]
pub mod pc;
