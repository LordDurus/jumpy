use crate::vector2::Vector2;
use crate::world::World;

pub const JUMP_VELOCITY: f32 = -15.0; // Adjust based on desired jump height

pub fn jump(world: &mut World, entity_id: u32) {
    if let Some(velocity) = world.velocities.get_mut(&entity_id) {
        velocity.set_y(JUMP_VELOCITY); // Apply upward velocity
    }
}
