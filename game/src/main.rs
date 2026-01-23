mod ai;
mod assets;
mod common;
mod ecs;
mod engine_math;
mod game;
mod physics;
mod platform;
mod tile;

use crate::{
	game::{game_session::GameSession, game_state::GameState, level::Level},
	platform::{
		audio::{AudioEngine, backend::MusicId},
		render::backend::RenderBackend,
	},
};

#[cfg(feature = "pc")]
use crate::platform::audio::pc::PcAudio;

#[cfg(feature = "pc")]
type ActiveRenderer = crate::platform::render::pc::PcRenderer;

#[cfg(feature = "gba")]
type ActiveRenderer = crate::platform::render::gba::GbaRenderer;

#[cfg(feature = "psp")]
type ActiveRenderer = crate::platform::render::psp::PspRenderer;

#[cfg(feature = "pc")]
fn main() {
	let mut game_session = GameSession::new();

	let audio: Box<dyn AudioEngine> = {
		let mut a = PcAudio::new();
		a.init();
		Box::new(a)
	};

	// bootstrap: create a state so transition_to_level can steal audio.
	// yes, this loads once here and once inside transition_to_level. it's fine for now.
	let first_level_path: &str = "../worlds/00/01.lvlb";
	let bootstrap_level: Level = Level::load_binary(first_level_path).expect("failed to load first level");
	let mut state = GameState::new(bootstrap_level, audio);

	// now do the real startup through the normal level transition path
	game_session.transition_to_level(&mut state, &first_level_path);

	if state.settings.is_background_music_enabled {
		state.audio.play_music(MusicId::World1, true);
	}

	let mut renderer = ActiveRenderer::new();
	renderer.init();

	loop {
		use crate::game::triggers;

		let input = renderer.poll_input();
		if input.quit {
			break;
		}

		// if triggers requested a level change last frame, do it now
		if let Some(next_level_name) = game_session.pending_level_name.take() {
			game_session.transition_to_level(&mut state, &next_level_name);
		}

		let player_id = state.get_player_id();

		// left/right
		let desired_x: f32 = if input.left && !input.right {
			-2.0
		} else if input.right && !input.left {
			2.0
		} else {
			0.0
		};

		if let Some(velocity) = state.velocities.get_mut(player_id) {
			velocity.set_x(desired_x);
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

		// triggers should run before jump is consumed by gameplay
		// message triggers can consume jump_pressed
		if triggers::handle_message_triggers(&mut state, jump_pressed) {
			jump_pressed = false;
		}

		// (when you wire it) level exit triggers should set:
		// game_session.pending_level_name = Some(...)
		triggers::handle_level_exit_triggers(&mut game_session, &mut state, jump_pressed);

		if jump_pressed {
			if let Some(jump_state) = state.jump_states.get_mut(player_id) {
				jump_state.jump_buffer_frames_left = state.settings.jump_buffer_frames_max;
			}
		}

		if jump_released {
			if let Some(velocity) = state.velocities.get_mut(player_id) {
				if velocity.y < 0.0 {
					velocity.y *= state.settings.jump_cut_multiplier;
				}
			}
			if let Some(jump_state) = state.jump_states.get_mut(player_id) {
				jump_state.jump_buffer_frames_left = 0;
			}
		}

		state.tick = state.tick.wrapping_add(1);

		physics::movement::patrol(&mut state);
		physics::gravity::apply(&mut state);
		physics::movement::move_and_collide(&mut state);

		state.tick_enemy_deaths();

		renderer.begin_frame();
		renderer.draw_level(&state);
		renderer.commit();
	}
}

#[cfg(not(feature = "pc"))]
fn main() {
	unimplemented!();
}
