use crate::{
	Session,
	common::coords::{PixelSize, Pointf32, Size, clamp_camera_to_level_world, get_screen, visible_tile_bounds},
	debugln,
	engine_math::Vec2,
	platform::{
		audio::backend::LocomotionAnim,
		render::{
			backend::RenderBackend,
			icon_registry::{ICON_FRAME_HEIGHT_PIXELS, ICON_FRAME_WIDTH_PIXELS, get_icon_src_rect_pixels, resolve_icon},
		},
	},
	runtime::{
		level::Level,
		state::{EntityKind, State},
		triggers::TriggerKind,
	},
	tile::TileKind,
};
use sdl2::render::{BlendMode, Texture};

use sdl2::{pixels::Color, rect::Rect};

use super::{PcRenderer, renderer::SlimeTextureKey};

impl PcRenderer {
	// pasted from your original pc.rs starting at fn draw_debug_triggers(...)
	// through the end of that impl block.
	//
	// NOTE: keep your existing method bodies exactly as-is here.
	//
	// this file should contain:
	// - draw_debug_triggers
	// - get_slime_texture_key
	// - draw_filled_rect / circle / triangle helpers
	// - draw_background
	// - draw_tiles_layer_atlas
	// - draw_level_internal
	// - draw_trigger_icons
	// - draw_entities
	// - draw_death_entity_internal (or whatever name you had)

