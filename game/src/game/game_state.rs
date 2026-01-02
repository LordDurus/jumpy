use crate::{ecs::component_store::ComponentStore, engine_math::Vec2, game::level::Level, physics::collision};
use std::collections::HashMap;

pub type EntityId = u32;

#[repr(u8)]
#[allow(dead_code)]
pub enum EntityKind {
	Player = 1,
	Slime = 2,
	Imp = 3,
	MovingPlatform = 4,
}

impl EntityKind {
	#[inline(always)]
	pub fn is_enemy(kind: u8) -> bool {
		kind == EntityKind::Slime as u8 || kind == EntityKind::Imp as u8
	}
}

/// Represents the game world, containing entities and their properties (runtime state).
pub struct GameState {
	pub level: Level,
	pub gravity: f32,
	pub positions: HashMap<EntityId, Vec2>,
	pub velocities: HashMap<EntityId, Vec2>,
	pub player_id: Option<EntityId>,
	pub spawn_point: Vec2,
	pub last_grounded_pos: Option<Vec2>,

	pub entity_kinds: HashMap<EntityId, u8>,
	pub render_styles: HashMap<EntityId, u8>,
	pub widths: HashMap<EntityId, u8>,
	pub heights: HashMap<EntityId, u8>,
	pub speeds: HashMap<EntityId, u8>,
	pub strengths: HashMap<EntityId, u8>,
	pub luck: HashMap<EntityId, u8>,

	pub gravity_multipliers: ComponentStore<u8>,
	pub range_mins: ComponentStore<f32>,
	pub range_maxes: ComponentStore<f32>,
	pub jump_multipliers: ComponentStore<u8>,

	// pub enemy_ids: ComponentStore<EntityId>,
	pub enemy_ids: Vec<EntityId>,

	next_entity_id: EntityId,
}

impl GameState {
	pub fn new(current_level: Level) -> GameState {
		let spawn_top = current_level.player_spawn_top;
		let spawn_left = current_level.player_spawn_left;

		let mut state = GameState {
			level: current_level,
			gravity: crate::physics::constants::LEVEL_GRAVITY,
			positions: HashMap::new(),
			velocities: HashMap::new(),
			player_id: None,
			spawn_point: Vec2::zero(),
			next_entity_id: 1,
			last_grounded_pos: None,

			entity_kinds: HashMap::new(),
			render_styles: HashMap::new(),
			widths: HashMap::new(),
			heights: HashMap::new(),
			speeds: HashMap::new(),
			strengths: HashMap::new(),
			luck: HashMap::new(),
			range_maxes: ComponentStore::new(),
			range_mins: ComponentStore::new(),
			jump_multipliers: ComponentStore::new(),
			gravity_multipliers: ComponentStore::new(),
			enemy_ids: Vec::new(),
		};

		state.set_spawn_point(spawn_top, spawn_left);
		// state.spawn_player();

		return state;
	}

	pub fn set_spawn_point(&mut self, top: f32, left: f32) {
		self.spawn_point.x = left;
		self.spawn_point.y = top;
	}

	/*
		// Create the player
		pub fn spawn_player(&mut self) -> EntityId {
			// if you already have a player, don't create another
			if self.player_id.is_some() {
				self.respawn_player();
				return self.get_player_id();
			}

			let id: EntityId = self.add_entity(1, self.spawn_point, Vec2::zero(), 0, 1, 16, 16, 0, 0, 0, 0.0, 0.0);
			self.set_player(id);
			let mut pos = self.spawn_point;
			let _ = collision::scan_down_to_ground(&self.level, &mut pos, 8.0, 8.0, 64);
			self.last_grounded_pos = Some(self.spawn_point);
			return id;
		}
	*/

	pub fn respawn_player(&mut self) {
		// TODO: Add wait here (.25 seconds)
		let player_id: EntityId = self.get_player_id();

		let spawn_pos: Vec2 = match self.last_grounded_pos {
			Some(p) => p,
			None => self.spawn_point, // level default
		};

		let (_half_width, half_height) = self.get_entity_half_values(player_id);
		let spawn_pos: Vec2 = spawn_pos + Vec2::new(0.0, -half_height - 0.1);

		if let Some(pos) = self.positions.get_mut(&player_id) {
			*pos = spawn_pos;
		}

		if let Some(vel) = self.velocities.get_mut(&player_id) {
			*vel = Vec2::zero();
		}
	}

