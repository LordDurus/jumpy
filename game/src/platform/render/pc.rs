const RENDER_SCALE: f32 = 1.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 360;
use crate::{
	game::game_state::GameState,
	platform::{RenderBackend, input::InputState, render::common::RenderCommon},
	tile::TileKind,
};
use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct PcRenderer {
	canvas: Canvas<Window>,
	event_pump: EventPump,
	common: RenderCommon,
	resized_from_world: bool,
}

impl RenderBackend for PcRenderer {
	fn screen_size(&self) -> (i32, i32) {
		let (w, h) = self.canvas.output_size().unwrap();
		return (w as i32, h as i32);
	}

	fn new() -> PcRenderer {
		let sdl = sdl2::init().unwrap();
		let video = sdl.video().unwrap();
		let window = video.window("jumpy", WINDOW_HEIGHT, WINDOW_WIDTH).position_centered().build().unwrap();
		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
		let event_pump = sdl.event_pump().unwrap();

		return PcRenderer {
			canvas,
			event_pump,
			common: RenderCommon::new(),
			resized_from_world: false,
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
		let mut input = InputState {
			quit: false,
			left: false,
			right: false,
			jump: false,
		};

		for event in self.event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => input.quit = true,
				Event::KeyDown {
					keycode: Some(Keycode::Escape), ..
				} => input.quit = true,
				_ => {}
			}
		}

		let keys = self.event_pump.keyboard_state();

		input.left = keys.is_scancode_pressed(sdl2::keyboard::Scancode::Left) || keys.is_scancode_pressed(sdl2::keyboard::Scancode::A);
		input.right = keys.is_scancode_pressed(sdl2::keyboard::Scancode::Right) || keys.is_scancode_pressed(sdl2::keyboard::Scancode::D);
		input.jump = keys.is_scancode_pressed(sdl2::keyboard::Scancode::Space);

		return input;
	}

	fn begin_frame(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	fn draw_world(&mut self, world: &GameState) {
		let level = &world.level;

		let mut cam_x: f32 = 0.0;
		let mut cam_y: f32 = 0.0;

		// follow player, not random hash map entry
		let player_id: u32 = world.get_player_id();
		if let Some(pos) = world.positions.get(&player_id) {
			let (screen_w, screen_h) = self.screen_size();
			cam_x = pos.x * self.render_scale() - (screen_w as f32) * 0.5;
			cam_y = pos.y * self.render_scale() - (screen_h as f32) * 0.5;
		}

		// ---- tiles ----
		let tile_w: f32 = level.tile_width as f32 * RENDER_SCALE;
		let tile_h: f32 = level.tile_height as f32 * RENDER_SCALE;

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

				let sx: i32 = (tx as f32 * tile_w - cam_x).round() as i32;
				let sy: i32 = (ty as f32 * tile_h - cam_y).round() as i32;

				let rect = Rect::new(sx, sy, tile_w.round() as u32, tile_h.round() as u32);

				if !self.resized_from_world {
					let scale = self.render_scale();
					let w = ((world.level.tile_width as f32) * (world.level.tile_width as f32) * scale).round() as u32;
					let h = ((world.level.tile_height as f32) * (world.level.tile_height as f32) * scale).round() as u32;

					let _ = self.canvas.window_mut().set_size(w, h);
					self.resized_from_world = true;
				}

				let _ = self.canvas.fill_rect(rect);
			}
		}

		// ---- entities ----
		self.canvas.set_draw_color(Color::RGB(255, 255, 255));
		for (id, pos) in world.positions.iter() {
			let (half_width, half_height) = world.entity_half_extents(*id);

			// convert entity rect to *scaled screen coords* and subtract camera
			let x: i32 = ((pos.x - half_width) * RENDER_SCALE - cam_x).round() as i32;
			let y: i32 = ((pos.y - half_height) * RENDER_SCALE - cam_y).round() as i32;

			let w: u32 = ((half_width * 2.0) * RENDER_SCALE).round() as u32;
			let h: u32 = ((half_height * 2.0) * RENDER_SCALE).round() as u32;

			let rect = Rect::new(x, y, w, h);
			let _ = self.canvas.fill_rect(rect);
		}
	}

	fn commit(&mut self) {
		self.canvas.present();
	}
}
