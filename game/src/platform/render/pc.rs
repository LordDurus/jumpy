const RENDER_SCALE: f32 = 1.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 360;
use crate::{
	game::game_state::GameState,
	platform::{RenderBackend, input::InputState, render::common::RenderCommon},
	tile::TileKind,
};
use sdl2::{EventPump, pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct PcRenderer {
	canvas: Canvas<Window>,
	event_pump: EventPump,
	common: RenderCommon,
}

impl PcRenderer {
	fn draw_filled_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
		self.canvas.set_draw_color(color);
		let rect = Rect::new(x, y, w, h);
		let _ = self.canvas.fill_rect(rect);
		return;
	}

	fn draw_filled_circle(&mut self, cx: i32, cy: i32, r: i32, color: Color) {
		self.canvas.set_draw_color(color);

		let mut y: i32 = -r;
		while y <= r {
			let yy: i32 = y * y;
			let rr: i32 = r * r;
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

		let apex_x: i32 = x + ww / 2;
		let apex_y: i32 = y;

		let base_y: i32 = y + hh;
		let left_x: i32 = x;
		let right_x: i32 = x + ww;

		let mut row: i32 = 0;
		while row <= hh {
			let t: f32 = row as f32 / hh.max(1) as f32;

			// interpolate half-width from 0 at apex to ww/2 at base
			let half: i32 = ((ww as f32 * 0.5) * t).round() as i32;

			let y_row: i32 = apex_y + row;
			let x0: i32 = apex_x - half;
			let x1: i32 = apex_x + half;

			let _ = self.canvas.draw_line((x0, y_row), (x1, y_row));
			row += 1;
		}

		return;
	}
}

impl RenderBackend for PcRenderer {
	fn screen_size(&self) -> (i32, i32) {
		let (w, h) = self.canvas.output_size().unwrap();
		return (w as i32, h as i32);
	}

	fn new() -> PcRenderer {
		let sdl = sdl2::init().unwrap();
		let video = sdl.video().unwrap();
		let window = video.window("jumpy", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().build().unwrap();
		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
		let event_pump = sdl.event_pump().unwrap();

		return PcRenderer {
			canvas,
			event_pump,
			common: RenderCommon::new(),
		};
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

	fn init(&mut self) {
		// nothing special yet
	}

	fn poll_input(&mut self) -> InputState {
		return crate::platform::input::pc::poll(&mut self.event_pump);
	}

	fn begin_frame(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	fn draw_world(&mut self, world: &GameState) {
		let level = &world.level;

		let (cam_x_world, cam_y_world) = self.common.compute_camera(self, world);
		let scale: f32 = self.render_scale();

		let tile_w_world: i32 = level.tile_width as i32;
		let tile_h_world: i32 = level.tile_height as i32;

		let tile_w_px: u32 = (level.tile_width as f32 * scale).round() as u32;
		let tile_h_px: u32 = (level.tile_height as f32 * scale).round() as u32;

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

				let world_x: i32 = tx * tile_w_world;
				let world_y: i32 = ty * tile_h_world;

				let sx: i32 = (((world_x - cam_x_world) as f32) * scale).round() as i32;
				let sy: i32 = (((world_y - cam_y_world) as f32) * scale).round() as i32;

				let rect = Rect::new(sx, sy, tile_w_px, tile_h_px);
				let _ = self.canvas.fill_rect(rect);
			}
		}

		// entities (single pass: kind -> color, render_style -> shape)
		for (id, pos) in world.positions.iter() {
			let kind: u8 = *world.entity_kind.get(id).unwrap_or(&0);
			let style: u8 = *world.render_style.get(id).unwrap_or(&0);

			let (half_width, half_height) = world.entity_half_extents(*id);

			let world_left: f32 = pos.x - half_width;
			let world_top: f32 = pos.y - half_height;

			let scale_top: i32 = ((world_top - cam_y_world as f32) * scale).round() as i32;
			let scale_left: i32 = ((world_left - cam_x_world as f32) * scale).round() as i32;

			let width: u32 = ((half_width * 2.0) * scale).round() as u32;
			let height: u32 = ((half_height * 2.0) * scale).round() as u32;

			let color: Color = match kind {
				1 => Color::RGB(255, 255, 255), // player (white)
				2 => Color::RGB(64, 160, 255),  // slime (blue)
				3 => Color::RGB(64, 200, 64),   // imp (green)
				4 => Color::RGB(255, 255, 0),   // platform (yellow)
				_ => Color::RGB(255, 0, 255),   // debug
			};

			// println!("draw entity id={} kind={} style={} color={:?}", id, kind, style, color);

			match style {
				1 => {
					let cx: i32 = scale_left + (width as i32 / 2);
					let cy: i32 = scale_top + (height as i32 / 2);
					let r: i32 = (width.min(height) as i32) / 2;
					self.draw_filled_circle(cx, cy, r, color);
				}
				2 => {
					self.draw_filled_triangle(scale_left, scale_top, width, height, color);
				}
				_ => {
					self.draw_filled_rect(scale_left, scale_top, width, height, color);
				}
			}
		}

		return;
	}

	fn commit(&mut self) {
		self.canvas.present();
	}
}
