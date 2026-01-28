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
	game::{
		book::{BookId, BookSlug},
		game_session::GameSession,
		game_state::GameState,
		level::Level,
	},
	platform::{
		audio::{AudioEngine, backend::MusicId},
		input::TriggerPresses,
		render::backend::RenderBackend,
	},
};

#[cfg(feature = "pc")]
use crate::game::book::reader_pc;

#[cfg(feature = "pc")]
use crate::platform::audio::pc::PcAudio;

#[cfg(feature = "pc")]
type ActiveRenderer = crate::platform::render::pc::PcRenderer;

#[cfg(feature = "pc")]
pub type ActiveBookTextSource = reader_pc::PcBookTextSource;

#[cfg(feature = "gba")]
pub type ActiveBookTextSource = book_reader_gba::GbaBookTextSource;

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

	// game_session.transition_to_level(&mut state, first_level_path);

	state.spawn_level_entities();
	state.apply_player_from_persistent(&game_session);
	game_session.current_level_name = Some(first_level_path.to_string());

	if game_session.settings.is_background_music_enabled {
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
	let mut inventory_was_down: bool = false;
	let mut read_was_down: bool = false;
	let mut page_up_was_down: bool = false;
	let mut page_down_was_down: bool = false;
	let mut copy_was_down: bool = false;
	let mut escape_was_down: bool = false;

	loop {
		use crate::game::triggers;

		let input: platform::input::InputState = renderer.poll_input();
		if input.quit {
			break;
		}

		let inventory_pressed: bool = input.inventory && !inventory_was_down;
		let read_pressed: bool = input.read && !read_was_down;

		if inventory_pressed {
			inventory_was_down = true;
			print_inventory(&game_session.inventory);
			continue;
		}

		if game_session.book_reading.is_open {
			if input.quit {
				break;
			}

			if !copy_was_down && input.copy {
				renderer.copy_book_page_to_clipboard(&game_session.book_reading.page_text);
			}

			if !escape_was_down && input.escape {
				game_session.book_reader.close_book(&mut game_session.book_reading);
			}

			if (!left_was_down && input.left) || (!page_up_was_down && input.page_up) {
				let _ = game_session.book_reader.turn_book_page(&mut game_session.book_reading, -1);
			}

			if (!right_was_down && input.right) || (!page_down_was_down && input.page_down) {
				let _ = game_session.book_reader.turn_book_page(&mut game_session.book_reading, 1);
			}

			right_was_down = input.right;
			left_was_down = input.left;
			page_down_was_down = input.page_down;
			page_up_was_down = input.page_up;
			copy_was_down = input.copy;
			escape_was_down = input.escape;

			renderer.begin_frame();
			renderer.draw_level(&state, &game_session);

			renderer.draw_book_overlay(&game_session);
			renderer.commit();
			continue;
		}

		if read_pressed {
			read_was_down = true;

			let book_id: BookId = 100;
			let book_slug: BookSlug = "tom_sawyer";

			let Some(_) = game_session.inventory.get_book(book_id) else {
				println!("tom_sawyer not in inventory");
				continue;
			};

			// open the ui at page 0 (or the saved page later)
			let result = game_session.book_reader.open_book(&mut game_session.book_reading, book_slug, 0);
			if let Err(e) = result {
				println!("open book failed: {}", e);
			}
			continue;
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
			renderer.draw_level(&state, &game_session);
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

		action_was_down = input.jump;
		up_was_down = input.up;
		down_was_down = input.down;
		left_was_down = input.left;
		right_was_down = input.right;
		inventory_was_down = input.inventory;
		read_was_down = input.read;
		page_down_was_down = input.page_down;
		page_up_was_down = input.page_up;
		copy_was_down = input.copy;

		// --- triggers run before gameplay consumes jump ---
		let mut jump_consumed_by_triggers: bool = false;

		if triggers::handle_message_triggers(&game_session, &mut state, presses) {
			jump_consumed_by_triggers = true;
		}

		triggers::handle_level_exit_triggers(&mut game_session, &mut state, presses);

		if triggers::handle_pickup_triggers(&mut game_session, &mut state, presses) {
			jump_consumed_by_triggers = true;
		}

		// --- gameplay jump logic (only if not consumed) ---
		if jump_pressed && !jump_consumed_by_triggers {
			if let Some(jump_state) = state.jump_states.get_mut(player_id) {
				jump_state.jump_buffer_frames_left = game_session.settings.jump_buffer_frames_max;
			}
		}

		if jump_released {
			if let Some(velocity) = state.velocities.get_mut(player_id) {
				if velocity.y < 0.0 {
					velocity.y *= game_session.settings.jump_cut_multiplier;
				}
			}
			if let Some(jump_state) = state.jump_states.get_mut(player_id) {
				jump_state.jump_buffer_frames_left = 0;
			}
		}

		state.tick = state.tick.wrapping_add(1);

		physics::movement::patrol(&mut state);
		physics::gravity::apply(&mut state, &game_session);
		physics::movement::move_and_collide(&mut state, &game_session);

		state.tick_enemy_deaths();

		renderer.begin_frame();
		renderer.draw_level(&state, &game_session);
		renderer.commit();
	}
}

fn print_inventory(inv: &crate::game::inventory::Inventory) {
	println!("--- inventory ---");
	println!("coins={}", inv.coins);

	println!("keys={}", inv.keys.len());
	for k in &inv.keys {
		println!(" key id={} used={}", k.key_id, k.is_used);
	}

	println!("books={}", inv.books.len());
	for b in &inv.books {
		println!(" book id={} page={}/{}", b.book_id, b.current_page, b.total_pages);
	}

	return;
}
