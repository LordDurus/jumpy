use crate::{
	ecs::component_store::ComponentStore,
	engine_math::Vec2,
	game::{Settings, level::Level},
	physics::collision,
	platform::audio::{AudioEngine, SfxId},
	tile::TileCollision,
};

pub type EntityId = u32;

#[derive(Clone, Copy)]
pub struct RespawnState {
	pub last_grounded_pos: Vec2,
	pub has_last_grounded_pos: bool,
	pub respawn_cooldown_frames: u8,
}

#[derive(Clone, Copy)]
pub struct JumpState {
	pub coyote_frames_left: u8,
	pub jump_buffer_frames_left: u8,
	pub jump_was_down: bool,
	pub was_grounded: bool,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntityKind {
	Empty = 0,
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

	#[inline(always)]
	pub fn is_player(kind: u8) -> bool {
		kind == EntityKind::Player as u8
	}

	pub fn from_u8(v: u8) -> EntityKind {
		match v {
			1 => EntityKind::Player,
			2 => EntityKind::Slime,
			3 => EntityKind::Imp,
			4 => EntityKind::MovingPlatform,
			_ => EntityKind::Empty,
		}
	}
}

/// Represents the game world, containing entities and their properties (runtime state).
pub struct GameState {
	pub level: Level,
	pub positions: ComponentStore<Vec2>,
	pub velocities: ComponentStore<Vec2>,
	pub player_id: Option<EntityId>,
	pub spawn_point: Vec2,
	pub entity_kinds: ComponentStore<u8>,
	pub render_styles: ComponentStore<u8>,
	pub widths: ComponentStore<u8>,
	pub heights: ComponentStore<u8>,
	pub speeds: ComponentStore<u8>,
	pub strengths: ComponentStore<u8>,
	pub luck: ComponentStore<u8>,
	pub gravity_multipliers: ComponentStore<u8>,
	pub range_mins: ComponentStore<f32>,
	pub range_maxes: ComponentStore<f32>,
	pub jump_multipliers: ComponentStore<u8>,
	pub patrolling: ComponentStore<bool>,
	pub patrol_flips: ComponentStore<bool>,
	pub bump_cooldowns: ComponentStore<u8>,
	pub enemy_ids: Vec<EntityId>,
	pub tick: u32,
	pub settings: Settings,
	pub jump_states: ComponentStore<JumpState>,
	pub respawn_states: ComponentStore<RespawnState>,
	pub respawn_cooldown_frames: u8,
	pub camera_baseline_max_bottom_world: Option<f32>,
	pub audio: Box<dyn AudioEngine>,
	next_entity_id: EntityId,
}

impl GameState {
	pub fn new(current_level: Level, audio: Box<dyn AudioEngine>) -> GameState {
		let spawn_top_tiles: u16 = current_level.player_spawn_top as u16;
		let spawn_left_tiles: u16 = current_level.player_spawn_left as u16;

		let mut state = GameState {
			level: current_level,
			positions: ComponentStore::new(),
			velocities: ComponentStore::new(),
			player_id: None,
			spawn_point: Vec2::zero(),
			next_entity_id: 1,
			entity_kinds: ComponentStore::new(),
			render_styles: ComponentStore::new(),
			widths: ComponentStore::new(),
			heights: ComponentStore::new(),
			speeds: ComponentStore::new(),
			strengths: ComponentStore::new(),
			luck: ComponentStore::new(),
			range_maxes: ComponentStore::new(),
			range_mins: ComponentStore::new(),
			jump_multipliers: ComponentStore::new(),
			gravity_multipliers: ComponentStore::new(),
			patrolling: ComponentStore::new(),
			patrol_flips: ComponentStore::new(),
			bump_cooldowns: ComponentStore::new(),
			settings: Settings::new(),
			jump_states: ComponentStore::new(),
			respawn_states: ComponentStore::new(),
			enemy_ids: Vec::new(),
			respawn_cooldown_frames: 0,
			camera_baseline_max_bottom_world: None,
			audio,
			tick: 0,
		};

		state.set_spawn_point_tiles(spawn_top_tiles, spawn_left_tiles);

		return state;
	}

	pub fn set_spawn_point_tiles(&mut self, top_tiles: u16, left_tiles: u16) {
		let tile_width: f32 = self.level.tile_width as f32;
		let tile_height: f32 = self.level.tile_height as f32;

		// player is 16x16 right now (or pull from game_state.width/height for player id if available)
		let player_width: f32 = 16.0;
		let player_height: f32 = 16.0;

		let left: f32 = (left_tiles as f32) * tile_width;
		let top: f32 = (top_tiles as f32) * tile_height;

		self.spawn_point.x = left + (player_width * 0.5);
		self.spawn_point.y = top + (player_height * 0.5);

		return;
	}

