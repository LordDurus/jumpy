#[cfg(feature = "gba")]
extern crate alloc;

#[cfg(feature = "gba")]
use alloc::string::String;

#[cfg(feature = "pc")]
use std::{fs, path::Path};

use crate::runtime::level::Level;

#[cfg(feature = "gba")]
struct EmbeddedLevel {
	world_id: u8,
	level_id: u8,
	bytes: &'static [u8],
}

#[cfg(feature = "gba")]
const EMBEDDED_LEVELS: &[EmbeddedLevel] = &[
	EmbeddedLevel {
		world_id: 0,
		level_id: 1,
		bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../worlds/00/01.lvlb")),
	},
	EmbeddedLevel {
		world_id: 0,
		level_id: 2,
		bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../worlds/00/02.lvlb")),
	},
	EmbeddedLevel {
		world_id: 1,
		level_id: 1,
		bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../worlds/01/01.lvlb")),
	},
	EmbeddedLevel {
		world_id: 2,
		level_id: 1,
		bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../worlds/02/01.lvlb")),
	},
];

// expects something like ".../worlds/01/01.lvlb" or "worlds/01/01.lvlb"
#[cfg(feature = "gba")]
fn parse_world_level_from_path(level_name: &str) -> Option<(u8, u8)> {
	let bytes = level_name.as_bytes();

	// find "worlds/"
	let mut i: usize = 0;
	while i + 7 <= bytes.len() {
		if &bytes[i..i + 7] == b"worlds/" {
			// worlds/WW/LL.lvlb
			let ww0 = i + 7;
			let ww1 = ww0 + 1;
			let slash1 = ww1 + 1;
			let ll0 = slash1 + 1;
			let ll1 = ll0 + 1;

			if ll1 < bytes.len()
				&& bytes[ww0].is_ascii_digit()
				&& bytes[ww1].is_ascii_digit()
				&& bytes[slash1] == b'/'
				&& bytes[ll0].is_ascii_digit()
				&& bytes[ll1].is_ascii_digit()
			{
				let world_id = (bytes[ww0] - b'0') * 10 + (bytes[ww1] - b'0');
				let level_id = (bytes[ll0] - b'0') * 10 + (bytes[ll1] - b'0');
				return Some((world_id, level_id));
			}
		}
		i += 1;
	}

	return None;
}

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
pub fn load_level_from_name(level_name: &str) -> Result<Level, String> {
	let (world_id, level_id) = parse_world_level_from_path(level_name).ok_or_else(|| String::from("level path must contain worlds/WW/LL.lvlb"))?;
	let bytes = find_embedded_level(world_id, level_id).ok_or_else(|| String::from("level not embedded in ROM"))?;
	let level = Level::load_binary(bytes).map_err(|_| String::from("failed to parse lvlb"))?;
	return Ok(level);
}

#[cfg(feature = "gba")]
fn find_embedded_level(world_id: u8, level_id: u8) -> Option<&'static [u8]> {
	for e in EMBEDDED_LEVELS {
		if e.world_id == world_id && e.level_id == level_id {
			return Some(e.bytes);
		}
	}
	return None;
}
