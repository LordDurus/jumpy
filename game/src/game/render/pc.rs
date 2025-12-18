use crate::game::world::World;
use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct PcInput {
	pub quit: bool,
	pub left: bool,
	pub right: bool,
	pub jump: bool,
}

pub struct PcRenderer {
	canvas: Canvas<Window>,
	event_pump: EventPump,
}

impl PcRenderer {
	pub fn new() -> PcRenderer {
		let sdl = sdl2::init().unwrap();
		let video = sdl.video().unwrap();

		let window = video.window("jumpy", 800, 600).position_centered().build().unwrap();

		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();

		let event_pump = sdl.event_pump().unwrap();

		return PcRenderer { canvas, event_pump };
	}

	pub fn init(&mut self) {
		// nothing special yet
	}

	pub fn poll_input(&mut self) -> PcInput {
		let mut input = PcInput {
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

	pub fn begin_frame(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	pub fn draw_world(&mut self, world: &World) {
		self.canvas.set_draw_color(Color::RGB(255, 255, 255));

		for (_id, pos) in world.positions.iter() {
			let x = pos.x as i32;
			let y = pos.y as i32;

			let rect = Rect::new(x, y, 16, 16);
			let _ = self.canvas.fill_rect(rect);
		}
	}

	pub fn commit(&mut self) {
		self.canvas.present();
	}
}