	pub fn kill_player(&mut self, player_id: EntityId) {
		if let Some(respawn_state) = self.respawn_states.get_mut(player_id) {
			respawn_state.respawn_cooldown_frames = self.respawn_cooldown_frames;
		}

		self.audio.play_sfx_and_wait(SfxId::Player1Died);
		self.respawn_cooldown_frames = 20;
		self.respawn_player(player_id);
	}

	pub fn respawn_player(&mut self, player_id: EntityId) {
		let spawn_base: Vec2 = match self.respawn_states.get(player_id) {
			Some(respawn_state) if respawn_state.has_last_grounded_pos => respawn_state.last_grounded_pos,
			_ => self.spawn_point,
		};

		let (_half_width, half_height) = self.get_entity_half_values(player_id);
		let spawn_pos: Vec2 = spawn_base + Vec2::new(0.0, -half_height - 0.1);

		if let Some(pos) = self.positions.get_mut(player_id) {
			*pos = spawn_pos;
		}

		if let Some(vel) = self.velocities.get_mut(player_id) {
			*vel = Vec2::zero();
		}
	}

	pub fn set_player(&mut self, id: EntityId) {
		self.player_id = Some(id);
	}

	pub fn get_player_id(&self) -> EntityId {
		self.player_id.expect("player_id not set")
	}

	#[inline(always)]
	pub fn on_moving_platform(&self, entity_id: EntityId) -> bool {
		self.get_moving_platform_vx(entity_id).is_some()
	}

	#[inline(always)]
	pub fn get_moving_platform_vx(&self, entity_id: EntityId) -> Option<f32> {
		let Some(pos) = self.positions.get(entity_id) else {
			return None;
		};

		let (half_width, half_height) = self.get_entity_half_values(entity_id);

		let inset_x: f32 = 0.5;
		let foot_y: f32 = pos.y + half_height + 0.5;
		let ent_left: f32 = pos.x - half_width + inset_x;
		let ent_right: f32 = pos.x + half_width - inset_x;

		for (entity_id, position) in self.positions.iter() {
			let kind_u8: u8 = *self.entity_kinds.get(entity_id).unwrap_or(&0);
			if EntityKind::from_u8(kind_u8) != EntityKind::MovingPlatform {
				continue;
			}

			let (ph_width, ph_height) = self.get_entity_half_values(entity_id);

			let plat_left: f32 = position.x - ph_width;
			let plat_right: f32 = position.x + ph_width;
			let plat_top: f32 = position.y - ph_height;

			if ent_right < plat_left || ent_left > plat_right {
				continue;
			}

			if (foot_y - plat_top).abs() <= 1.0 {
				let vx: f32 = self.velocities.get(entity_id).map(|v| v.x).unwrap_or(0.0);
				return Some(vx);
			}
		}

		return None;
	}

	pub fn on_wall_left(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(id) else {
			return false;
		};

		let (half_w, half_h) = self.get_entity_half_values(id);

		let inset: f32 = 0.5;
		let probe_x: f32 = pos.x - half_w - inset;

		let y_top: f32 = pos.y - half_h + inset;
		let y_mid: f32 = pos.y;
		let y_bot: f32 = pos.y + half_h - inset;

		let hit: bool = self.level.is_solid_tile_f32(probe_x, y_top) || self.level.is_solid_tile_f32(probe_x, y_mid) || self.level.is_solid_tile_f32(probe_x, y_bot);

		return hit;
	}

	pub fn on_wall_right(&self, id: EntityId) -> bool {
		let Some(pos) = self.positions.get(id) else {
			return false;
		};

		let (half_w, half_h) = self.get_entity_half_values(id);

		let inset: f32 = 0.5;
		let probe_x: f32 = pos.x + half_w + inset;

		let y_top: f32 = pos.y - half_h + inset;
		let y_mid: f32 = pos.y;
		let y_bot: f32 = pos.y + half_h - inset;

		let hit: bool = self.level.is_solid_tile_f32(probe_x, y_top) || self.level.is_solid_tile_f32(probe_x, y_mid) || self.level.is_solid_tile_f32(probe_x, y_bot);

		return hit;
	}

	pub fn get_entity_half_values(&self, id: EntityId) -> (f32, f32) {
		let width: f32 = self.widths.get(id).copied().unwrap_or(16) as f32;
		let height: f32 = self.heights.get(id).copied().unwrap_or(16) as f32;

		let half_width: f32 = width * 0.5;
		let half_height: f32 = height * 0.5;

		return (half_width, half_height);
	}

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
		let width: u8 = if width == 0 { 1 } else { width };
		let height: u8 = if height == 0 { 1 } else { height };

		let id: EntityId = self.next_entity_id;
		self.next_entity_id += 1;
		self.positions.set(id, position);

