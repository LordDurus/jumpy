use std::{fs, io::Write, path::PathBuf};

#[derive(Clone, Copy)]
pub struct WindowSettings {
	pub left: i32,
	pub top: i32,
	pub width_pixels: u32,
	pub height_pixels: u32,
	pub is_maximized: bool,
}

fn window_settings_path() -> PathBuf {
	let mut base: PathBuf = if cfg!(target_os = "windows") {
		std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."))
	} else if cfg!(target_os = "macos") {
		std::env::var_os("HOME")
			.map(|h| PathBuf::from(h).join("Library").join("Application Support"))
			.unwrap_or_else(|| PathBuf::from("."))
	} else {
		if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
			PathBuf::from(xdg)
		} else {
			std::env::var_os("HOME")
				.map(|h| PathBuf::from(h).join(".config"))
				.unwrap_or_else(|| PathBuf::from("."))
		}
	};

	base = base.join("jumpy");
	let _ = fs::create_dir_all(&base);
	return base.join("window.txt");
}

pub fn load_window_settings() -> Option<WindowSettings> {
	let path = window_settings_path();
	let text = fs::read_to_string(path).ok()?;
	let parts: Vec<&str> = text.split_whitespace().collect();
	if parts.len() < 5 {
		return None;
	}

	let left: i32 = parts[0].parse().ok()?;
	let top: i32 = parts[1].parse().ok()?;
	let width_pixels: u32 = parts[2].parse().ok()?;
	let height_pixels: u32 = parts[3].parse().ok()?;
	let is_maximized: bool = parts[4].parse().ok()?;

	if width_pixels < 320 || height_pixels < 180 {
		return None;
	}

	return Some(WindowSettings {
		left,
		top,
		width_pixels,
		height_pixels,
		is_maximized,
	});
}

pub fn save_window_settings(window: &sdl2::video::Window) {
	let (left, top) = window.position();
	let (width_pixels, height_pixels) = window.size();
	let is_maximized: bool = (window.window_flags() & sdl2::sys::SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32) != 0;

	let path = window_settings_path();
	let mut file = match fs::File::create(path) {
		Ok(f) => f,
		Err(_) => return,
	};

	let _ = writeln!(file, "{} {} {} {} {}", left, top, width_pixels, height_pixels, is_maximized);
	return;
}