	fn draw_debug_triggers(&mut self, state: &State, session: &Session, cam_left_world: f32, cam_top_world: f32, scale: f32) {
		if !session.settings.show_triggers {
			return;
		}

		use sdl2::{pixels::Color, rect::Rect};

		let tile_width_world: f32 = state.level.tile_width as f32;
		let tile_height_world: f32 = state.level.tile_height as f32;

		for t in &state.level.triggers {
			let idx: usize = t.id as usize;
			if idx < state.triggers_armed.len() && state.triggers_armed[idx] {
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

	pub fn draw_level_internal(&mut self, state: &State, session: &Session) {
		let (camera_left, camera_top) = self.common.compute_camera(self, state, session);
		let scale: f32 = self.get_render_scale();

		// background first, tiles on top
		self.draw_background(camera_left, camera_top, scale);

		let tile_cols: u32 = self.tile_texture.as_mut().expect("tile_texture does not have a value").query().width / self.atlas_tile_width_pixels;
		for layer in 0..(state.level.layer_count as u32) {
			self.draw_tiles_layer_atlas(&state.level, layer, camera_left as f32, camera_top as f32, scale, self.frame_index);
		}

		self.frame_index = self.frame_index.wrapping_add(1);
		self.draw_entities(state, session, tile_cols, camera_left as f32, camera_top as f32, scale, self.frame_index);
		self.draw_debug_triggers(state, session, camera_left as f32, camera_top as f32, scale);
		self.draw_trigger_icons(state, session, camera_left as f32, camera_top as f32, scale);
		return;
	}

	pub fn draw_death_entity_internal(
		&mut self,
		state: &State,
		session: &Session,
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
		let total: u16 = session.settings.enemy_death_frame_count as u16;

		// how many frames are in the sheet (tune this once)
		let frame_count: u32 = session.settings.frame_count;

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

		let sprite_world_scale: f32 = state.enemy_sprite_scale as f32;
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

	fn draw_trigger_icons(&mut self, state: &State, _session: &Session, cam_left_world: f32, cam_top_world: f32, scale: f32) {
		let atlas: &Texture<'static> = &self.trigger_texture;

		let tile_width: f32 = state.level.tile_width as f32;
		let tile_height: f32 = state.level.tile_height as f32;

		let (screen_width_pixels, screen_height_pixels) = self.screen_size();
		let view_width_world: f32 = (screen_width_pixels as f32) / scale;
		let view_height_world: f32 = (screen_height_pixels as f32) / scale;

		let view_left_world: f32 = cam_left_world;
		let view_top_world: f32 = cam_top_world;
		let view_right_world: f32 = view_left_world + view_width_world;
		let view_bottom_world: f32 = view_top_world + view_height_world;

		let padding_world: f32 = 16.0;

		for trigger in &state.level.triggers {
			if trigger.icon_id == 0 {
				continue;
			}

			let trigger_id: usize = trigger.id as usize;
			if trigger_id < state.triggers_armed.len() && state.triggers_armed[trigger_id] {
				continue; // consumed -> don't draw
			}

			let Some(def) = resolve_icon(trigger.icon_id) else {
				continue;
			};

			let trigger_left_world: f32 = (trigger.left_tiles as f32) * tile_width;
			let trigger_top_world: f32 = (trigger.top_tiles as f32) * tile_height;
			let trigger_width_world: f32 = (trigger.width_tiles as f32) * tile_width;
			let trigger_height_world: f32 = (trigger.height_tiles as f32) * tile_height;

			let trigger_right_world: f32 = trigger_left_world + trigger_width_world;
			let trigger_bottom_world: f32 = trigger_top_world + trigger_height_world;

			let visible: bool = trigger_right_world >= view_left_world - padding_world
				&& trigger_left_world <= view_right_world + padding_world
				&& trigger_bottom_world >= view_top_world - padding_world
				&& trigger_top_world <= view_bottom_world + padding_world;

			if !visible {
				continue;
			}

			let trigger_center_left_world: f32 = trigger_left_world + (trigger_width_world * 0.5);

			let half_icon_height_world: f32 = (ICON_FRAME_HEIGHT_PIXELS as f32) / scale * 0.5;
			let bottom_padding_world: f32 = 2.0 / scale; // 2 pixels padding

			let icon_world_left: f32 = trigger_center_left_world;
			let icon_world_top: f32 = trigger_bottom_world - half_icon_height_world - bottom_padding_world;

			let frame_index: u16 = if def.frame_count <= 1 || def.frame_duration_ticks == 0 {
				0
			} else {
				let frame_u32: u32 = (self.frame_index as u32 / def.frame_duration_ticks as u32) % def.frame_count as u32;
				frame_u32 as u16
			};

			let (src_left_pixels, src_top_pixels, src_width_pixels, src_height_pixels) = get_icon_src_rect_pixels(trigger.icon_id, frame_index);

			let src = sdl2::rect::Rect::new(src_left_pixels, src_top_pixels, src_width_pixels, src_height_pixels);

			// screen-space: keep icon size constant (32x32), don’t multiply by world scale
			let screen_left: i32 = ((icon_world_left - cam_left_world) * scale) as i32 - (ICON_FRAME_WIDTH_PIXELS as i32 / 2);
			let screen_top: i32 = ((icon_world_top - cam_top_world) * scale) as i32 - (ICON_FRAME_HEIGHT_PIXELS as i32 / 2);

			let dest = sdl2::rect::Rect::new(screen_left, screen_top, ICON_FRAME_WIDTH_PIXELS, ICON_FRAME_HEIGHT_PIXELS);

			let _ = self.canvas.copy(atlas, Some(src), dest);
		}
	}

	fn draw_entities(&mut self, state: &State, session: &Session, tile_cols: u32, camera_left: f32, camera_top: f32, scale: f32, _frame_index: u32) {
		//let texture = self.tile_texture.as_mut().expect("tile_texture does not have a value");
		for (id, pos) in state.positions.iter() {
			let kind = *state.entity_kinds.get(id).unwrap_or(&0);
			let entity_kind = EntityKind::from_u8(kind);

			if entity_kind == EntityKind::Empty {
				debugln!("Warning: entity id {} has unknown kind {}", id, kind);
				continue;
			}

			let style: u8 = *state.render_styles.get(id).unwrap_or(&0);
			let (half_width, half_height) = state.get_entity_half_values(id);
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
				let death_timer: u16 = state.death_timers.get(id).copied().unwrap_or(0);
				if death_timer > 0 {
					self.draw_death_entity(state, session, entity_kind, pos, half_height, camera_left, camera_top, scale, death_timer);
					continue;
				}

				let vel: Vec2 = state.velocities.get(id).copied().unwrap_or_default();
				let abs_vx: f32 = vel.x.abs();

				let is_dying: bool = state.death_timers.get(id).copied().unwrap_or(0) > 0;

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

				let sprite_world_scale: f32 = state.enemy_sprite_scale as f32;
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
				let width_pixels: f32 = *state.widths.get(id).unwrap_or(&16) as f32;
				let tile_width: f32 = state.level.tile_width as f32;
				let width_tiles: i32 = ((width_pixels / tile_width).ceil() as i32).max(1);

				self.draw_platform_entity_tiles(
					tile_cols,
					self.atlas_tile_width_pixels,
					world_left,
					world_top,
					width_tiles,
					&state.level,
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
