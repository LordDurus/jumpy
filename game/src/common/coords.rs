#[derive(Clone, Copy, Debug)]
pub struct TileRect {
	pub start_left: i32,
	pub start_top: i32,
	pub end_left: i32,
	pub end_top: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Pointf32 {
	pub left: f32,
	pub top: f32,
}

impl Pointf32 {
	pub fn new(left: f32, top: f32) -> Self {
		Self { left, top }
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Pointi32 {
	pub left: i32,
	pub top: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Size {
	pub width: f32,
	pub height: f32,
}

impl Size {
	pub fn new(width: f32, height: f32) -> Self {
		Self { width, height }
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PixelSize {
	pub width: i32,
	pub height: i32,
}
impl PixelSize {
	pub fn new(width: i32, height: i32) -> Self {
		Self { width, height }
	}
}

#[inline(always)]
pub fn get_screen(pointf32: Pointf32, cam: Pointf32, scale: f32) -> Pointi32 {
	return Pointi32 {
		left: ((pointf32.left - cam.left) * scale) as i32,
		top: ((pointf32.top - cam.top) * scale) as i32,
	};
}

#[inline(always)]
pub fn get_tile(pointf32: Pointf32, tile_size: Size) -> Pointi32 {
	return Pointi32 {
		left: (pointf32.left / tile_size.width) as i32,
		top: (pointf32.top / tile_size.height) as i32,
	};
}

#[inline(always)]
pub fn get_view_size(view_pixels: PixelSize, scale: f32) -> Size {
	return Size {
		width: (view_pixels.width as f32) / scale,
		height: (view_pixels.height as f32) / scale,
	};
}

#[inline(always)]
pub fn visible_tile_bounds(camera: Pointf32, view_pixels: PixelSize, scale: f32, tile_size: Size, level_width_tiles: i32, level_height_tiles: i32) -> TileRect {
	let view_size = get_view_size(view_pixels, scale);

	let start = get_tile(camera, tile_size);

	let end = Pointf32 {
		left: camera.left + view_size.width,
		top: camera.top + view_size.height,
	};
	let end = get_tile(end, tile_size);

	return TileRect {
		start_left: (start.left - 1).max(0),
		start_top: (start.top - 1).max(0),
		end_left: (end.left + 2).min(level_width_tiles),
		end_top: (end.top + 2).min(level_height_tiles),
	};
}

#[inline(always)]
pub fn clamp_camera_to_level_world(cam: Pointf32, view_pixels: PixelSize, scale: f32, tile_size: Size, level_width_tiles: i32, level_height_tiles: i32) -> Pointf32 {
	let view_width_world: f32 = (view_pixels.width as f32) / scale;
	let view_height_world: f32 = (view_pixels.height as f32) / scale;

	let level_width_world: f32 = (level_width_tiles as f32) * tile_size.width;
	let level_height_world: f32 = (level_height_tiles as f32) * tile_size.height;

	let max_left: f32 = (level_width_world - view_width_world).max(0.0);
	let max_top: f32 = (level_height_world - view_height_world).max(0.0);

	return Pointf32 {
		left: cam.left.clamp(0.0, max_left),
		top: cam.top.clamp(0.0, max_top),
	};
}
