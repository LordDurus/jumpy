#[cfg(feature = "gba")]
extern crate alloc;

#[cfg(feature = "gba")]
use alloc::string::String;

#[cfg(feature = "pc")]
use std::{fs, path::Path};

use crate::runtime::level::Level;

#[cfg(feature = "pc")]
pub fn load_level_from_file(path: &Path) -> Level {
	let bytes = fs::read(path).expect("failed to read level file");
	return Level::load_binary(&bytes).unwrap();
}

#[cfg(feature = "pc")]
pub fn load_level_from_name(level_name: &str) -> Result<crate::runtime::level::Level, String> {
	use std::path::Path;
	let path = Path::new(level_name);
	let level = crate::platform::level_loader::load_level_from_file(path);
	return Ok(level);
}

#[cfg(feature = "gba")]
pub fn load_level_from_name(_level_name: &str) -> Result<Level, String> {
	return Err(String::from("load_level_from_name not supported on gba"));
}
