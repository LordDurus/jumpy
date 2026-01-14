mod ai;
mod common;
mod ecs;
mod engine_math;
mod game;
mod physics;
mod platform;
mod tile;

use crate::{game::game_state::GameState, platform::render::backend::RenderBackend};

#[cfg(feature = "pc")]
type ActiveRenderer = crate::platform::render::pc::PcRenderer;

#[cfg(feature = "gba")]
type ActiveRenderer = crate::platform::render::gba::GbaRenderer;

#[cfg(feature = "psp")]
type ActiveRenderer = crate::platform::render::psp::PspRenderer;

#[cfg(feature = "pc")]
fn main() {
	let level = match game::level::Level::load_binary("../levels/debug.lvlb") {
		Ok(l) => l,
		Err(e) => {
			eprintln!("level load failed: {}", e);
			return;
		}
	};

	let mut state = GameState::new(level);

	state.spawn_level_entities();
	let player_id = state.get_player_id();

	let mut renderer = ActiveRenderer::new();

	renderer.init();

	loop {
		let input = renderer.poll_input();
		if input.quit {
			break;
		}

		// left/right
		let desired_x: f32 = if input.left && !input.right {
			-2.0
		} else if input.right && !input.left {
			2.0
		} else {
			0.0
		};

		if let Some(v) = state.velocities.get_mut(player_id) {
			v.set_x(desired_x);
		}

		// jump edge detection must run every frame
		let jump_down: bool = input.jump;

		let mut jump_pressed: bool = false;
		let mut jump_released: bool = false;

		if let Some(js) = state.jump_states.get_mut(player_id) {
			jump_pressed = jump_down && !js.jump_was_down;
			jump_released = !jump_down && js.jump_was_down;

			js.jump_was_down = jump_down;
		}

		// now no borrows are active, so this is fine
		if jump_pressed {
			physics::movement::try_jump(&mut state, player_id);
		}
		if jump_released {
			if let Some(vel) = state.velocities.get_mut(player_id) {
				// only cut jump if still going up
				if vel.y < 0.0 {
					vel.y *= state.settings.jump_cut_multiplier;
					// println!("short jump cut"); // keep temporarily if you want
				}
			}
		}

		state.tick = state.tick.wrapping_add(1);

		// ai::system::update(&mut state);
		physics::movement::patrol(&mut state);
		physics::gravity::apply(&mut state);
		physics::movement::move_and_collide(&mut state);

		renderer.begin_frame();
		renderer.draw_level(&state);
		renderer.commit();
	}
}

#[cfg(not(feature = "pc"))]
fn main() {
	unimplemented!();
}
