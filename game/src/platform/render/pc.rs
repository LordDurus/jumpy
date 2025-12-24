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
}

/*
impl PcRenderer {
	fn poll(&mut self) -> InputState {
		let mut input: InputState = InputState::default();

		for event in self.event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => {
					input.quit = true;
				}
				Event::KeyDown {
					keycode: Some(Keycode::Left),
					repeat: false,
					..
				} => {
					input.left = true;
				}
				Event::KeyDown {
					keycode: Some(Keycode::Right),
					repeat: false,
					..
				} => {
					input.right = true;
				}
				Event::KeyDown {
					keycode: Some(Keycode::Space),
					repeat: false,
					..
				} => {
					input.jump = true;
				}
				_ => {}
			}
		}

		return input;
	}
}
	*/

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

				// world -> screen pixels
				let world_x: i32 = tx * tile_w_world;
				let world_y: i32 = ty * tile_h_world;

				let sx: i32 = (((world_x - cam_x_world) as f32) * scale).round() as i32;
				let sy: i32 = (((world_y - cam_y_world) as f32) * scale).round() as i32;

				let rect = Rect::new(sx, sy, tile_w_px, tile_h_px);
				let _ = self.canvas.fill_rect(rect);
			}
		}

		// entities (same rule: (world - cam) * scale)
		self.canvas.set_draw_color(Color::RGB(255, 255, 255));
		for (id, pos) in world.positions.iter() {
			let (half_w, half_h) = world.entity_half_extents(*id);

			let world_left: f32 = pos.x - half_w;
			let world_top: f32 = pos.y - half_h;

			let sx: i32 = (((world_left as i32 - cam_x_world) as f32) * scale).round() as i32;
			let sy: i32 = (((world_top as i32 - cam_y_world) as f32) * scale).round() as i32;

			let w: u32 = ((half_w * 2.0) * scale).round() as u32;
			let h: u32 = ((half_h * 2.0) * scale).round() as u32;

			let rect = Rect::new(sx, sy, w, h);
			let _ = self.canvas.fill_rect(rect);
		}
	}

	fn commit(&mut self) {
		self.canvas.present();
	}
}
