use crate::vector2::Vector2;
use crate::world::World;

pub fn movement(world: &mut World) {
    for (id, position) in world.positions.iter_mut() {
        if let Some(velocity) = world.velocities.get(id) {
            position.add(velocity);
        }
    }
}
