use crate::{
	debugln,
	engine_math::{do_they_overlap, random_u16},
	platform::input::TriggerPresses,
	runtime::{
		session::Session,
		state::{EntityId, State},
	},
};

pub const TRIGGER_MODE_AUTO: u16 = 0;
pub const TRIGGER_MODE_ACTION: u16 = 1;
pub const TRIGGER_MODE_UP: u16 = 2;
pub const TRIGGER_MODE_DOWN: u16 = 3;
pub const TRIGGER_MODE_LEFT: u16 = 4;
pub const TRIGGER_MODE_RIGHT: u16 = 5;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriggerKind {
	Empty = 0,
	LevelExit = 1,
	Message = 2,
	Pickup = 3,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PickupKind {
	Empty = 0,
	Coin = 1,
	Book = 2,
	Key = 3,
	Random = 4,
}

impl PickupKind {
	pub fn from_u8(v: u8) -> PickupKind {
		match v {
			1 => PickupKind::Coin,
			2 => PickupKind::Key,
			3 => PickupKind::Book,
			4 => PickupKind::Random,
			_ => PickupKind::Empty,
		}
	}
}

impl TriggerKind {
	pub fn from_u8(v: u8) -> TriggerKind {
		match v {
			1 => TriggerKind::LevelExit,
			2 => TriggerKind::Message,
			3 => TriggerKind::Pickup,
			_ => TriggerKind::Empty,
		}
	}
}

#[derive(Debug, Clone)]
pub struct LevelTrigger {
	pub kind: u8,
	pub id: u16,

	// position in tiles (convert to world when needed)
	pub left_tiles: u16,
	pub top_tiles: u16,
	pub width_tiles: u16,
	pub height_tiles: u16,
	pub activation_mode: u8,

	// generic params from file (meaning depends on kind)
	pub p0: u16,
	pub p1: u16,
	pub icon_id: u16,
}

impl LevelTrigger {
	// --- shared ---
	#[inline(always)]
	pub fn get_activation_mode(&self) -> u16 {
		return self.activation_mode as u16;
	}

	// --- message ---
	#[inline(always)]
	pub fn get_message_id(&self) -> u16 {
		return self.p1;
	}

	// --- level exit ---
	#[inline(always)]
	pub fn get_world_id(&self) -> u16 {
		return self.p0;
	}

	#[inline(always)]
	pub fn get_level_id(&self) -> u16 {
		return self.p1;
	}
}

/// returns true if an ACTION trigger consumed the action press (so caller should NOT jump)
pub fn handle_message_triggers(session: &Session, state: &mut State, trigger_presses: TriggerPresses) -> bool {
	let player_id: EntityId = state.get_player_id();

	let Some(player_pos) = state.positions.get(player_id) else {
		return false;
	};

	let (player_half_width, player_half_height) = state.get_entity_half_values(player_id);

	let player_left_world: f32 = player_pos.x - player_half_width;
	let player_top_world: f32 = player_pos.y - player_half_height;
	let player_width_world: f32 = player_half_width * 2.0;
	let player_height_world: f32 = player_half_height * 2.0;

	let tile_width_world: f32 = state.level.tile_width as f32;
	let tile_height_world: f32 = state.level.tile_height as f32;

	let armed_len: usize = state.triggers_armed.len();
	let mut consumed_action: bool = false;

	for trigger in &state.level.triggers {
		if TriggerKind::from_u8(trigger.kind) != TriggerKind::Message {
			continue;
		}

		let trigger_index: usize = trigger.id as usize;
		if trigger_index >= armed_len {
			continue;
		}

		let trig_left_world: f32 = (trigger.left_tiles as f32) * tile_width_world;
		let trig_top_world: f32 = (trigger.top_tiles as f32) * tile_height_world;
		let trig_width_world: f32 = (trigger.width_tiles as f32) * tile_width_world;
		let trig_height_world: f32 = (trigger.height_tiles as f32) * tile_height_world;

		let is_overlapping: bool = do_they_overlap(
			player_left_world,
			player_top_world,
			player_width_world,
			player_height_world,
			trig_left_world,
			trig_top_world,
			trig_width_world,
			trig_height_world,
		);

		if !is_overlapping {
			state.triggers_armed[trigger_index] = false;
			continue;
		}

		let activation_mode: u16 = trigger.p0;
		let message_id: u16 = trigger.get_message_id();

		if activation_mode == TRIGGER_MODE_AUTO {
			if !state.triggers_armed[trigger_index] {
				state.triggers_armed[trigger_index] = true;

				let msg: &str = session.message_table.get(message_id);
				debugln!("{}", msg);
			}
		} else if activation_mode == TRIGGER_MODE_ACTION {
			if trigger_presses.action_pressed && !state.triggers_armed[trigger_index] {
				state.triggers_armed[trigger_index] = true;

				let msg: &str = session.message_table.get(message_id);
				debugln!("{}", msg);

				consumed_action = true;
			}
		}
	}

	return consumed_action;
}

pub fn handle_level_exit_triggers(session: &mut Session, game: &mut State, presses: TriggerPresses) {
	let player_id: EntityId = game.get_player_id();
	let Some(player_pos) = game.positions.get(player_id) else {
		return;
	};

	let (player_half_width, player_half_height) = game.get_entity_half_values(player_id);

	let player_left_world: f32 = player_pos.x - player_half_width;
	let player_top_world: f32 = player_pos.y - player_half_height;
	let player_width_world: f32 = player_half_width * 2.0;
	let player_height_world: f32 = player_half_height * 2.0;

	let tile_width_world: f32 = game.level.tile_width as f32;
	let tile_height_world: f32 = game.level.tile_height as f32;

	let armed_len: usize = game.triggers_armed.len();

	for trigger in &game.level.triggers {
		if TriggerKind::from_u8(trigger.kind) != TriggerKind::LevelExit {
			continue;
		}

		let trigger_index: usize = trigger.id as usize;
		if trigger_index >= armed_len {
			continue;
		}

		let trig_left_world: f32 = (trigger.left_tiles as f32) * tile_width_world;
		let trig_top_world: f32 = (trigger.top_tiles as f32) * tile_height_world;
		let trig_width_world: f32 = (trigger.width_tiles as f32) * tile_width_world;
		let trig_height_world: f32 = (trigger.height_tiles as f32) * tile_height_world;

		let is_overlapping: bool = do_they_overlap(
			player_left_world,
			player_top_world,
			player_width_world,
			player_height_world,
			trig_left_world,
			trig_top_world,
			trig_width_world,
			trig_height_world,
		);

		if !is_overlapping {
			game.triggers_armed[trigger_index] = false;
			continue;
		}

		let mode: u16 = trigger.get_activation_mode();

		// one-shot while overlapping for all modes
		if game.triggers_armed[trigger_index] {
			continue;
		}

		// auto fires immediately on overlap, others only fire on matching press
		if mode != TRIGGER_MODE_AUTO && !should_fire(mode, presses) {
			continue;
		}

		game.triggers_armed[trigger_index] = true;

		let next_level_name: String = format!("../worlds/{:02}/{:02}.lvlb", trigger.get_world_id(), trigger.get_level_id());
		session.pending_level_name = Some(next_level_name);
		return;
	}
}

pub fn handle_pickup_triggers(session: &mut Session, state: &mut State, presses: TriggerPresses) -> bool {
	let mut consumed_action: bool = false;
	let player_id: EntityId = state.get_player_id();
	let Some(player_pos) = state.positions.get(player_id) else {
		return false;
	};

	let (player_half_width, player_half_height) = state.get_entity_half_values(player_id);
	let player_left_world: f32 = player_pos.x - player_half_width;
	let player_top_world: f32 = player_pos.y - player_half_height;
	let player_width_world: f32 = player_half_width * 2.0;
	let player_height_world: f32 = player_half_height * 2.0;

	let tile_width_world: f32 = state.level.tile_width as f32;
	let tile_height_world: f32 = state.level.tile_height as f32;

	let armed_len: usize = state.triggers_armed.len();

	for trigger in &state.level.triggers {
		let kind: TriggerKind = TriggerKind::from_u8(trigger.kind);
		if kind != TriggerKind::Pickup {
			continue;
		}

		let trigger_index: usize = trigger.id as usize;
		if trigger_index >= armed_len {
			continue;
		}

		// already consumed -> never fire again
		if state.triggers_armed[trigger_index] {
			continue;
		}

		let trig_left_world: f32 = (trigger.left_tiles as f32) * tile_width_world;
		let trig_top_world: f32 = (trigger.top_tiles as f32) * tile_height_world;
		let trig_width_world: f32 = (trigger.width_tiles as f32) * tile_width_world;
		let trig_height_world: f32 = (trigger.height_tiles as f32) * tile_height_world;

		let is_overlapping: bool = do_they_overlap(
			player_left_world,
			player_top_world,
			player_width_world,
			player_height_world,
			trig_left_world,
			trig_top_world,
			trig_width_world,
			trig_height_world,
		);

		if !is_overlapping {
			// NOTE: pickups are one-shot, so we do NOT clear trigger_armed here
			continue;
		}

		let mode: u16 = trigger.get_activation_mode();
		if mode != TRIGGER_MODE_AUTO && !should_fire(mode, presses) {
			continue;
		}

		// consume
		state.triggers_armed[trigger_index] = true;

		if mode == TRIGGER_MODE_ACTION {
			// todo remove trigger here
			consumed_action = true;
		}

		match kind {
			TriggerKind::Pickup => {
				let pickup_kind = PickupKind::from_u8(trigger.p0 as u8);
				// p0 = pickup type, p1 = value
				match pickup_kind {
					PickupKind::Coin | PickupKind::Key | PickupKind::Book => {
						apply_pickup(session, trigger.p0, trigger.p1);
					}
					PickupKind::Random => {
						// coin only for now
						let value = random_u16(&mut session.random_state_u16);
						apply_pickup(session, 1, value);
					}

					_ => {
						panic!("Unknown pickup kind: {:?}", pickup_kind);
					}
				}
			}
			_ => {}
		}
	}

	return consumed_action;
}

#[inline(always)]
fn apply_pickup(session: &mut Session, pickup_type: u16, value: u16) {
	if pickup_type == 1 {
		debugln!("adding coins({})", value);

		session.inventory.add_coins(value);
		return;
	}

	if pickup_type == 2 {
		session.inventory.add_key(value);
		return;
	}

	if pickup_type == 3 {
		session.inventory.add_book(value, 200);
		return;
	}

	return;
}

#[inline(always)]
fn should_fire(mode: u16, presses: TriggerPresses) -> bool {
	let result: bool = match mode {
		TRIGGER_MODE_AUTO => true,
		TRIGGER_MODE_ACTION => presses.action_pressed,
		TRIGGER_MODE_UP => presses.up_pressed,
		TRIGGER_MODE_DOWN => presses.down_pressed,
		TRIGGER_MODE_LEFT => presses.left_pressed,
		TRIGGER_MODE_RIGHT => presses.right_pressed,
		_ => false,
	};

	return result;
}
