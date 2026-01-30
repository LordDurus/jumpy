use crate::platform::input;

pub mod backend;
pub mod common;
pub mod icon_registry;

#[cfg(feature = "pc")]
pub mod pc;

#[cfg(feature = "gba")]
pub mod gba;

#[cfg(feature = "psp")]
pub mod psp;
