mod engine_math;
mod game;
mod physics;
mod tile;

use crate::{
	engine_math::Vec2,
	game::{game_state::GameState, render::pc::PcRenderer},
};

#[cfg(feature = "pc")]
fn main() {
	let level = match game::level::Level::load_binary("../levels/sample.LLB") {
		Ok(l) => l,
		Err(e) => {
			eprintln!("level load failed: {}", e);
			return;
		}
	};

	let mut state = GameState::new(level);

	let player_id = state.add_entity(Vec2::new(100.0, 100.0), Vec2::zero());

	let mut renderer = PcRenderer::new();
	renderer.init();

	loop {
		let input = renderer.poll_input();
		if input.quit {
			break;
		}

		// left/right + jump impulse
		if let Some(v) = state.velocities.get_mut(&player_id) {
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
		renderer.draw_world(&state);
		renderer.commit();
	}
}

#[cfg(not(feature = "pc"))]
fn main() {
	unimplemented!();
}
