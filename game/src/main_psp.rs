#![no_std]
#![no_main]

use agb::entry;

#[entry]
fn run(mut gba: agb::Gba) -> ! {
	loop {
		gba.vblank.wait_for_vblank();
	}
}
