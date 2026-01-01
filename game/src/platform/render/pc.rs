const RENDER_SCALE: f32 = 1.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 360;

use crate::{
	game::game_state::GameState,
	platform::{RenderBackend, input::InputState, render::common::RenderCommon},
	tile::TileKind,
};

use sdl2::{
	EventPump,
	image::{InitFlag, LoadTexture},
	pixels::Color,
	rect::Rect,
	render::{Canvas, Texture, TextureCreator},
	video::{Window, WindowContext},
};

pub struct PcRenderer {
	canvas: Canvas<Window>,
	event_pump: EventPump,
	common: RenderCommon,

	// bg parallax
	texture_creator: &'static TextureCreator<WindowContext>,
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

	fn get_window_size(&self) -> (i32, i32) {
		let (w, h) = self.canvas.output_size().unwrap();
		return (w as i32, h as i32);
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

	fn draw_background(&mut self, cam_x_world: i32, cam_y_world: i32, scale: f32) {
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

		/*
		let cam_x_px: f32 = cam_x_world as f32 * scale;
		let cam_y_px: f32 = cam_y_world as f32 * scale;
		*/

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

	fn draw_level(&mut self, game_state: &GameState) {
		let level = &game_state.level;
		let (cam_x_world, cam_y_world) = self.common.compute_camera(self, game_state);
		let scale: f32 = self.render_scale();

		self.draw_background(cam_x_world, cam_y_world, scale);

		let tile_width_world: i32 = level.tile_width as i32;
		let tile_height_world: i32 = level.tile_height as i32;

		let tile_width_scale: u32 = (level.tile_width as f32 * scale).round() as u32;
		let tile_height_scale: u32 = (level.tile_height as f32 * scale).round() as u32;

		// tiles
		for ty in 0..(level.height as i32) {
			for tx in 0..(level.width as i32) {
				let layer: u32 = level.collision_layer_index() as u32;
				let kind: TileKind = level.tile_at_layer(layer, tx, ty);
				if kind == TileKind::Empty {
					continue;
				}

				match kind {
					TileKind::Dirt => self.canvas.set_draw_color(Color::RGB(110, 72, 36)),
					TileKind::GrassTop => self.canvas.set_draw_color(Color::RGB(48, 160, 64)),
					TileKind::Water => self.canvas.set_draw_color(Color::RGB(48, 96, 200)),
					TileKind::SpikeUp | TileKind::SpikeDown | TileKind::SpikeLeft | TileKind::SpikeRight => self.canvas.set_draw_color(Color::RGB(200, 48, 48)),
					TileKind::Empty => {}
				}

				let world_x: i32 = tx * tile_width_world;
				let world_y: i32 = ty * tile_height_world;

				let sx: i32 = (((world_x - cam_x_world) as f32) * scale).round() as i32;
				let sy: i32 = (((world_y - cam_y_world) as f32) * scale).round() as i32;

				let rect = Rect::new(sx, sy, tile_width_scale, tile_height_scale);
				let _ = self.canvas.fill_rect(rect);
			}
		}

		// entities (single pass: kind -> color, render_style -> shape)
		for (id, pos) in game_state.positions.iter() {
			let kind: u8 = *game_state.entity_kind.get(id).unwrap_or(&0);
			let style: u8 = *game_state.render_style.get(id).unwrap_or(&0);

			let (half_width, half_height) = game_state.get_entity_half_values(*id);

			let world_left: f32 = pos.x - half_width;
			let world_top: f32 = pos.y - half_height;

			let scale_top: i32 = ((world_top - cam_y_world as f32) * scale).round() as i32;
			let scale_left: i32 = ((world_left - cam_x_world as f32) * scale).round() as i32;

			let width: u32 = ((half_width * 2.0) * scale).round() as u32; // TODO: get the entiy widht
			let height: u32 = ((half_height * 2.0) * scale).round() as u32;

			let color: Color = match kind {
				0 => Color::RGB(0, 0, 0),       // not set (black)
				1 => Color::RGB(255, 255, 255), // player (white)
				2 => Color::RGB(64, 160, 255),  // slime (blue)
				3 => Color::RGB(64, 200, 64),   // imp (green)
				4 => Color::RGB(255, 255, 0),   // platform (yellow)
				_ => Color::RGB(255, 0, 255),   // debug
			};

			// println!("draw entity id={} kind={} style={} color={:?}", id, kind, style, color);

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

		let _image = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
		let video = sdl.video().unwrap();
		let window = video.window("jumpy", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().build().unwrap();
		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
		let event_pump = sdl.event_pump().unwrap();

		let creator_box = Box::new(canvas.texture_creator());
		let texture_creator: &'static sdl2::render::TextureCreator<sdl2::video::WindowContext> = Box::leak(creator_box);

		// TODO: use world/level image
		let bg_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/gfx/pc/bg_parallax1.png");
		let bg_texture = texture_creator.load_texture(bg_path).ok();
		if bg_texture.is_none() {
			println!("manifest_dir={}", env!("CARGO_MANIFEST_DIR"));
			println!("bg texture failed to load: {}", bg_path);
		}

		return PcRenderer {
			canvas,
			event_pump,
			common: RenderCommon::new(),
			texture_creator,
			bg_texture,
			bg_parallax_x: 0.35,
			bg_parallax_y: 0.15,
		};
	}

	fn screen_size(&self) -> (i32, i32) {
		let (w, h) = self.canvas.output_size().unwrap();
		return (w as i32, h as i32);
	}

	fn draw_tile(&mut self, sheet_id: u16, x: i32, y: i32, w: u32, h: u32) {
		// temporary: colored blocks, ignore sheet_id or map it to a color
		// if you want a simple mapping:
		if sheet_id == 0 {
			return;
		}

		// pick a default color (you can improve this later)
		self.canvas.set_draw_color(Color::RGB(255, 255, 255));

		let rect = Rect::new(x, y, w, h);
		let _ = self.canvas.fill_rect(rect);
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
		// println!("RenderBackend: draw_world");
		self.draw_level(game_state);
	}

	fn commit(&mut self) {
		self.canvas.present();
	}
}
