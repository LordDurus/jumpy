const RENDER_SCALE: f32 = 1.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 360;
const TILE_PIXELS: u32 = 16;

#[path = "pc_platform.rs"]
mod pc_platform;

use crate::{
	common::coords::{PixelSize, Pointf32, Size, clamp_camera_to_level_world, get_screen, visible_tile_bounds},
	game::{
		game_state::{EntityKind, GameState},
		level::Level,
	},
	platform::{RenderBackend, input::InputState, render::common::RenderCommon},
	tile::TileKind,
};
use sdl2::{
	EventPump,
	image::LoadTexture,
	pixels::Color,
	rect::Rect,
	render::{BlendMode, Canvas, Texture},
	video::Window,
};
use std::path::{Path, PathBuf};

pub struct PcRenderer {
	canvas: Canvas<Window>,
	event_pump: EventPump,
	common: RenderCommon,
	pub frame_index: u32,

	// bg parallax
	bg_texture: Option<Texture<'static>>,
	bg_parallax_x: f32,
	bg_parallax_y: f32,
}

impl PcRenderer {
	fn draw_filled_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
		self.canvas.set_draw_color(color);
		let rect = Rect::new(x, y, w, h);
		let _ = self.canvas.fill_rect(rect);
		return;
	}

	fn render_scale(&self) -> f32 {
		return RENDER_SCALE;
	}

	fn draw_filled_circle(&mut self, cx: i32, cy: i32, r: i32, color: Color) {
		self.canvas.set_draw_color(color);

		let rr: i32 = r * r;
		let mut y: i32 = -r;
		while y <= r {
			let yy: i32 = y * y;
			let dx: f32 = ((rr - yy) as f32).sqrt();
			let x0: i32 = cx - dx.round() as i32;
			let x1: i32 = cx + dx.round() as i32;

			let _ = self.canvas.draw_line((x0, cy + y), (x1, cy + y));
			y += 1;
		}

		return;
	}

	fn draw_filled_triangle(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
		self.canvas.set_draw_color(color);

		let ww: i32 = w as i32;
		let hh: i32 = h as i32;

		let x0: i32 = x;
		let y0: i32 = y + hh;

		let x1: i32 = x + ww;
		let y1: i32 = y + hh;

		let x2: i32 = x + ww / 2;
		let y2: i32 = y;

		// simple scanline fill
		let min_y: i32 = y2.min(y0.min(y1));
		let max_y: i32 = y2.max(y0.max(y1));

		let mut yy: i32 = min_y;
		while yy <= max_y {
			let mut xs: Vec<i32> = Vec::new();
			Self::tri_edge_intersect_y(x0, y0, x1, y1, yy, &mut xs);
			Self::tri_edge_intersect_y(x1, y1, x2, y2, yy, &mut xs);
			Self::tri_edge_intersect_y(x2, y2, x0, y0, yy, &mut xs);

			if xs.len() >= 2 {
				xs.sort();
				let _ = self.canvas.draw_line((xs[0], yy), (xs[xs.len() - 1], yy));
			}

			yy += 1;
		}

		return;
	}

	fn tri_edge_intersect_y(x0: i32, y0: i32, x1: i32, y1: i32, y: i32, out: &mut Vec<i32>) {
		if (y < y0 && y < y1) || (y > y0 && y > y1) || (y0 == y1) {
			return;
		}

		let dy: i32 = y1 - y0;
		let dx: i32 = x1 - x0;

		let t_num: i32 = y - y0;
		let x: i32 = x0 + (dx * t_num) / dy;

		out.push(x);

		return;
	}

	fn draw_background(&mut self, cam_x_world: i32, cam_y_world: i32, _scale: f32) {
		// always draw a sky fallback so you know this ran
		let (sw, sh) = self.canvas.output_size().unwrap_or((WINDOW_WIDTH, WINDOW_HEIGHT));
		self.canvas.set_draw_color(Color::RGB(60, 110, 190));
		let _ = self.canvas.fill_rect(Rect::new(0, 0, sw, sh));

		let Some(bg) = self.bg_texture.as_ref() else {
			return;
		};

		let q = bg.query();
		let bg_width: i32 = q.width as i32;
		let bg_height: i32 = q.height as i32;
		if bg_width <= 0 || bg_height <= 0 {
			return;
		}

		let bg_cam_x: f32 = cam_x_world as f32 * self.bg_parallax_x * self.bg_parallax_x;
		let bg_cam_y: f32 = cam_y_world as f32 * self.bg_parallax_y * self.bg_parallax_y;

		let start_left: i32 = -(((bg_cam_x as i32) % bg_width + bg_width) % bg_width);
		let mut start_top: i32 = -(((bg_cam_y as i32) % bg_height + bg_height) % bg_height);

		if bg_height >= sh as i32 {
			start_top = 0;
		}

		let mut x: i32 = start_left;
		while x < sw as i32 {
			let mut y: i32 = start_top;
			while y < sh as i32 {
				let dst = Rect::new(x, y, q.width, q.height);
				let _ = self.canvas.copy(bg, None, dst);
				y += bg_height;
			}
			x += bg_width;
		}

		return;
	}

	fn draw_tiles_layer_atlas(
		&mut self,
		tile_tex: &sdl2::render::Texture,
		tile_pixel: u32,
		level: &Level,
		layer: u32,
		camera_left: f32,
		camera_top: f32,
		scale: f32,
		_frame_index: u32,
	) {
		let tile_width: f32 = level.tile_width as f32;
		let tile_height: f32 = level.tile_height as f32;
		let cam = Pointf32::new(camera_left, camera_top);
		let tile_size = Size::new(level.tile_width as f32, level.tile_height as f32);
		let view_pixels = PixelSize::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
		let cam = clamp_camera_to_level_world(cam, view_pixels, scale, tile_size, level.width as i32, level.height as i32);
		let bounds = visible_tile_bounds(cam, view_pixels, scale, tile_size, level.width as i32, level.height as i32);
		let start_tile_left: i32 = bounds.start_left;
		let start_tile_top: i32 = bounds.start_top;
		let end_tile_left: i32 = bounds.end_left;
		let end_tile_top: i32 = bounds.end_top;
		let tile_cols: u32 = tile_tex.query().width / tile_pixel;
		let scale_i32: i32 = scale as i32;
		let tile_pixel_scaled: i32 = tile_pixel as i32 * scale_i32;

		let cam_left_pixels: i32 = (cam.left * scale).floor() as i32;
		let cam_top_pixels: i32 = (cam.top * scale).floor() as i32;

		for ty in start_tile_top..end_tile_top {
			for tx in start_tile_left..end_tile_left {
				let tile_id: u8 = level.get_tile_id_at_layer(layer, tx, ty);
				if tile_id == 0 {
					continue; // empty
				}
				let id = tile_id as u32;

				let source_left: i32 = ((id % tile_cols) * tile_pixel) as i32;
				let source_top: i32 = ((id / tile_cols) * tile_pixel) as i32;
				let source = sdl2::rect::Rect::new(source_left, source_top, tile_pixel, tile_pixel);

				let destination_left: i32 = tx * tile_pixel_scaled - cam_left_pixels;
				let destination_top: i32 = ty * tile_pixel_scaled - cam_top_pixels;

				let destination = Rect::new(
					destination_left,
					destination_top,
					(tile_width * scale).round() as u32,
					(tile_height * scale).round() as u32,
				);

				let _ = self.canvas.copy(tile_tex, source, destination).unwrap();
			}
		}

		return;
	}

	fn draw_level_internal(&mut self, game_state: &GameState) {
		let (cam_left_world, cam_top_world) = self.common.compute_camera(self, game_state);
		let scale: f32 = self.render_scale();

		// background first, tiles on top
		self.draw_background(cam_left_world, cam_top_world, scale);

		let texture_creator = self.canvas.texture_creator();
		let tile_path: PathBuf = get_asset_root().join("pc").join("tiles.png");
		let tile_tex: sdl2::render::Texture = texture_creator.load_texture(&tile_path).unwrap();
		let tile_cols: u32 = tile_tex.query().width / TILE_PIXELS;

		for layer in 0..(game_state.level.layer_count as u32) {
			self.draw_tiles_layer_atlas(
				&tile_tex,
				TILE_PIXELS,
				&game_state.level,
				layer,
				cam_left_world as f32,
				cam_top_world as f32,
				scale,
				self.frame_index,
			);
		}

		self.frame_index = self.frame_index.wrapping_add(1);

		self.draw_entities(game_state, &tile_tex, tile_cols, cam_left_world as f32, cam_top_world as f32, scale, self.frame_index);

		return;
	}

	fn draw_entities(
		&mut self,
		game_state: &GameState,
		texture: &sdl2::render::Texture,
		tile_cols: u32,
		camera_left: f32,
		camera_top: f32,
		scale: f32,
		_frame_index: u32,
	) {
		for (id, pos) in game_state.positions.iter() {
			let kind = *game_state.entity_kinds.get(id).unwrap_or(&0);
			let entity_kind = EntityKind::from_u8(kind);

			if entity_kind == EntityKind::Empty {
				println!("Warning: entity id {} has unknown kind {}", id, kind);
				continue;
			}

			let style: u8 = *game_state.render_styles.get(id).unwrap_or(&0);
			let (half_width, half_height) = game_state.get_entity_half_values(id);
			let world_left: f32 = pos.x - half_width;
			let world_top: f32 = pos.y - half_height;
			let cam: Pointf32 = Pointf32::new(camera_left, camera_top);

			let world: Pointf32 = Pointf32 {
				left: world_left,
				top: world_top,
			};
			let screen = get_screen(world, cam, scale);

			let scale_left: i32 = screen.left;
			let scale_top: i32 = screen.top;
			let width: u32 = ((half_width * 2.0) * scale).round() as u32;
			let height: u32 = ((half_height * 2.0) * scale).round() as u32;

			if entity_kind == EntityKind::MovingPlatform {
				let width_pixels: f32 = *game_state.widths.get(id).unwrap_or(&16) as f32;
				let tile_width: f32 = game_state.level.tile_width as f32;
				let width_tiles: i32 = ((width_pixels / tile_width).ceil() as i32).max(1);

				self.draw_platform_entity_tiles(
					texture,
					tile_cols,
					TILE_PIXELS,
					world_left,
					world_top,
					width_tiles,
					&game_state.level,
					camera_left,
					camera_top,
					scale,
					TileKind::MovingPlatformLeft,
					TileKind::MovingPlatformMiddle,
					TileKind::MovingPlatformRight,
				);
				continue;
			}

			let color: Color = match entity_kind {
				EntityKind::Empty => Color::RGB(0, 0, 0),              // not set (black)
				EntityKind::Player => Color::RGB(255, 255, 255),       // player (white)
				EntityKind::Slime => Color::RGB(64, 160, 255),         // slime (blue)
				EntityKind::Imp => Color::RGB(64, 200, 64),            // imp (green)
				EntityKind::MovingPlatform => Color::RGB(255, 255, 0), // platform (yellow)
			};

			match style {
				2 => {
					let cx: i32 = scale_left + (width as i32 / 2);
					let cy: i32 = scale_top + (height as i32 / 2);
					let r: i32 = (width.min(height) as i32) / 2;
					self.draw_filled_circle(cx, cy, r, color);
				}
				3 => {
					self.draw_filled_triangle(scale_left, scale_top, width, height, color);
				}
				_ => {
					self.draw_filled_rect(scale_left, scale_top, width, height, color);
				}
			}
		}

		return;
	}
}

