use crate::{game::world::World, physics::constants::WORLD_GRAVITY};

#[inline(always)]
pub fn apply(world: &mut World) {
	for (_id, vel) in world.velocities.iter_mut() {
		vel.y += world.gravity;
	}
}
