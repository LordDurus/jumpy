#![no_std]
#![no_main]

use agb::entry;

#[entry]
fn main(mut gba: agb::Gba) -> ! {
	loop {
		gba.vblank.wait_for_vblank();
	}
}