impl RenderBackend for PcRenderer {
	fn init(&mut self) {
		// nothing special yet
	}

	fn new() -> Self {
		let sdl = sdl2::init().unwrap();
		let _ = sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0"); // nearest

		let _image = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
		let video = sdl.video().unwrap();
		let window = video.window("jumpy", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().build().unwrap();
		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
		let event_pump = sdl.event_pump().unwrap();

		let creator_box = Box::new(canvas.texture_creator());
		let texture_creator: &'static sdl2::render::TextureCreator<sdl2::video::WindowContext> = Box::leak(creator_box);

		let bg_path: PathBuf = get_asset_root().join("pc").join("bg_parallax_forest.png");
		let mut bg_texture = texture_creator.load_texture(bg_path).expect("failed to load bg_parallax_forest.png");

		bg_texture.set_blend_mode(BlendMode::Blend);
		bg_texture.set_alpha_mod(208);

		return PcRenderer {
			canvas,
			event_pump,
			common: RenderCommon::new(),
			frame_index: 0,
			bg_texture: Some(bg_texture),
			bg_parallax_x: 0.35,
			bg_parallax_y: 0.15,
		};
	}

	fn screen_size(&self) -> (i32, i32) {
		let (w, h) = self.canvas.output_size().unwrap();
		return (w as i32, h as i32);
	}

	fn render_scale(&self) -> f32 {
		return RENDER_SCALE;
	}

	fn poll_input(&mut self) -> InputState {
		return crate::platform::input::pc::poll(&mut self.event_pump);
	}

	fn begin_frame(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	fn draw_level(&mut self, game_state: &GameState) {
		self.draw_level_internal(game_state);
	}

	fn commit(&mut self) {
		self.canvas.present();
	}
}

fn get_asset_root() -> PathBuf {
	let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("assets").join("gfx");
	return root;
}
