#![cfg(feature = "pc")]

use crate::platform::input::InputState;
use sdl2::{
	EventPump,
	event::Event,
	keyboard::{Keycode, Scancode},
};

pub fn poll(event_pump: &mut EventPump) -> InputState {
	// make sure keyboard_state is current even when there are no events this frame
	event_pump.pump_events();

	let mut input = InputState::default();

	for event in event_pump.poll_iter() {
		match event {
			Event::Quit { .. } => input.quit = true,
			Event::KeyDown { keycode: Some(Keycode::Q), .. } => input.quit = true,
			_ => {}
		}
	}

	let keys = event_pump.keyboard_state();
	let ctrl_down = keys.is_scancode_pressed(Scancode::LCtrl) || keys.is_scancode_pressed(Scancode::RCtrl);

	input.left = keys.is_scancode_pressed(Scancode::Left) || keys.is_scancode_pressed(Scancode::A);
	input.right = keys.is_scancode_pressed(Scancode::Right) || keys.is_scancode_pressed(Scancode::D);
	input.up = keys.is_scancode_pressed(Scancode::Up) || keys.is_scancode_pressed(Scancode::W);
	input.down = keys.is_scancode_pressed(Scancode::Down) || keys.is_scancode_pressed(Scancode::S);
	input.jump = keys.is_scancode_pressed(Scancode::Space);
	input.inventory = keys.is_scancode_pressed(Scancode::I);
	input.read = keys.is_scancode_pressed(Scancode::R);
	input.escape = keys.is_scancode_pressed(Scancode::Escape);
	input.page_up = keys.is_scancode_pressed(Scancode::PageUp);
	input.page_down = keys.is_scancode_pressed(Scancode::PageDown);
	input.copy = keys.is_scancode_pressed(Scancode::C) && ctrl_down;
	input.escape = keys.is_scancode_pressed(Scancode::Escape);

	return input;
}
