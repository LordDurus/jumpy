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

	#[inline(always)]
	pub fn on_ground(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(&id) else {
			return false;
		};

		let (half_w, half_h) = self.entity_half_extents(id);

		let inset: f32 = 0.5;

		let foot_y: f32 = pos.y + half_h + inset;
		let left_x: f32 = pos.x - half_w + inset;
		let right_x: f32 = pos.x + half_w - inset;

		let grounded: bool = self.level.is_solid_world_f32(left_x, foot_y) || self.level.is_solid_world_f32(right_x, foot_y);

		return grounded;
	}

	#[inline(always)]
	pub fn on_wall_left(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(&id) else {
			return false;
		};

		let (half_w, half_h) = self.entity_half_extents(id);

		let inset: f32 = 0.5;
		let probe_x: f32 = pos.x - half_w - inset;

		let y_top: f32 = pos.y - half_h + inset;
		let y_mid: f32 = pos.y;
		let y_bot: f32 = pos.y + half_h - inset;

		let hit: bool = self.level.is_solid_world_f32(probe_x, y_top) || self.level.is_solid_world_f32(probe_x, y_mid) || self.level.is_solid_world_f32(probe_x, y_bot);

		return hit;
	}

	#[inline(always)]
	pub fn on_wall_right(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(&id) else {
			return false;
		};

		let (half_w, half_h) = self.entity_half_extents(id);

		let inset: f32 = 0.5;
		let probe_x: f32 = pos.x + half_w + inset;

		let y_top: f32 = pos.y - half_h + inset;
		let y_mid: f32 = pos.y;
		let y_bot: f32 = pos.y + half_h - inset;

		let hit: bool = self.level.is_solid_world_f32(probe_x, y_top) || self.level.is_solid_world_f32(probe_x, y_mid) || self.level.is_solid_world_f32(probe_x, y_bot);

		return hit;
	}

	#[inline(always)]
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
