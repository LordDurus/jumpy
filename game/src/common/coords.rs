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
pub fn screen_to_world(screen: ScreenPoint, cam: WorldPoint, scale: f32) -> WorldPoint {
	return WorldPoint {
		left: (screen.left as f32) / scale + cam.left,
		top: (screen.top as f32) / scale + cam.top,
	};
}

#[inline(always)]
pub fn tile_to_world(tile: TilePoint, tile_size: WorldSize) -> WorldPoint {
	return WorldPoint {
		left: (tile.left as f32) * tile_size.width,
		top: (tile.top as f32) * tile_size.height,
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
