pub const BACKGROUND_ID_LIBRARY_STONE: u8 = 1;
pub const BACKGROUND_PARALLAX_FOREST: u8 = 2;

#[path = "pc_platform.rs"]
mod pc_platform;

use crate::{
	assets::get_gfx_root,
	common::coords::{PixelSize, Pointf32, Size, clamp_camera_to_level_world, get_screen, visible_tile_bounds},
	engine_math::Vec2,
	game::{
		game_session::{self, GameSession},
		game_state::{EntityKind, GameState},
		level::Level,
		triggers::TriggerKind,
	},
	platform::{
		RenderBackend,
		audio::backend::LocomotionAnim,
		input::InputState,
		render::{
			common::RenderCommon,
			pc::pc_platform::{WindowSettings, load_window_settings, save_window_settings},
		},
	},
	tile::TileKind,
};
use sdl2::ttf::{Font, Sdl2TtfContext};

use sdl2::{
	EventPump,
	image::LoadTexture,
	pixels::Color,
	rect::Rect,
	render::{BlendMode, Canvas, Texture},
	video::Window,
};
use std::path::PathBuf;

#[derive(Clone, Copy)]
enum SlimeTextureKey {
	BlueWalk,
	BlueRun,
	BlueDeath,
	UndeadWalk,
	UndeadRun,
	UndeadDeath,
	LavaWalk,
	LavaRun,
	LavaDeath,
}

pub struct PcRenderer {
	video: sdl2::VideoSubsystem,
	canvas: Canvas<Window>,
	event_pump: EventPump,
	common: RenderCommon,

	slime_blue_walk_texture: Texture<'static>,
	slime_blue_run_texture: Texture<'static>,
	slime_blue_death_texture: Texture<'static>,
	slime_undead_walk_texture: Texture<'static>,
	slime_undead_run_texture: Texture<'static>,
	slime_undead_death_texture: Texture<'static>,
	slime_lava_walk_texture: Texture<'static>,
	slime_lava_run_texture: Texture<'static>,
	slime_lava_death_texture: Texture<'static>,

	pub frame_index: u32,
	pub atlas_tile_width_pixels: u32,
	pub atlas_tile_height_pixels: u32,

	// pub ttf: &'static Sdl2TtfContext,
	pub book_font: Font<'static, 'static>,

	// texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
	texture_creator: &'static sdl2::render::TextureCreator<sdl2::video::WindowContext>,

	// bg parallax
	bg_texture: Option<Texture<'static>>,
	bg_id: u8,

	tile_texture: Option<Texture<'static>>,
	bg_parallax_x: f32,
	bg_parallax_y: f32,
	render_scale: u32,
	//pub fn get_background_file_name(background_id: u8) -> &'static str),
}

impl Drop for PcRenderer {
	fn drop(&mut self) {
		save_window_settings(self.canvas.window());
	}
}

impl PcRenderer {
	pub fn set_level_background(&mut self, background_id: u8) {
		if self.bg_texture.is_some() && self.bg_id == background_id {
			return;
		}

		self.bg_id = background_id;
		let file_name = parse_background_id(background_id);
		let bg_path = gfx_pc_path(&["background", file_name]);

		let bg_texture = load_texture(&self.texture_creator, bg_path);
		self.bg_texture = Some(bg_texture);
		// let _ = sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0"); // nearest

		if let Some(background) = self.bg_texture.as_mut() {
			background.set_blend_mode(BlendMode::Blend);
			background.set_alpha_mod(208);
		}

		// optional: set parallax rules per background id
		if background_id == BACKGROUND_ID_LIBRARY_STONE {
			self.bg_parallax_x = 0.35;
			self.bg_parallax_y = 0.15;
		} else if background_id == BACKGROUND_PARALLAX_FOREST {
			self.bg_parallax_x = 0.35;
			self.bg_parallax_y = 0.15;
		} else {
			self.bg_parallax_x = 0.0;
			self.bg_parallax_y = 0.0;
		}
	}

	pub fn copy_book_page_to_clipboard(&self, text: &str) {
		let clipboard = self.video.clipboard();
		let _ = clipboard.set_clipboard_text(text);
	}

