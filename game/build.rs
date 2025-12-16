use std::{env, fs, path::PathBuf};

fn main() {
	if env::var_os("CARGO_FEATURE_PC").is_none() {
		return;
	}

	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let profile = env::var("PROFILE").unwrap(); // debug/release

	let target_dir = env::var("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|_| manifest_dir.join("target"));

	let target = env::var("TARGET").unwrap();
	let exe_dir = if target == env::var("HOST").unwrap() {
		target_dir.join(&profile)
	} else {
		target_dir.join(&target).join(&profile)
	};

	let sdl2_dll = env::var("SDL2_DLL")
		.map(PathBuf::from)
		.unwrap_or_else(|_| PathBuf::from(r"C:\libs\SDL2-2.30.11\lib\x64\SDL2.dll"));

	println!("cargo:warning=copying SDL2.dll from {}", sdl2_dll.display());
	println!("cargo:warning=to {}", exe_dir.display());

	fs::create_dir_all(&exe_dir).unwrap();
	fs::copy(&sdl2_dll, exe_dir.join("SDL2.dll")).expect("failed to copy SDL2.dll");
}
