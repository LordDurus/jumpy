use crate::vector2::Vector2;
use crate::world::World;

pub fn gravity_system(world: &mut World) {
    for velocity in world.velocities.values_mut() {
        velocity.add(&crate::vector2::GRAVITY);
    }
}
