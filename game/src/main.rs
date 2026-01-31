#[cfg(feature = "pc")]
mod main_pc;

#[cfg(feature = "gba")]
mod main_gba;

#[cfg(feature = "pc")]
fn main() {
	main_pc::run();
}

#[cfg(feature = "gba")]
fn main() {
	main_gba::run();
}

#[cfg(not(any(feature = "pc", feature = "gba")))]
fn main() {
	compile_error!("enable exactly one of: pc, gba");
}
