use crate::game::game_state::GameState;

#[inline(always)]
pub fn apply(game_state: &mut GameState) {
	for (_id, vel) in game_state.velocities.iter_mut() {
		vel.y += game_state.gravity;
	}
}
