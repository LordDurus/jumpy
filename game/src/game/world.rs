use crate::engine_math::Vec2;
use std::collections::HashMap;

pub type EntityId = u32;

pub struct World {
	pub gravity: f32,
	pub positions: HashMap<EntityId, Vec2>,
	pub velocities: HashMap<EntityId, Vec2>,
	next_entity_id: EntityId,
}

impl World {
	pub fn new() -> World {
		return World {
			gravity: crate::physics::constants::WORLD_GRAVITY,
			positions: HashMap::new(),
			velocities: HashMap::new(),
			next_entity_id: 1,
		};
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
