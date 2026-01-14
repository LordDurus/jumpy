use crate::game::game_state::GameState;

#[inline(always)]
pub fn apply(game_state: &mut GameState) {
	for (id, vel) in game_state.velocities.iter_mut() {
		let grav: u8 = *game_state.gravity_multipliers.get(id).unwrap_or(&1);

		if grav == 0 {
			continue;
		}

		vel.y += game_state.settings.gravity * grav as f32;
	}
}
