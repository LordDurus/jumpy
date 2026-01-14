// use crate::{game::level::Level, platform::render::pc::PcRenderer, tile::TileKind};
// use sdl2::render::Texture;

// use super::PcRenderer;
// use crate::{game::level::Level, platform::render::pc::PcRenderer, tile::TileKind};
// use sdl2::render::Texture;

use super::PcRenderer;
use crate::{game::level::Level, tile::TileKind};

impl PcRenderer {
	pub fn draw_platform_entity_tiles(
		&mut self,
		tile_cols: u32,
		tile_pixel: u32,
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
		// let tile_height_world: f32 = level.tile_height as f32;

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

			let dest_width_pixels: u32 = (tile_pixel as f32 * scale).round() as u32;
			let dest_height_pixels: u32 = (tile_pixel as f32 * scale).round() as u32;

			let src = sdl2::rect::Rect::new(source_left, source_top, tile_pixel, tile_pixel);
			let dst = sdl2::rect::Rect::new(screen_left, screen_top, dest_width_pixels, dest_height_pixels);

			let _ = self.canvas.copy(texture, src, dst);
		}
	}
}
