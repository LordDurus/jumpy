use crate::{game::game_state::GameState, physics::movement::try_jump};

pub struct InputHandler {
	pub is_jumping: bool,
}

impl InputHandler {
	pub fn new() -> Self {
		Self { is_jumping: false }
	}

	pub fn handle_input(&mut self, world: &mut GameState, entity_id: u32) {
		// PC Example: Detect spacebar for jump
		#[cfg(feature = "pc")]
		if check_jump_input_pc() && !self.is_jumping {
			try_jump(world, entity_id);
			self.is_jumping = true;
		}

		// GBA Example: Detect button A for jump
		#[cfg(feature = "gba")]
		if check_jump_input_gba() && !self.is_jumping {
			try_jump(world, entity_id);
			self.is_jumping = true;
		}

		// Reset jump state when the entity is grounded
		if let Some(position) = world.positions.get(&entity_id) {
			if position.y() >= 0.0 {
				self.is_jumping = false;
			}
		}
	}
}

// PC: Check for spacebar input
#[cfg(feature = "pc")]
fn check_jump_input_pc() -> bool {
	// Replace with actual PC input handling, e.g., SDL2
	false
}

// GBA: Check for button A input
#[cfg(feature = "gba")]
fn check_jump_input_gba() -> bool {
	// Replace with actual GBA input handling
	false
}
