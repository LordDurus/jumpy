use crate::game::game_state::GameState;

#[inline(always)]
pub fn apply(world: &mut GameState) {
	for (_id, vel) in world.velocities.iter_mut() {
		vel.y += world.gravity;
	}
}
