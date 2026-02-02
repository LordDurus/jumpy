#![cfg_attr(feature = "gba", no_std)]
#![cfg_attr(feature = "gba", no_main)]

#[cfg(feature = "gba")]
extern crate alloc;

#[cfg(feature = "gba")]
mod main_gba;

#[cfg(feature = "pc")]
mod main_pc;

#[cfg(feature = "psp")]
mod main_psp;

#[cfg(feature = "gba")]
use agb::entry;

#[cfg(feature = "gba")]
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
	return main_gba::run(gba);
}

#[cfg(feature = "pc")]
fn main() {
	return main_pc::run();
}

#[cfg(feature = "psp")]
fn main() {
	return main_psp::run();
}
