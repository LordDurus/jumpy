mod engine_math;
mod game;
mod physics;

use crate::{
	engine_math::Vec2,
	game::{render::pc::PcRenderer, world::World},
};

#[cfg(feature = "pc")]
fn main() {
	let mut world = World::new();
	let player_id = world.add_entity(Vec2::new(100.0, 100.0), Vec2::zero());

	let mut renderer = PcRenderer::new();
	renderer.init();

	loop {
		let input = renderer.poll_input();
		if input.quit {
			break;
		}

		// left/right + jump impulse
		if let Some(v) = world.velocities.get_mut(&player_id) {
			if input.left && !input.right {
				v.set_x(-2.0);
			} else if input.right && !input.left {
				v.set_x(2.0);
			} else {
				v.set_x(0.0);
			}

			if input.jump {
				physics::jump::apply(&mut world, player_id);
			}
		}

		physics::gravity::apply(&mut world);
		physics::movement::apply(&mut world);

		renderer.begin_frame();
		renderer.draw_world(&world);
		renderer.commit();
	}
}

#[cfg(not(feature = "pc"))]
fn main() {
	unimplemented!();
}
