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
	let level = match game::level::Level::load_binary("../levels/tutorial.lvlb") {
		Ok(l) => l,
		Err(e) => {
			eprintln!("level load failed: {}", e);
			return;
		}
	};

	let mut state = GameState::new(level);
	let player_id = state.spawn_player();

	if state.player_id.is_none() {
		eprintln!("failed to create player entity");
		return;
	}

	state.spawn_level_entities();

	let mut renderer = ActiveRenderer::new();

	renderer.init();

	loop {
		let input = renderer.poll_input();
		if input.quit {
			break;
		}

		// left/right + jump impulse
		if let Some(v) = state.velocities.get_mut(&state.get_player_id()) {
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