	#[inline(always)]
	pub fn set_player(&mut self, id: EntityId) {
		self.player_id = Some(id);
	}

	#[inline(always)]
	pub fn get_player_id(&self) -> EntityId {
		self.player_id.expect("player_id not set")
	}

	#[allow(dead_code)]
	#[inline(always)]
	pub fn player_pos(&self) -> Option<&Vec2> {
		self.player_id.and_then(|id| self.positions.get(&id))
	}

	#[inline(always)]
	pub fn on_ground(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(&id) else {
			return false;
		};

		let (half_width, half_height) = self.get_entity_half_values(id);

		let inset: f32 = 0.5;

		let foot_y: f32 = pos.y + half_height + inset;
		let left_x: f32 = pos.x - half_width + inset;
		let right_x: f32 = pos.x + half_width - inset;

		let grounded: bool = self.level.is_solid_world_f32(left_x, foot_y) || self.level.is_solid_world_f32(right_x, foot_y);

		return grounded;
	}

	pub fn on_wall_left(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(&id) else {
			return false;
		};

		let (half_w, half_h) = self.get_entity_half_values(id);

		let inset: f32 = 0.5;
		let probe_x: f32 = pos.x - half_w - inset;

		let y_top: f32 = pos.y - half_h + inset;
		let y_mid: f32 = pos.y;
		let y_bot: f32 = pos.y + half_h - inset;

		let hit: bool = self.level.is_solid_world_f32(probe_x, y_top) || self.level.is_solid_world_f32(probe_x, y_mid) || self.level.is_solid_world_f32(probe_x, y_bot);

		return hit;
	}

	pub fn on_wall_right(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(&id) else {
			return false;
		};

		let (half_w, half_h) = self.get_entity_half_values(id);

		let inset: f32 = 0.5;
		let probe_x: f32 = pos.x + half_w + inset;

		let y_top: f32 = pos.y - half_h + inset;
		let y_mid: f32 = pos.y;
		let y_bot: f32 = pos.y + half_h - inset;

		let hit: bool = self.level.is_solid_world_f32(probe_x, y_top) || self.level.is_solid_world_f32(probe_x, y_mid) || self.level.is_solid_world_f32(probe_x, y_bot);

		return hit;
	}

	pub fn get_entity_half_values(&self, id: EntityId) -> (f32, f32) {
		let width: f32 = self.widths.get(&id).copied().unwrap_or(16) as f32;
		let height: f32 = self.heights.get(&id).copied().unwrap_or(16) as f32;

		let half_width: f32 = width * 0.5;
		let half_height: f32 = height * 0.5;

		return (half_width, half_height);
	}

	/*
		pub fn get_entity_half_values(&self, entity_id: EntityId) -> (f32, f32) {
			let width_sub: u8 = *self.width.get(&entity_id).unwrap_or(&16); // default 1 tile
			let height_sub: u8 = *self.height.get(&entity_id).unwrap_or(&16); // default 1 tile

			let tile_width: f32 = self.level.tile_width as f32;
			let tile_height: f32 = self.level.tile_height as f32;

			let width_tiles: f32 = (width_sub as f32) / 16.0;
			let height_tiles: f32 = (height_sub as f32) / 16.0;

			let half_width: f32 = (width_tiles * tile_width) * 0.5;
			let half_height: f32 = (height_tiles * tile_height) * 0.5;

			return (half_width, half_height);
		}
	*/

