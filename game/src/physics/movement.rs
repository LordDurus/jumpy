use crate::game::world::World;

pub fn apply(world: &mut World) {
	for (id, pos) in world.positions.iter_mut() {
		if let Some(vel) = world.velocities.get(id) {
			pos.add(vel);
		}
	}
}
