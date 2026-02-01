use crate::runtime::{session::Session, state::State};

#[inline(always)]
pub fn apply(state: &mut State, session: &Session) {
	for (id, vel) in state.velocities.iter_mut() {
		let grav: u8 = *state.gravity_multipliers.get(id).unwrap_or(&1);

		if grav == 0 {
			continue;
		}

		vel.y += session.settings.gravity * grav as f32;
	}
}
