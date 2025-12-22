#![cfg(feature = "pc")]

use crate::platform::input::InputState;
use sdl2::{
	EventPump,
	event::Event,
	keyboard::{Keycode, Scancode},
};

pub fn poll(event_pump: &mut EventPump) -> InputState {
	let mut input = InputState::default();

	for event in event_pump.poll_iter() {
		match event {
			Event::Quit { .. } => input.quit = true,
			Event::KeyDown {
				keycode: Some(Keycode::Escape), ..
			} => input.quit = true,
			_ => {}
		}
	}

	let keys = event_pump.keyboard_state();
	input.left = keys.is_scancode_pressed(Scancode::Left) || keys.is_scancode_pressed(Scancode::A);
	input.right = keys.is_scancode_pressed(Scancode::Right) || keys.is_scancode_pressed(Scancode::D);
	input.jump = keys.is_scancode_pressed(Scancode::Space);

	return input;
}
