use crate::{
	game::world::{EntityId, World},
	physics::constants::JUMP_VELOCITY,
};

pub fn apply(world: &mut World, entity_id: EntityId) {
	match world.velocities.get_mut(&entity_id) {
		Some(vel) => {
			vel.set_y(JUMP_VELOCITY);
		}
		None => {}
	}
}
