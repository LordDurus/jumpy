use std::{
	env, fs,
	path::{Path, PathBuf},
};

fn main() {
	//rerun when build script changes
	println!("cargo:rerun-if-changed=build.rs");

	let sdl2_dll = resolve_sdl2_dll().expect("could not find SDL2.dll (set SDL2_LIB_DIR or LIB)");

	//copy next to the .exe (target\debug or target\release)
	let out_exe_dir = target_exe_dir().expect("could not determine target output folder");
	let dest = out_exe_dir.join("SDL2.dll");

	fs::create_dir_all(&out_exe_dir).unwrap();
	fs::copy(&sdl2_dll, &dest).unwrap();

	println!("cargo:warning=copying SDL2.dll from {} -> {}", sdl2_dll.display(), dest.display());
}

fn resolve_sdl2_dll() -> Option<PathBuf> {
	//preferred: your explicit env var
	if let Ok(dir) = env::var("SDL2_LIB_DIR") {
		let p = PathBuf::from(dir).join("SDL2.dll");
		if p.exists() {
			return Some(p);
		}
	}

	//fallback: scan LIB paths
	let lib = env::var("LIB").ok()?;
	for dir in lib.split(';').filter(|s| !s.is_empty()) {
		let p = PathBuf::from(dir).join("SDL2.dll");
		if p.exists() {
			return Some(p);
		}
	}

	None
}

fn target_exe_dir() -> Option<PathBuf> {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").ok()?);
	let profile = env::var("PROFILE").ok()?; // "debug" or "release"

	//if you use a workspace, this crate’s Cargo.toml might be in /game; adjust if needed
	Some(manifest_dir.join("target").join(profile))
}
