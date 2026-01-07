#[derive(Clone, Copy, Debug)]
pub struct TileRect {
	pub start_left: i32,
	pub start_top: i32,
	pub end_left: i32,
	pub end_top: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldPoint {
	pub left: f32,
	pub top: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct ScreenPoint {
	pub left: i32,
	pub top: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct TilePoint {
	pub left: i32,
	pub top: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldSize {
	pub width: f32,
	pub height: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct PixelSize {
	pub width: i32,
	pub height: i32,
}

#[inline(always)]
pub fn world_to_screen(world: WorldPoint, cam: WorldPoint, scale: f32) -> ScreenPoint {
	return ScreenPoint {
		left: ((world.left - cam.left) * scale).round() as i32,
		top: ((world.top - cam.top) * scale).round() as i32,
	};
}

#[inline(always)]
pub fn world_to_tile(world: WorldPoint, tile_size: WorldSize) -> TilePoint {
	return TilePoint {
		left: (world.left / tile_size.width).floor() as i32,
		top: (world.top / tile_size.height).floor() as i32,
	};
}

#[inline(always)]
pub fn view_world_size(view_pixels: PixelSize, scale: f32) -> WorldSize {
	return WorldSize {
		width: (view_pixels.width as f32) / scale,
		height: (view_pixels.height as f32) / scale,
	};
}

#[inline(always)]
pub fn visible_tile_bounds(cam: WorldPoint, view_pixels: PixelSize, scale: f32, tile_size: WorldSize, level_width_tiles: i32, level_height_tiles: i32) -> TileRect {
	let view_world = view_world_size(view_pixels, scale);

	let start = world_to_tile(cam, tile_size);

	let end_world = WorldPoint {
		left: cam.left + view_world.width,
		top: cam.top + view_world.height,
	};
	let end = world_to_tile(end_world, tile_size);

	return TileRect {
		start_left: (start.left - 1).max(0),
		start_top: (start.top - 1).max(0),
		end_left: (end.left + 2).min(level_width_tiles),
		end_top: (end.top + 2).min(level_height_tiles),
	};
}