	pub fn add_entity(
		&mut self,
		kind: u8,
		position: Vec2,
		velocity: Vec2,
		render_style: u8,
		gravity_multiplier: u8,
		jump_multiplier: u8,
		width: u8,
		height: u8,
		speed: u8,
		strength: u8,
		luck: u8,
		range_min: f32,
		range_max: f32,
	) -> EntityId {
		let width: u8 = if width == 0 { 16 } else { width };
		let height: u8 = if height == 0 { 16 } else { height };

		let id: EntityId = self.next_entity_id;
		self.next_entity_id += 1;
		self.positions.insert(id, position);
		self.velocities.insert(id, velocity);
		self.entity_kinds.insert(id, kind);
		self.render_styles.insert(id, render_style);
		self.gravity_multipliers.push(id, gravity_multiplier);
		self.widths.insert(id, width);
		self.heights.insert(id, height);
		self.speeds.insert(id, speed);
		self.strengths.insert(id, strength);
		self.luck.insert(id, luck);

		self.jump_multipliers.push(id, jump_multiplier);

		if range_min > 0.0 {
			self.range_mins.push(id, range_min);
		}

		if range_max > 0.0 {
			self.range_maxes.push(id, range_max);
		}

		if EntityKind::is_enemy(kind) {
			self.enemy_ids.push(id);
		}

		return id;
	}

	pub fn remove_entity(&mut self, id: EntityId) {
		self.positions.remove(&id);
		self.velocities.remove(&id);
		self.entity_kinds.remove(&id);
		self.render_styles.remove(&id);
		self.widths.remove(&id);
		self.heights.remove(&id);
		self.speeds.remove(&id);
		self.strengths.remove(&id);
		self.luck.remove(&id);

		self.range_mins.remove(id);
		self.range_maxes.remove(id);
		self.gravity_multipliers.remove(id);
		self.jump_multipliers.remove(id);

		// linear scan is fine. I’ll have maybe dozens of enemies, not millions.
		self.enemy_ids.retain(|&e| e != id);

		if self.player_id == Some(id) {
			self.player_id = None;
		}
	}

	pub fn spawn_level_entities(&mut self) {
		let tile_w: f32 = self.level.tile_width as f32;
		let tile_height: f32 = self.level.tile_height as f32;

		// clone to avoid borrow conflicts: self.level.entities (immut) vs self (mut) for add_entity
		let entities = self.level.entities.clone();

		for e in entities {
			let position: Vec2 = Vec2::new((e.left as f32 + 0.5) * tile_w, (e.top as f32 + 0.5) * tile_height);

			let range_min_x: f32 = (e.range_min as f32) * tile_w;
			let range_max: f32 = (e.range_max as f32) * tile_w;

			// let id: EntityId = self.add_entity(1, self.spawn_point, Vec2::zero(), 0, 1, 16, 16, 0, 0, 0, 0.0, 0.0);

			let id: EntityId = self.add_entity(
				e.kind,
				position,
				Vec2::zero(),
				e.render_style,
				e.gravity_multiplier,
				e.jump_multiplier,
				e.width,
				e.height,
				e.speed,
				e.strength,
				e.luck,
				range_min_x,
				range_max,
			);

			if e.gravity_multiplier > 0 {
				let (hw, hh) = self.get_entity_half_values(id);
				if let Some(p) = self.positions.get_mut(&id) {
					let _ = collision::scan_down_to_ground(&self.level, p, hw, hh, 64);
				}
			}

			if e.kind == EntityKind::Player as u8 {
				let (hw, hh) = self.get_entity_half_values(id);
				if let Some(p) = self.positions.get_mut(&id) {
					let _ = collision::scan_down_to_ground(&self.level, p, hw, hh, 64);
					self.last_grounded_pos = Some(*p);
				}
				self.set_player(id);
			}
		}

		return;
	}

	#[inline(always)]
	pub fn on_ground_safe(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(&id) else {
			return false;
		};

		let (half_w, half_h) = self.get_entity_half_values(id);

		let inset: f32 = 2.0; // > 0.5 so we’re not on the lip
		let foot_y: f32 = pos.y + half_h + 0.5;

		let left_x: f32 = pos.x - half_w + inset;
		let right_x: f32 = pos.x + half_w - inset;

		let left_ok: bool = self.level.is_solid_world_f32(left_x, foot_y);
		let right_ok: bool = self.level.is_solid_world_f32(right_x, foot_y);

		return left_ok && right_ok;
	}
}
