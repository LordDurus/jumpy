use crate::{engine_math::Vec2, game::level::Level};
use std::collections::HashMap;

pub type EntityId = u32;

/// Represents the game world, containing entities and their properties (runtime state).
pub struct GameState {
	pub level: Level,
	pub gravity: f32,
	pub positions: HashMap<EntityId, Vec2>,
	pub velocities: HashMap<EntityId, Vec2>,
	next_entity_id: EntityId,
}

impl GameState {
	pub fn new(current_level: Level) -> GameState {
		return GameState {
			level: current_level,
			gravity: crate::physics::constants::WORLD_GRAVITY,
			positions: HashMap::new(),
			velocities: HashMap::new(),
			next_entity_id: 1,
		};
	}

	pub fn entity_half_extents(&self, _entity_id: EntityId) -> (f32, f32) {
		// TEMPORARY: default player/block size
		return (8.0, 8.0);
	}

	pub fn add_entity(&mut self, position: Vec2, velocity: Vec2) -> EntityId {
		let id = self.next_entity_id;
		self.next_entity_id += 1;

		self.positions.insert(id, position);
		self.velocities.insert(id, velocity);

		return id;
	}

	pub fn remove_entity(&mut self, id: EntityId) {
		self.positions.remove(&id);
		self.velocities.remove(&id);
	}
}