		self.velocities.set(id, velocity);
		self.entity_kinds.set(id, kind);
		self.render_styles.set(id, render_style);
		self.gravity_multipliers.set(id, gravity_multiplier);

		self.widths.set(id, width);
		self.heights.set(id, height);
		self.speeds.set(id, speed);
		self.strengths.set(id, strength);
		self.luck.set(id, luck);

		self.jump_multipliers.set(id, jump_multiplier);

		if range_min > 0.0 {
			self.range_mins.set(id, range_min);
		}

		if range_max > 0.0 {
			self.range_maxes.set(id, range_max);
		}

		if EntityKind::is_enemy(kind) {
			self.enemy_ids.push(id);
		} else {
			self.jump_states.set(
				id,
				JumpState {
					coyote_frames_left: 0,
					jump_buffer_frames_left: 0,
					jump_was_down: false,
					was_grounded: false,
				},
			);

			self.respawn_states.set(
				id,
				RespawnState {
					last_grounded_pos: self.spawn_point,
					has_last_grounded_pos: false,
					respawn_cooldown_frames: 0,
				},
			);
		}

		if (range_min > 0.0 && range_max > 0.0) || gravity_multiplier == 0 && speed > 0 {
			self.patrolling.set(id, true);
		}

		return id;
	}

	pub fn remove_entity(&mut self, id: EntityId) {
		self.positions.remove(id);
		self.velocities.remove(id);
		self.entity_kinds.remove(id);
		self.render_styles.remove(id);
		self.widths.remove(id);
		self.heights.remove(id);
		self.speeds.remove(id);
		self.strengths.remove(id);
		self.luck.remove(id);
		self.range_mins.remove(id);
		self.range_maxes.remove(id);
		self.gravity_multipliers.remove(id);
		self.jump_multipliers.remove(id);
		self.patrolling.remove(id);
		self.jump_states.remove(id);
		self.respawn_states.remove(id);

		// linear scan is fine. I’ll have maybe dozens of enemies, not millions.
		self.enemy_ids.retain(|&e| e != id);

		if self.player_id == Some(id) {
			self.player_id = None;
		}
	}

	pub fn spawn_level_entities(&mut self) {
		let tile_w: f32 = self.level.tile_width as f32;
		let tile_height: f32 = self.level.tile_height as f32;

		// clone to avoid borrow conflicts: self.level.entities (immutable) vs self (mutable) for add_entity
		let entities = self.level.entities.clone();

		for e in entities {
			let position: Vec2 = Vec2::new((e.left as f32 + 0.5) * tile_w, (e.top as f32 + 0.5) * tile_height);

			let range_min_x: f32 = (e.range_min as f32) * tile_w;
			let range_max: f32 = (e.range_max as f32) * tile_w;

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
				if let Some(p) = self.positions.get_mut(id) {
					let _ = collision::scan_down_to_ground(&self.level, p, hw, hh, 30);
				}
			}

			if EntityKind::is_player(e.kind) {
				self.set_player(id);
			}
		}

		return;
	}

	pub fn is_grounded_now(&self, entity_id: EntityId) -> bool {
		let (grounded, grounded_safe) = self.get_ground_state(entity_id);
		let on_platform: bool = self.on_moving_platform(entity_id);
		return (grounded && grounded_safe) || on_platform;
	}

	pub fn get_ground_state(&self, entity_id: EntityId) -> (bool, bool) {
		let (half_width, half_height) = self.get_entity_half_values(entity_id);

		let Some(pos) = self.positions.get(entity_id) else {
			return (false, false);
		};

		let tile_width: f32 = self.level.tile_width as f32;
		let tile_height: f32 = self.level.tile_height as f32;

		let eps: f32 = 0.05;
		let foot_y: f32 = pos.y + half_height;
		let probe_tile_y: i32 = ((foot_y + eps) / tile_height).floor() as i32;

		let foot_left_x: f32 = pos.x - half_width + eps;
		let foot_right_x: f32 = pos.x + half_width - eps;

		let layer: u32 = self.level.get_action_layer_index() as u32;

		let mut grounded: bool = false;
		let mut grounded_safe: bool = false;

		for foot_x in [foot_left_x, foot_right_x] {
			let tx: i32 = (foot_x / tile_width).floor() as i32;

			if tx < 0 || tx >= self.level.width as i32 {
				continue;
			}
			if probe_tile_y < 0 || probe_tile_y >= self.level.height as i32 {
				continue;
			}

			let tile = self.level.get_tile_at_layer(layer, tx, probe_tile_y);
			match tile.get_collision_kind() {
				TileCollision::Solid => {
					grounded = true;
					grounded_safe = true;
					break;
				}
				TileCollision::OneWay => {
					grounded = true;
					grounded_safe = true; // important with your current grounded_now logic
					break;
				}
				_ => {}
			}
		}

		return (grounded, grounded_safe);
	}
}
