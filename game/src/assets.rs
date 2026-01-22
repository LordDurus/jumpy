use std::path::{Path, PathBuf};

pub fn get_asset_root() -> PathBuf {
	let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("assets");
	return root;
}

pub fn get_gfx_root() -> PathBuf {
	let root: PathBuf = get_asset_root().join("gfx");
	return root;
}

pub fn get_audio_root() -> PathBuf {
	let root: PathBuf = get_asset_root().join("audio");
	return root;
}

pub fn get_messages_root() -> PathBuf {
	let root: PathBuf = get_asset_root().join("messages");
	return root;
}
