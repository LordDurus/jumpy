use crate::{enemy_kind::EnemyKind, engine_math::Vec2, game::level::Level};
use std::collections::HashMap;

pub type EntityId = u32;

/// Represents the game world, containing entities and their properties (runtime state).
pub struct GameState {
	pub level: Level,
	pub gravity: f32,
	pub positions: HashMap<EntityId, Vec2>,
	pub velocities: HashMap<EntityId, Vec2>,
	pub player_id: Option<EntityId>,
	pub spawn_point: Vec2,
	pub last_grounded_pos: Option<Vec2>,

	pub entity_kind: HashMap<EntityId, u8>,
	pub render_style: HashMap<EntityId, u8>,
	pub width: HashMap<EntityId, u8>,
	pub height: HashMap<EntityId, u8>,
	pub speed: HashMap<EntityId, u8>,
	pub strength: HashMap<EntityId, u8>,
	pub luck: HashMap<EntityId, u8>,
	next_entity_id: EntityId,
}

impl GameState {
	pub fn new(current_level: Level) -> GameState {
		let spawn_x = current_level.player_spawn_x;
		let spawn_y = current_level.player_spawn_y;

		let mut state = GameState {
			level: current_level,
			gravity: crate::physics::constants::LEVEL_GRAVITY,
			positions: HashMap::new(),
			velocities: HashMap::new(),
			player_id: None,
			spawn_point: Vec2::zero(),
			next_entity_id: 1,
			last_grounded_pos: None,

			entity_kind: HashMap::new(),
			render_style: HashMap::new(),
			width: HashMap::new(),
			height: HashMap::new(),
			speed: HashMap::new(),
			strength: HashMap::new(),
			luck: HashMap::new(),
		};

		state.set_spawn_point(spawn_x, spawn_y);
		println!("spawn_x = {} | spawn_y = {}", spawn_x, spawn_y);
		state.spawn_player();

		return state;
	}

	pub fn set_spawn_point(&mut self, x: f32, y: f32) {
		self.spawn_point.x = x;
		self.spawn_point.y = y;
		return;
	}

	// Create the player
	pub fn spawn_player(&mut self) -> EntityId {
		// if you already have a player, don't create another
		if self.player_id.is_some() {
			self.respawn_player();
			return self.get_player_id();
		}

		let id: EntityId = self.add_entity(
			1, // kind (player)
			self.spawn_point,
			Vec2::zero(),
			0, // render_style
			1,
			1, // width, height
			0,
			0,
			0, // speed, strength, luck
		);

		self.set_player(id);
		self.last_grounded_pos = Some(self.spawn_point);
		return id;
	}

	pub fn respawn_player(&mut self) {
		let player_id: EntityId = self.get_player_id();

		let spawn_pos: Vec2 = match self.last_grounded_pos {
			Some(p) => p,
			None => self.spawn_point, // level default
		};

		let (_half_width, half_height) = self.entity_half_extents(player_id);

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

		let (half_w, half_h) = self.entity_half_extents(id);

		let inset: f32 = 0.5;

		let foot_y: f32 = pos.y + half_h + inset;
		let left_x: f32 = pos.x - half_w + inset;
		let right_x: f32 = pos.x + half_w - inset;

		let grounded: bool = self.level.is_solid_world_f32(left_x, foot_y) || self.level.is_solid_world_f32(right_x, foot_y);

		return grounded;
	}

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

	pub fn entity_half_extents(&self, _entity_id: EntityId) -> (f32, f32) {
		// TEMPORARY: default player/block size
		return (8.0, 8.0);
	}

	pub fn add_entity(&mut self, kind: u8, position: Vec2, velocity: Vec2, render_style: u8, width: u8, height: u8, speed: u8, strength: u8, luck: u8) -> EntityId {
		let id: EntityId = self.next_entity_id;
		self.next_entity_id += 1;

		self.positions.insert(id, position);
		self.velocities.insert(id, velocity);

		self.entity_kind.insert(id, kind);
		self.render_style.insert(id, render_style);
		self.width.insert(id, width);
		self.height.insert(id, height);
		self.speed.insert(id, speed);
		self.strength.insert(id, strength);
		self.luck.insert(id, luck);

		return id;
	}

	pub fn remove_entity(&mut self, id: EntityId) {
		self.positions.remove(&id);
		self.velocities.remove(&id);
		self.entity_kind.remove(&id);
		self.render_style.remove(&id);
		self.width.remove(&id);
		self.height.remove(&id);
		self.speed.remove(&id);
		self.strength.remove(&id);
		self.luck.remove(&id);

		if self.player_id == Some(id) {
			self.player_id = None;
		}
	}

	pub fn spawn_level_entities(&mut self) {
		let tile_w: f32 = self.level.tile_width as f32;
		let tile_h: f32 = self.level.tile_height as f32;

		// collect first to avoid borrow issues (immutable borrow of self.level.entities vs &mut self for add_entity)
		let spawns: Vec<(u8, Vec2, u8, u8, u8, u8, u8, u8)> = self
			.level
			.entities
			.iter()
			.filter(|e| e.kind != 0) // 0 = PlayerStart
			.map(|e| {
				let x: f32 = (e.x as f32 + 0.5) * tile_w;
				let y: f32 = (e.y as f32 + 0.5) * tile_h;
				let pos: Vec2 = Vec2::new(x, y);

				return (e.kind, pos, e.render_style, e.width, e.height, e.speed, e.strength, e.luck);
			})
			.collect();

		for (kind, pos, render_style, width, height, speed, strength, luck) in spawns {
			self.add_entity(kind, pos, Vec2::zero(), render_style, width, height, speed, strength, luck);
		}
	}

	#[inline(always)]
	pub fn on_ground_safe(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(&id) else {
			return false;
		};

		let (half_w, half_h) = self.entity_half_extents(id);

		let inset: f32 = 2.0; // > 0.5 so we’re not on the lip
		let foot_y: f32 = pos.y + half_h + 0.5;

		let left_x: f32 = pos.x - half_w + inset;
		let right_x: f32 = pos.x + half_w - inset;

		let left_ok: bool = self.level.is_solid_world_f32(left_x, foot_y);
		let right_ok: bool = self.level.is_solid_world_f32(right_x, foot_y);

		return left_ok && right_ok;
	}
}
