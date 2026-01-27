use super::PcRenderer;
use crate::{game::level::Level, platform::render::pc::PathBuf, tile::TileKind};

impl PcRenderer {
	pub fn draw_platform_entity_tiles(
		&mut self,
		tile_cols: u32,
		tile_pixel: u32, // atlas tile size in pixels (16 or 64)
		world_left: f32,
		world_top: f32,
		width_tiles: i32,
		level: &Level,
		camera_left: f32,
		camera_top: f32,
		scale: f32,
		left_kind: TileKind,
		mid_kind: TileKind,
		right_kind: TileKind,
	) {
		let texture = self.tile_texture.as_mut().expect("tile_texture is not set");
		if width_tiles <= 0 {
			return;
		}

		let tile_width_world: f32 = level.tile_width as f32;
		let tile_height_world: f32 = level.tile_height as f32;

		let dest_width_pixels: u32 = (tile_width_world * scale).round() as u32;
		let dest_height_pixels: u32 = (tile_height_world * scale).round() as u32;

		for i in 0..width_tiles {
			let tile_kind: TileKind = if width_tiles == 1 {
				mid_kind
			} else if i == 0 {
				left_kind
			} else if i == width_tiles - 1 {
				right_kind
			} else {
				mid_kind
			};

			let tile_id: u32 = tile_kind as u32;

			let source_left: i32 = ((tile_id % tile_cols) * tile_pixel) as i32;
			let source_top: i32 = ((tile_id / tile_cols) * tile_pixel) as i32;

			let seg_world_left: f32 = world_left + (i as f32) * tile_width_world;
			let seg_world_top: f32 = world_top;

			let screen_left: i32 = ((seg_world_left - camera_left) * scale).round() as i32;
			let screen_top: i32 = ((seg_world_top - camera_top) * scale).round() as i32;

			let src = sdl2::rect::Rect::new(source_left, source_top, tile_pixel, tile_pixel);
			let dst = sdl2::rect::Rect::new(screen_left, screen_top, dest_width_pixels, dest_height_pixels);

			let _ = self.canvas.copy(texture, src, dst);
		}
	}
}

use std::{fs, io::Write};

#[derive(Clone, Copy)]
pub struct WindowSettings {
	pub left: i32,
	pub top: i32,
	pub width_pixels: u32,
	pub height_pixels: u32,
	pub is_maximized: bool,
}

fn window_settings_path() -> PathBuf {
	// windows: %APPDATA%\jumpy\window.txt
	// linux:   $XDG_CONFIG_HOME/jumpy/window.txt or ~/.config/jumpy/window.txt
	// mac:     ~/Library/Application Support/jumpy/window.txt (best effort)
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
}
