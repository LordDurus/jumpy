use std::{
	env, fs,
	path::{Path, PathBuf},
};

fn copy_if_exists(src: &Path, dst: &Path) {
	if src.exists() {
		let _ = fs::copy(src, dst);
	}
}

fn main() {
	println!("cargo:rerun-if-changed=build.rs");

	if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
		return;
	}

	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let profile = env::var("PROFILE").unwrap(); // debug / release

	// .../target/{debug|release}/build/<crate>/out -> .../target/{debug|release}
	let target_dir = out_dir.ancestors().find(|p| p.ends_with(&profile)).expect("failed to locate target dir");

	let vcpkg_root = match env::var("VCPKG_ROOT") {
		Ok(v) => PathBuf::from(v),
		Err(_) => return,
	};

	let bin_dir = vcpkg_root.join("installed").join("x64-windows").join("bin");

	// required runtime dlls
	copy_if_exists(&bin_dir.join("SDL2.dll"), &target_dir.join("SDL2.dll"));
	copy_if_exists(&bin_dir.join("SDL2_image.dll"), &target_dir.join("SDL2_image.dll"));

	// common deps pulled by SDL2_image (copy if present)
	copy_if_exists(&bin_dir.join("libpng16.dll"), &target_dir.join("libpng16.dll"));
	copy_if_exists(&bin_dir.join("zlib1.dll"), &target_dir.join("zlib1.dll"));
}