	fn draw_book_footer(&mut self, panel_left: i32, panel_top: i32, panel_width_pixels: u32, panel_height_pixels: u32) {
		let padding_pixels: i32 = 16;
		let footer_height_pixels: i32 = 34;

		let footer_left: i32 = panel_left;
		let footer_top: i32 = panel_top + (panel_height_pixels as i32) - footer_height_pixels;
		let footer_width_pixels: u32 = panel_width_pixels;
		let footer_height_pixels_u32: u32 = footer_height_pixels as u32;

		// footer bar (same scheme, slightly lighter)
		self.canvas.set_draw_color(Color::RGBA(28, 28, 40, 235));
		let _ = self
			.canvas
			.fill_rect(Rect::new(footer_left, footer_top, footer_width_pixels, footer_height_pixels_u32));

		// optional: subtle divider line at top of footer
		self.canvas.set_draw_color(Color::RGBA(60, 60, 80, 110));
		let _ = self
			.canvas
			.draw_line((footer_left, footer_top), (footer_left + footer_width_pixels as i32, footer_top));

		// text
		let left_text: &str = "esc: close    \u{2190}/\u{2192}: page    ctrl+c: copy";
		let right_text: &str = "r: read";

		let text_top: i32 = footer_top + 8;

		self.draw_book_text_line(footer_left + padding_pixels, text_top, left_text);

		let (right_width_pixels, _) = match self.book_font.size_of(right_text) {
			Ok(v) => v,
			Err(_) => return,
		};

		let right_left: i32 = footer_left + (footer_width_pixels as i32) - padding_pixels - (right_width_pixels as i32);
		self.draw_book_text_line(right_left, text_top, right_text);

		return;
	}

