mod ai;
mod common;
mod ecs;
mod engine_math;
mod game;
mod physics;
mod platform;
mod tile;

use crate::{engine_math::Vec2, game::game_state::GameState, platform::render::backend::RenderBackend};

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

		// left/right + jump impulse
		if let Some(v) = state.velocities.get_mut(player_id) {
			if input.left && !input.right {
				v.set_x(-2.0);
			} else if input.right && !input.left {
				v.set_x(2.0);
			} else {
				v.set_x(0.0);
			}

			if input.jump {
				physics::movement::try_jump(&mut state, player_id);
			}
		}

		state.tick = state.tick.wrapping_add(1);

		ai::system::update(&mut state);
		physics::gravity::apply(&mut state);
		physics::movement::move_and_collide(&mut state);

		renderer.begin_frame();
		renderer.draw_level(&state);
		renderer.commit();

		/*
		// debug logging
		if (renderer.frame_index % 60) == 0 {
			for (id, pos) in state.positions.iter() {
				let kind: u8 = *state.entity_kinds.get(id).unwrap_or(&0);
				if kind == 2 {
					// slime
					let vel = state.velocities.get(id).copied().unwrap_or(Vec2::zero());
					println!("slime id={} pos=({}, {}) vel=({}, {})", id, pos.x, pos.y, vel.x, vel.y);
				}
			}
		}
		*/
	}
}

#[cfg(not(feature = "pc"))]
fn main() {
	unimplemented!();
}
