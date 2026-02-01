#[cfg(feature = "pc")]
mod pc;
#[cfg(feature = "pc")]
pub use pc::*;

#[cfg(feature = "gba")]
mod gba;
#[cfg(feature = "gba")]
pub use gba::*;

#[cfg(feature = "psp")]
mod psp;
#[cfg(feature = "psp")]
pub use psp::*;
