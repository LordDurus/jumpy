// use crate::platform::render::{InputState, Renderer};
// use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{
	game::game_state::GameState,
	platform::{input::InputState, render::Renderer},
	tile::TileKind,
};

use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct PcRenderer {
	canvas: Canvas<Window>,
	event_pump: EventPump,
}

impl Renderer for PcRenderer {
	fn new() -> PcRenderer {
		let sdl = sdl2::init().unwrap();
		let video = sdl.video().unwrap();
		let window = video.window("jumpy", 1030, 500).position_centered().build().unwrap();
		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
		let event_pump = sdl.event_pump().unwrap();

		return PcRenderer { canvas, event_pump };
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
		let tile_w: i32 = level.tile_width as i32;
		let tile_h: i32 = level.tile_height as i32;

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
					TileKind::Empty => self.canvas.set_draw_color(Color::RGB(0, 0, 0)),
				}

				let px: i32 = tx * tile_w;
				let py: i32 = ty * tile_h;
				let rect = Rect::new(px, py, level.tile_width, level.tile_height);
				let _ = self.canvas.fill_rect(rect);
			}
		}

		// ---- entities ----
		self.canvas.set_draw_color(Color::RGB(255, 255, 255));
		for (id, pos) in world.positions.iter() {
			let (half_width, half_height) = world.entity_half_extents(*id);

			let x: i32 = (pos.x - half_width).round() as i32;
			let y: i32 = (pos.y - half_height).round() as i32;

			let w: u32 = (half_width * 2.0).round() as u32;
			let h: u32 = (half_height * 2.0).round() as u32;

			let rect = Rect::new(x, y, w, h);
			let _ = self.canvas.fill_rect(rect);
		}
	}

	fn commit(&mut self) {
		self.canvas.present();
	}
}
