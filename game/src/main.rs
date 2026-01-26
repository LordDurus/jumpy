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
		input::TriggerPresses,
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

	let first_level_path: &str = "../worlds/00/01.lvlb";
	let bootstrap_level: Level = Level::load_binary(first_level_path).expect("failed to load first level");
	let mut state = GameState::new(bootstrap_level, audio);

	game_session.transition_to_level(&mut state, first_level_path);

	if state.settings.is_background_music_enabled {
		state.audio.play_music(MusicId::World1, true);
	}

	let mut renderer = ActiveRenderer::new();
	renderer.init();
	renderer.set_level_background(state.level.background_id);

	let mut up_was_down: bool = false;
	let mut down_was_down: bool = false;
	let mut left_was_down: bool = false;
	let mut right_was_down: bool = false;
	let mut action_was_down: bool = false; // "action" is jump for now

	loop {
		use crate::game::triggers;

		let input = renderer.poll_input();
		if input.quit {
			break;
		}

		// if triggers requested a level change last frame, do it now
		if let Some(next_level_name) = game_session.pending_level_name.take() {
			game_session.transition_to_level(&mut state, &next_level_name);
			renderer.set_level_background(state.level.background_id);
		}

		let Some(player_id) = state.try_get_player_id() else {
			// no player yet; still tick/render so you can see what's going on
			state.tick = state.tick.wrapping_add(1);
			renderer.begin_frame();
			renderer.draw_level(&state);
			renderer.commit();
			continue;
		};

		// left/right movement (held)
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

		// --- edge detection ---
		let jump_down: bool = input.jump;
		let mut jump_pressed: bool = false;
		let mut jump_released: bool = false;

		if let Some(js) = state.jump_states.get_mut(player_id) {
			jump_pressed = jump_down && !js.jump_was_down;
			jump_released = !jump_down && js.jump_was_down;
			js.jump_was_down = jump_down;
		}

		// for now, "action" == jump button
		let presses = TriggerPresses {
			action_pressed: input.jump && !action_was_down, // or a dedicated action button later
			up_pressed: input.up && !up_was_down,
			down_pressed: input.down && !down_was_down,
			left_pressed: input.left && !left_was_down,
			right_pressed: input.right && !right_was_down,
		};

		/*
		println!(
			"action_pressed={} up_pressed={} down_pressed={} left_pressed={} right_pressed={}",
			presses.action_pressed, presses.up_pressed, presses.down_pressed, presses.left_pressed, presses.right_pressed
		);
		*/

		action_was_down = input.jump;
		up_was_down = input.up;
		down_was_down = input.down;
		left_was_down = input.left;
		right_was_down = input.right;

		// --- triggers run before gameplay consumes jump ---
		let mut jump_consumed_by_triggers: bool = false;

		if triggers::handle_message_triggers(&mut state, presses) {
			jump_consumed_by_triggers = true;
		}

		triggers::handle_level_exit_triggers(&mut game_session, &mut state, presses);

		// --- gameplay jump logic (only if not consumed) ---
		if jump_pressed && !jump_consumed_by_triggers {
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