	pub fn draw_book_overlay(&mut self, session: &GameSession) {
		let state = &session.book_reading;
		if !state.is_open {
			return;
		}

		let (screen_width_pixels, screen_height_pixels) = self.screen_size();

		// --- dim background ---
		self.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
		self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 180));
		let _ = self.canvas.fill_rect(Rect::new(0, 0, screen_width_pixels as u32, screen_height_pixels as u32));

		// --- panel ---
		let panel_margin_pixels: i32 = 40;
		let panel_left: i32 = panel_margin_pixels;
		let panel_top: i32 = panel_margin_pixels;
		let panel_width_pixels: u32 = screen_width_pixels as u32 - (panel_margin_pixels as u32 * 2);
		let panel_height_pixels: u32 = screen_height_pixels as u32 - (panel_margin_pixels as u32 * 2);

		self.canvas.set_draw_color(Color::RGBA(20, 20, 28, 255));
		let _ = self.canvas.fill_rect(Rect::new(panel_left, panel_top, panel_width_pixels, panel_height_pixels));

		// --- render text (simple: one line per row, clipped) ---
		let padding_pixels: i32 = 18;
		let text_left: i32 = panel_left + padding_pixels;
		let mut text_top: i32 = panel_top + padding_pixels;

		// header line
		let header = format!("{}  page {}/{}", state.book_slug, state.page_index + 1, state.total_pages);
		self.draw_book_header_right(panel_left, panel_top, panel_width_pixels, &header);

		text_top += 26;

		// page body
		for line in state.page_text.lines() {
			if text_top > (panel_top + panel_height_pixels as i32 - padding_pixels - 26) {
				break;
			}
			self.draw_book_text_line(text_left, text_top, line);
			self.draw_book_footer(panel_left, panel_top, panel_width_pixels, panel_height_pixels);
			text_top += 22;
		}
	}

	fn draw_book_header_right(&mut self, panel_left: i32, panel_top: i32, panel_width_pixels: u32, text: &str) {
		let (text_width_pixels, _) = match self.book_font.size_of(text) {
			Ok(v) => v,
			Err(_) => return,
		};

		let padding_pixels: i32 = 16;
		let left: i32 = panel_left + (panel_width_pixels as i32) - padding_pixels - (text_width_pixels as i32);
		let top: i32 = panel_top + padding_pixels;

		self.draw_book_text_line(left, top, text);
	}

	fn draw_book_text_line(&mut self, left: i32, top: i32, text: &str) {
		// white text
		let surface = self.book_font.render(text).blended(Color::RGBA(235, 235, 235, 255));
		let surface = match surface {
			Ok(s) => s,
			Err(_) => return,
		};

		let texture = self.texture_creator.create_texture_from_surface(&surface);
		let texture = match texture {
			Ok(t) => t,
			Err(_) => return,
		};

		let query = texture.query();
		let destination = Rect::new(left, top, query.width, query.height);
		let _ = self.canvas.copy(&texture, None, Some(destination));
	}

	fn draw_debug_triggers(&mut self, game_state: &GameState, session: &GameSession, cam_left_world: f32, cam_top_world: f32, scale: f32) {
		if !session.settings.show_triggers {
			return;
		}

		use sdl2::{pixels::Color, rect::Rect};

		let tile_width_world: f32 = game_state.level.tile_width as f32;
		let tile_height_world: f32 = game_state.level.tile_height as f32;

		for t in &game_state.level.triggers {
			let idx: usize = t.id as usize;
			if idx < game_state.trigger_armed.len() && game_state.trigger_armed[idx] {
				continue; // consumed -> don't draw
			}

			let left_world: f32 = (t.left_tiles as f32) * tile_width_world;
			let top_world: f32 = (t.top_tiles as f32) * tile_height_world;
			let width_world: f32 = (t.width_tiles as f32) * tile_width_world;
			let height_world: f32 = (t.height_tiles as f32) * tile_height_world;

			let left_pixels: i32 = ((left_world - cam_left_world) * scale).round() as i32;
			let top_pixels: i32 = ((top_world - cam_top_world) * scale).round() as i32;
			let width_pixels: u32 = (width_world * scale).round().max(1.0) as u32;
			let height_pixels: u32 = (height_world * scale).round().max(1.0) as u32;

			// pick different colors by kind (optional)
			if t.kind == TriggerKind::Message as u8 {
				self.canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
			} else {
				self.canvas.set_draw_color(Color::RGBA(255, 255, 0, 255));
			}

			let _ = self.canvas.draw_rect(Rect::new(left_pixels, top_pixels, width_pixels, height_pixels));
		}
	}

	fn get_slime_texture_key(&self, kind: EntityKind, anim: LocomotionAnim) -> SlimeTextureKey {
		match (kind, anim) {
			(EntityKind::SlimeBlue, LocomotionAnim::Walk) => return SlimeTextureKey::BlueWalk,
			(EntityKind::SlimeBlue, LocomotionAnim::Run) => return SlimeTextureKey::BlueRun,
			(EntityKind::SlimeBlue, LocomotionAnim::Death) => return SlimeTextureKey::BlueDeath,

			(EntityKind::SlimeUndead, LocomotionAnim::Walk) => return SlimeTextureKey::UndeadWalk,
			(EntityKind::SlimeUndead, LocomotionAnim::Run) => return SlimeTextureKey::UndeadRun,
			(EntityKind::SlimeUndead, LocomotionAnim::Death) => return SlimeTextureKey::UndeadDeath,

			(EntityKind::SlimeLava, LocomotionAnim::Walk) => return SlimeTextureKey::LavaWalk,
			(EntityKind::SlimeLava, LocomotionAnim::Run) => return SlimeTextureKey::LavaRun,
			(EntityKind::SlimeLava, LocomotionAnim::Death) => return SlimeTextureKey::LavaDeath,

			_ => panic!("get_slime_texture_key called for non-slime {:?}", kind),
		}
	}

	fn draw_filled_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
		self.canvas.set_draw_color(color);
		let rect = Rect::new(x, y, w, h);
		let _ = self.canvas.fill_rect(rect);
		return;
	}

	fn draw_filled_circle(&mut self, circle_x: i32, circle_y: i32, radius: i32, color: Color) {
		self.canvas.set_draw_color(color);

		let rr: i32 = radius * radius;
		let mut y: i32 = -radius;
		while y <= radius {
			let yy: i32 = y * y;
			let dx: f32 = ((rr - yy) as f32).sqrt();
			let x0: i32 = circle_x - dx.round() as i32;
			let x1: i32 = circle_x + dx.round() as i32;

			let _ = self.canvas.draw_line((x0, circle_y + y), (x1, circle_y + y));
			y += 1;
		}

		return;
	}

	fn draw_color_only_tile(&mut self, tile_kind: TileKind, destination: Rect) {
		self.canvas.set_blend_mode(BlendMode::Blend);

		match tile_kind {
			TileKind::Blackout => {
				self.canvas.set_blend_mode(BlendMode::Blend);
				self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
				let _ = self.canvas.fill_rect(destination);
			}

			TileKind::TorchGlow => {
				// make this area less dark than full blackout
				self.canvas.set_blend_mode(BlendMode::Blend);
				self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 200));
				let _ = self.canvas.fill_rect(destination);

				// add warm light (needs a real alpha)
				self.canvas.set_blend_mode(BlendMode::Add);
				self.canvas.set_draw_color(Color::RGBA(255, 235, 160, 220)); // pale warm yellow
				let _ = self.canvas.fill_rect(destination);

				self.canvas.set_blend_mode(BlendMode::Blend);
			}

			TileKind::DarkBrownRock => {
				self.canvas.set_blend_mode(BlendMode::Blend);
				self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 235));
				let _ = self.canvas.fill_rect(destination);
				self.canvas.set_blend_mode(BlendMode::Add);
				self.canvas.set_draw_color(Color::RGBA(255, 235, 100, 80)); // pale yellow
				let _ = self.canvas.fill_rect(destination);
				self.canvas.set_blend_mode(BlendMode::Blend);
			}

			_ => {
				// Silently do nothing instead of drawing the wrong thing
			}
		}

		return;
	}

	fn draw_filled_triangle(&mut self, x: i32, y: i32, width: u32, h: u32, color: Color) {
		self.canvas.set_draw_color(color);

		let ww: i32 = width as i32;
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

	fn draw_background(&mut self, cam_left_world: i32, cam_top_world: i32, scale: f32) {
		let (sw_u32, sh_u32) = match self.canvas.output_size() {
			Ok(v) => v,
			Err(_) => self.canvas.window().size(),
		};

		// sky fallback
		self.canvas.set_draw_color(Color::RGB(60, 110, 190));
		let _ = self.canvas.fill_rect(Rect::new(0, 0, sw_u32, sh_u32));

		let Some(bg) = self.bg_texture.as_ref() else {
			return;
		};

		let query = bg.query();
		if query.width == 0 || query.height == 0 {
			return;
		}

		let bg_tile_width_pixels: i32 = query.width as i32;
		let bg_tile_height_pixels: i32 = query.height as i32;

		if bg_tile_width_pixels <= 0 || bg_tile_height_pixels <= 0 {
			return;
		}

		let sw: i32 = sw_u32 as i32;
		let sh: i32 = sh_u32 as i32;

		// camera -> pixels
		let cam_left_pixels: f32 = cam_left_world as f32 * scale;
		let cam_top_pixels: f32 = cam_top_world as f32 * scale;

		// parallax offsets in pixels
		let bg_cam_left_pixels: i32 = (cam_left_pixels * self.bg_parallax_x).floor() as i32;
		let bg_cam_top_pixels: i32 = (cam_top_pixels * self.bg_parallax_y).floor() as i32;

		// horizontal wrap (repeat)
		let start_left: i32 = -(((bg_cam_left_pixels % bg_tile_width_pixels) + bg_tile_width_pixels) % bg_tile_width_pixels);

		// vertical clamp (no repeat)
		let mut top: i32 = -bg_cam_top_pixels;
		if bg_tile_height_pixels >= sh {
			let min_top: i32 = sh - bg_tile_height_pixels; // negative or 0
			if top < min_top {
				top = min_top;
			}
			if top > 0 {
				top = 0;
			}
		} else {
			// bg shorter than screen: pin to top (sky fill covers the rest)
			top = 0;
		}

		let mut left: i32 = start_left;
		while left < sw {
			let dst = Rect::new(left, top, bg_tile_width_pixels as u32, bg_tile_height_pixels as u32);
			let _ = self.canvas.copy(bg, None, dst);
			left += bg_tile_width_pixels;
		}
	}

	fn draw_tiles_layer_atlas(&mut self, level: &Level, layer: u32, camera_left: f32, camera_top: f32, scale: f32, _frame_index: u32) {
		let tile_width: f32 = level.tile_width as f32;
		let tile_height: f32 = level.tile_height as f32;
		let cam = Pointf32::new(camera_left, camera_top);
		let tile_size = Size::new(level.tile_width as f32, level.tile_height as f32);

		let (view_width_pixels, view_height_pixels) = match self.canvas.output_size() {
			Ok(v) => v,
			Err(_) => self.canvas.window().size(),
		};
		let view_pixels = PixelSize::new(view_width_pixels as i32, view_height_pixels as i32);

		let cam = clamp_camera_to_level_world(cam, view_pixels, scale, tile_size, level.width as i32, level.height as i32);
		let bounds = visible_tile_bounds(cam, view_pixels, scale, tile_size, level.width as i32, level.height as i32);
		let start_tile_left: i32 = bounds.start_left;
		let start_tile_top: i32 = bounds.start_top;
		let end_tile_left: i32 = bounds.end_left;
		let end_tile_top: i32 = bounds.end_top;
		let atlas_tile_width_pixels: u32 = self.atlas_tile_width_pixels;
		let atlas_tile_height_pixels: u32 = self.atlas_tile_height_pixels;
		let tile_cols: u32 = self.tile_texture.as_mut().unwrap().query().width / atlas_tile_width_pixels;

		for tile_top in start_tile_top..end_tile_top {
			for tile_left in start_tile_left..end_tile_left {
				let tile_id: u8 = level.get_tile_id_at_layer(layer, tile_left, tile_top);
				let tile_kind: TileKind = TileKind::from_u8(tile_id);
				if tile_kind.is_empty() {
					continue;
				}

				let tile_dest_width_pixels: i32 = (tile_width * scale).round() as i32;
				let tile_dest_height_pixels: i32 = (tile_height * scale).round() as i32;

				let camera_left_pixels: i32 = (cam.left * scale).floor() as i32;
				let camera_top_pixels: i32 = (cam.top * scale).floor() as i32;

				let destination_left: i32 = tile_left * tile_dest_width_pixels - camera_left_pixels;
				let destination_top: i32 = tile_top * tile_dest_height_pixels - camera_top_pixels;

				let destination = Rect::new(destination_left, destination_top, tile_dest_width_pixels as u32, tile_dest_height_pixels as u32);

				/*
				let scale_i32: i32 = scale as i32;
				let camera_left_pixels: i32 = (cam.left * scale).floor() as i32;
				let camera_top_pixels: i32 = (cam.top * scale).floor() as i32;
				let tile_pixel_scaled: i32 = atlas_tile_width_pixels as i32 * scale_i32;
				let destination_left: i32 = tile_left * tile_pixel_scaled - camera_left_pixels;
				let destination_top: i32 = tile_top * tile_pixel_scaled - camera_top_pixels;


				let destination = Rect::new(
					destination_left,
					destination_top,
					(tile_width * scale).round() as u32,
					(tile_height * scale).round() as u32,
				);
				*/

				// color-only overlays (no atlas sampling)
				if tile_kind.is_color_only() {
					self.draw_color_only_tile(tile_kind, destination);
					continue;
				}

				// normal atlas draw path (interactive / solid / regular tiles)
				let id: u32 = tile_id as u32;
				let source_left: i32 = ((id % tile_cols) * atlas_tile_width_pixels) as i32;
				let source_top: i32 = ((id / tile_cols) * atlas_tile_height_pixels) as i32;
				let source = Rect::new(source_left, source_top, atlas_tile_width_pixels, atlas_tile_height_pixels);
				let texture = self.tile_texture.as_mut().unwrap();
				let _ = self.canvas.copy(&texture, source, destination).unwrap();
			}
		}
		return;
	}

	fn draw_level_internal(&mut self, game_state: &GameState, game_session: &GameSession) {
		let (camera_left, camera_top) = self.common.compute_camera(self, game_state, game_session);
		let scale: f32 = self.get_render_scale();

		// background first, tiles on top
		self.draw_background(camera_left, camera_top, scale);

		let tile_cols: u32 = self.tile_texture.as_mut().expect("tile_texture does not have a value").query().width / self.atlas_tile_width_pixels;
		for layer in 0..(game_state.level.layer_count as u32) {
			self.draw_tiles_layer_atlas(&game_state.level, layer, camera_left as f32, camera_top as f32, scale, self.frame_index);
		}

		self.frame_index = self.frame_index.wrapping_add(1);
		self.draw_entities(game_state, game_session, tile_cols, camera_left as f32, camera_top as f32, scale, self.frame_index);
		self.draw_debug_triggers(game_state, game_session, camera_left as f32, camera_top as f32, scale);
		return;
	}

	fn draw_entities(&mut self, game_state: &GameState, game_session: &GameSession, tile_cols: u32, camera_left: f32, camera_top: f32, scale: f32, _frame_index: u32) {
		//let texture = self.tile_texture.as_mut().expect("tile_texture does not have a value");
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

			if entity_kind == EntityKind::SlimeBlue || entity_kind == EntityKind::SlimeUndead || entity_kind == EntityKind::SlimeLava {
				let death_timer: u16 = game_state.death_timers.get(id).copied().unwrap_or(0);
				if death_timer > 0 {
					self.draw_death_entity(game_state, game_session, entity_kind, pos, half_height, camera_left, camera_top, scale, death_timer);
					continue;
				}

				let vel: Vec2 = game_state.velocities.get(id).copied().unwrap_or_default();
				let abs_vx: f32 = vel.x.abs();

				let is_dying: bool = game_state.death_timers.get(id).copied().unwrap_or(0) > 0;

				let anim: LocomotionAnim = if is_dying {
					LocomotionAnim::Death
				} else if abs_vx >= 3.0 {
					LocomotionAnim::Run
				} else {
					LocomotionAnim::Walk
				};

				let frame_count: u32 = if anim == LocomotionAnim::Death { 10 } else { 8 };
				let frame_index: u32 = (_frame_index / 6) % frame_count;

				// row_index: keep what you were doing for walk/run. for death you probably want row 0.
				let row_index: u32 = if anim == LocomotionAnim::Death { 0 } else { 2 };

				let src_left_pixels: i32 = (frame_index as i32) * 64;
				let src_top_pixels: i32 = (row_index as i32) * 64;
				let src: Rect = Rect::new(src_left_pixels, src_top_pixels, 64, 64);

				let sprite_world_scale: f32 = game_state.enemy_sprite_scale as f32;
				let dest_width_pixels: u32 = (64.0 * sprite_world_scale * scale).round() as u32;
				let dest_height_pixels: u32 = (64.0 * sprite_world_scale * scale).round() as u32;

				// anchor point on physics body: bottom-center
				let entity_bottom_center_world_x: f32 = pos.x;
				let entity_bottom_center_world_y: f32 = pos.y + half_height;

				let entity_bottom_center_screen_left: i32 = ((entity_bottom_center_world_x - camera_left) * scale).round() as i32;
				let entity_bottom_center_screen_top: i32 = ((entity_bottom_center_world_y - camera_top) * scale).round() as i32;

				let anchor_left_frac: f32 = 32.0 / 64.0;
				let anchor_top_frac: f32 = 40.0 / 64.0;

				let sprite_feet_left_pixels: i32 = (dest_width_pixels as f32 * anchor_left_frac).round() as i32;
				let sprite_feet_top_pixels: i32 = (dest_height_pixels as f32 * anchor_top_frac).round() as i32;

				let dest_left_pixels: i32 = entity_bottom_center_screen_left - sprite_feet_left_pixels;
				let dest_top_pixels: i32 = entity_bottom_center_screen_top - sprite_feet_top_pixels;
				let dest: Rect = Rect::new(dest_left_pixels, dest_top_pixels, dest_width_pixels, dest_height_pixels);
				let flip_horizontal: bool = vel.x > 0.0;

				let key: SlimeTextureKey = self.get_slime_texture_key(entity_kind, anim);

				let _ = match key {
					SlimeTextureKey::BlueWalk => self.canvas.copy_ex(&self.slime_blue_walk_texture, src, dest, 0.0, None, flip_horizontal, false),
					SlimeTextureKey::BlueRun => self.canvas.copy_ex(&self.slime_blue_run_texture, src, dest, 0.0, None, flip_horizontal, false),
					SlimeTextureKey::BlueDeath => self.canvas.copy_ex(&self.slime_blue_death_texture, src, dest, 0.0, None, flip_horizontal, false),

					SlimeTextureKey::UndeadWalk => self.canvas.copy_ex(&self.slime_undead_walk_texture, src, dest, 0.0, None, flip_horizontal, false),
					SlimeTextureKey::UndeadRun => self.canvas.copy_ex(&self.slime_undead_run_texture, src, dest, 0.0, None, flip_horizontal, false),
					SlimeTextureKey::UndeadDeath => self.canvas.copy_ex(&self.slime_undead_death_texture, src, dest, 0.0, None, flip_horizontal, false),

					SlimeTextureKey::LavaWalk => self.canvas.copy_ex(&self.slime_lava_walk_texture, src, dest, 0.0, None, flip_horizontal, false),
					SlimeTextureKey::LavaRun => self.canvas.copy_ex(&self.slime_lava_run_texture, src, dest, 0.0, None, flip_horizontal, false),
					SlimeTextureKey::LavaDeath => self.canvas.copy_ex(&self.slime_lava_death_texture, src, dest, 0.0, None, flip_horizontal, false),
				}
				.unwrap();

				continue;
			}

			if entity_kind == EntityKind::MovingPlatform {
				let width_pixels: f32 = *game_state.widths.get(id).unwrap_or(&16) as f32;
				let tile_width: f32 = game_state.level.tile_width as f32;
				let width_tiles: i32 = ((width_pixels / tile_width).ceil() as i32).max(1);

				self.draw_platform_entity_tiles(
					tile_cols,
					self.atlas_tile_width_pixels,
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
				EntityKind::Empty => Color::RGB(0, 0, 0),
				EntityKind::Imp => Color::RGB(64, 200, 64),
				EntityKind::MovingPlatform => Color::RGB(255, 255, 0),
				EntityKind::SlimeBlue => Color::RGB(64, 160, 255),
				EntityKind::SlimeLava => Color::RGB(255, 0, 0),
				EntityKind::SlimeUndead => Color::RGB(255, 255, 255),
				EntityKind::Player => Color::RGB(255, 255, 255),
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
		// Nothing yet
	}

	fn get_render_scale(&self) -> f32 {
		return self.render_scale as f32;
	}

	fn new() -> Self {
		let sdl = sdl2::init().unwrap();
		let _ = sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0"); // nearest

		let _image = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
		let video = sdl.video().unwrap();
		let dm = video.desktop_display_mode(0).expect("desktop_display_mode failed");
		let desktop_width_pixels: u32 = dm.w as u32;
		let desktop_height_pixels: u32 = dm.h as u32;
		let target_aspect: f32 = 16.0 / 9.0;

		let saved: Option<WindowSettings> = load_window_settings();

		let (window_width_pixels, window_height_pixels) = if let Some(s) = saved {
			(s.width_pixels, s.height_pixels)
		} else {
			let mut window_height_pixels: u32 = ((desktop_height_pixels as f32) * 0.80) as u32;
			if window_height_pixels < 360 {
				window_height_pixels = 360;
			}

			let mut window_width_pixels: u32 = (window_height_pixels as f32 * target_aspect) as u32;
			if window_width_pixels > desktop_width_pixels {
				window_width_pixels = desktop_width_pixels;
				window_height_pixels = (window_width_pixels as f32 / target_aspect) as u32;
			}

			(window_width_pixels, window_height_pixels)
		};

		let mut window = video
			.window("jumpy", window_width_pixels, window_height_pixels)
			.position_centered()
			.resizable()
			.build()
			.unwrap();

		if let Some(s) = saved {
			window.set_position(sdl2::video::WindowPos::Positioned(s.left), sdl2::video::WindowPos::Positioned(s.top));

			if s.is_maximized {
				window.maximize();
			}
		}

		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
		let event_pump = sdl.event_pump().unwrap();

		let texture_creator = leak_texture_creator(&canvas);
		let tile_path = gfx_pc_path(&["tiles", "tiles64.png"]);
		let tile_texture = load_texture(&texture_creator, tile_path);

		// slimes
		let slime_blue_walk_path = gfx_pc_path(&["slime", "blue", "walk_body.png"]);
		let slime_blue_walk_tex = load_texture(&texture_creator, slime_blue_walk_path);

		let slime_blue_run_path = gfx_pc_path(&["slime", "blue", "run_body.png"]);
		let slime_blue_run_tex = load_texture(&texture_creator, slime_blue_run_path);

		let slime_undead_walk_path = gfx_pc_path(&["slime", "undead", "walk_body.png"]);
		let slime_undead_walk_tex = load_texture(&texture_creator, slime_undead_walk_path);

		let slime_undead_run_path = gfx_pc_path(&["slime", "undead", "run_body.png"]);
		let slime_undead_run_tex = load_texture(&texture_creator, slime_undead_run_path);

		let slime_lava_walk_path = gfx_pc_path(&["slime", "lava", "walk_body.png"]);
		let slime_lava_walk_tex = load_texture(&texture_creator, slime_lava_walk_path);

		let slime_lava_run_path = gfx_pc_path(&["slime", "lava", "run_body.png"]);
		let slime_lava_run_tex = load_texture(&texture_creator, slime_lava_run_path);

		let slime_blue_death_path: PathBuf = gfx_pc_path(&["slime", "blue", "death_body.png"]);
		//let slime_blue_death_texture = texture_creator.load_texture(slime_blue_death_path).expect("failed to load slime_blue_death.png");
		let slime_blue_death_texture = load_texture(&texture_creator, slime_blue_death_path);

		let slime_undead_death_path: PathBuf = gfx_pc_path(&["slime", "undead", "death_body.png"]);
		let slime_undead_death_texture = texture_creator.load_texture(slime_undead_death_path).expect("failed to load slime_undead_death.png");

		let slime_lava_death_path: PathBuf = gfx_pc_path(&["slime", "lava", "death_body.png"]);
		let slime_lava_death_texture = texture_creator.load_texture(slime_lava_death_path).expect("failed to load slime_lava_death.png");

		let ttf_ctx: Sdl2TtfContext = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
		let ttf: &'static Sdl2TtfContext = Box::leak(Box::new(ttf_ctx));

		let font_path = crate::assets::get_font_path("DejaVuSansMono.ttf");
		let book_font = ttf.load_font(font_path, 16).map_err(|e| e.to_string()).unwrap();

		return PcRenderer {
			video,
			canvas,
			event_pump,
			common: RenderCommon::new(),
			frame_index: 0,
			bg_texture: None,
			bg_parallax_x: 0.35,
			bg_parallax_y: 0.15,
			atlas_tile_width_pixels: 64,
			atlas_tile_height_pixels: 64,
			tile_texture: Some(tile_texture),
			render_scale: 4,
			slime_blue_run_texture: slime_blue_run_tex,
			slime_blue_walk_texture: slime_blue_walk_tex,
			slime_blue_death_texture: slime_blue_death_texture,
			slime_lava_walk_texture: slime_lava_walk_tex,
			slime_lava_run_texture: slime_lava_run_tex,
			slime_lava_death_texture: slime_lava_death_texture,
			slime_undead_run_texture: slime_undead_run_tex,
			slime_undead_walk_texture: slime_undead_walk_tex,
			slime_undead_death_texture: slime_undead_death_texture,
			texture_creator,
			bg_id: 0,
			book_font,
		};
	}

	fn screen_size(&self) -> (i32, i32) {
		let (w, h) = self.canvas.output_size().unwrap();
		return (w as i32, h as i32);
	}

	fn poll_input(&mut self) -> InputState {
		return crate::platform::input::pc::poll(&mut self.event_pump);
	}

	fn begin_frame(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	fn draw_level(&mut self, game_state: &GameState, game_session: &GameSession) {
		self.draw_level_internal(game_state, game_session);
	}

	fn draw_death_entity(
		&mut self,
		game_state: &GameState,
		game_session: &GameSession,
		entity_kind: EntityKind,
		pos: &Vec2,
		half_height: f32,
		camera_left: f32,
		camera_top: f32,
		scale: f32,
		death_timer: u16,
	) {
		// 64x64 frames in the death sheet
		let frame_size_pixels: i32 = 64;

		// how long death lasts total
		let total: u16 = game_session.settings.enemy_death_frame_count as u16;

		// how many frames are in the sheet (tune this once)
		let frame_count: u32 = game_session.settings.frame_count;

		// elapsed frames since death started
		let elapsed: u16 = total.saturating_sub(death_timer);

		// spread elapsed over frame_count
		let frame_index: u32 = ((elapsed as u32) * frame_count / (total as u32)).min(frame_count - 1);

		let row_index: u32 = 0;

		let src_left_pixels: i32 = (frame_index as i32) * frame_size_pixels;
		let src_top_pixels: i32 = (row_index as i32) * frame_size_pixels;
		let src = Rect::new(src_left_pixels, src_top_pixels, frame_size_pixels as u32, frame_size_pixels as u32);

		// let dest_width_pixels_u32: u32 = ((half_width * 2.0) * scale).round().max(1.0) as u32;
		// let dest_height_pixels_u32: u32 = ((half_height * 2.0) * scale).round().max(1.0) as u32;

		let sprite_world_scale: f32 = game_state.enemy_sprite_scale as f32;
		let dest_width_pixels_u32: u32 = (64.0 * sprite_world_scale * scale).round().max(1.0) as u32;
		let dest_height_pixels_u32: u32 = (64.0 * sprite_world_scale * scale).round().max(1.0) as u32;

		// physics anchor: bottom-center
		let entity_bottom_center_world_x: f32 = pos.x;
		let entity_bottom_center_world_y: f32 = pos.y + half_height;

		let entity_bottom_center_screen_left: i32 = ((entity_bottom_center_world_x - camera_left) * scale).round() as i32;
		let entity_bottom_center_screen_top: i32 = ((entity_bottom_center_world_y - camera_top) * scale).round() as i32;

		// tune these once for your death sheet
		let anchor_left_frac: f32 = 32.0 / 64.0;
		let anchor_top_frac: f32 = 40.0 / 64.0;

		let anchor_left_pixels: i32 = (dest_width_pixels_u32 as f32 * anchor_left_frac).round() as i32;
		let anchor_top_pixels: i32 = (dest_height_pixels_u32 as f32 * anchor_top_frac).round() as i32;

		let dest_left_pixels: i32 = entity_bottom_center_screen_left - anchor_left_pixels;
		let dest_top_pixels: i32 = entity_bottom_center_screen_top - anchor_top_pixels;

		let dest = Rect::new(dest_left_pixels, dest_top_pixels, dest_width_pixels_u32, dest_height_pixels_u32);

		// pick texture based on kind (or reuse one texture sheet)
		let tex = match entity_kind {
			EntityKind::SlimeBlue => &self.slime_blue_death_texture,
			EntityKind::SlimeUndead => &self.slime_undead_death_texture,
			EntityKind::SlimeLava => &self.slime_lava_death_texture,
			_ => &self.slime_blue_death_texture,
		};

		let _ = self.canvas.copy_ex(tex, src, dest, 0.0, None, false, false).unwrap();
	}

	fn commit(&mut self) {
		self.canvas.present();
	}
}

/*
fn leak_texture_creator(canvas: &sdl2::render::Canvas<sdl2::video::Window>) -> &'static sdl2::render::TextureCreator<sdl2::video::WindowContext> {
	let creator_box = Box::new(canvas.texture_creator());
	let texture_creator: &'static sdl2::render::TextureCreator<sdl2::video::WindowContext> = Box::leak(creator_box);
	return texture_creator;
}
*/

fn gfx_pc_path(parts: &[&str]) -> PathBuf {
	let mut path = get_gfx_root().join("pc");
	for p in parts {
		path = path.join(p);
	}
	return path;
}

fn load_texture<'a>(texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>, path: PathBuf) -> sdl2::render::Texture<'a> {
	let path_string = path.to_string_lossy().to_string();
	let texture = texture_creator
		.load_texture(&path)
		.unwrap_or_else(|_| panic!("failed to load texture: {}", path_string));
	return texture;
}

fn parse_background_id(background_id: u8) -> &'static str {
	match background_id {
		BACKGROUND_ID_LIBRARY_STONE => "bg_library_stone.png",
		BACKGROUND_PARALLAX_FOREST => "bg_parallax_forest.png",
		_ => panic!("Unknown id: {}", background_id),
	}
}

fn leak_texture_creator(canvas: &sdl2::render::Canvas<sdl2::video::Window>) -> &'static sdl2::render::TextureCreator<sdl2::video::WindowContext> {
	let creator_box = Box::new(canvas.texture_creator());
	let texture_creator: &'static sdl2::render::TextureCreator<sdl2::video::WindowContext> = Box::leak(creator_box);
	return texture_creator;
}
