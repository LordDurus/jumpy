use crate::{
	engine_math::rects_overlap,
	game::{
		game_session::GameSession,
		game_state::{EntityId, GameState},
	},
};

const TRIGGER_MODE_AUTO: u16 = 0;
const TRIGGER_MODE_ACTION: u16 = 1;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum TriggerKind {
	Empty = 0,
	LevelExit = 1,
	Message = 2,
}

impl TriggerKind {
	pub fn from_u8(v: u8) -> TriggerKind {
		match v {
			1 => TriggerKind::LevelExit,
			2 => TriggerKind::Message,
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

	// generic params from file (meaning depends on kind)
	pub p0: u16,
	pub p1: u16,
}

/// returns true if an ACTION trigger consumed the action press (so caller should NOT jump)
pub fn handle_message_triggers(game_state: &mut GameState, action_pressed: bool) -> bool {
	let player_id: EntityId = game_state.get_player_id();

	let Some(player_pos) = game_state.positions.get(player_id) else {
		return false;
	};

	let (player_half_width, player_half_height) = game_state.get_entity_half_values(player_id);

	let player_left_world: f32 = player_pos.x - player_half_width;
	let player_top_world: f32 = player_pos.y - player_half_height;
	let player_width_world: f32 = player_half_width * 2.0;
	let player_height_world: f32 = player_half_height * 2.0;

	let tile_width_world: f32 = game_state.level.tile_width as f32;
	let tile_height_world: f32 = game_state.level.tile_height as f32;

	let armed_len: usize = game_state.trigger_armed.len();
	let mut consumed_action: bool = false;

	for trigger in &game_state.level.triggers {
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

		let is_overlapping: bool = rects_overlap(
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
			game_state.trigger_armed[trigger_index] = false;
			continue;
		}

		let activation_mode: u16 = trigger.p0;
		let message_id: u16 = trigger.p1;

		if activation_mode == TRIGGER_MODE_AUTO {
			if !game_state.trigger_armed[trigger_index] {
				game_state.trigger_armed[trigger_index] = true;

				let msg: &str = game_state.message_table.get(message_id);
				println!("{}", msg);
			}
		} else if activation_mode == TRIGGER_MODE_ACTION {
			if action_pressed && !game_state.trigger_armed[trigger_index] {
				game_state.trigger_armed[trigger_index] = true;

				let msg: &str = game_state.message_table.get(message_id);
				println!("{}", msg);

				consumed_action = true;
			}
		}
	}

	return consumed_action;
}

pub fn handle_level_exit_triggers(session: &mut GameSession, game_state: &mut GameState, _action_pressed: bool) {
	let player_id: EntityId = game_state.get_player_id();

	let Some(player_pos) = game_state.positions.get(player_id) else {
		return;
	};

	let (player_half_width, player_half_height) = game_state.get_entity_half_values(player_id);

	let player_left_world: f32 = player_pos.x - player_half_width;
	let player_top_world: f32 = player_pos.y - player_half_height;
	let player_width_world: f32 = player_half_width * 2.0;
	let player_height_world: f32 = player_half_height * 2.0;

	let tile_width_world: f32 = game_state.level.tile_width as f32;
	let tile_height_world: f32 = game_state.level.tile_height as f32;

	let armed_len: usize = game_state.trigger_armed.len();

	for trigger in &game_state.level.triggers {
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

		let is_overlapping: bool = rects_overlap(
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
			game_state.trigger_armed[trigger_index] = false;
			continue;
		}

		// one-shot while overlapping
		if game_state.trigger_armed[trigger_index] {
			continue;
		}
		game_state.trigger_armed[trigger_index] = true;

		let world_id: u16 = trigger.p0;
		let level_id: u16 = trigger.p1;

		// matches your current convention in main.rs
		let next_level_name: String = format!("../worlds/{:02}/{:02}.lvlb", world_id, level_id);

		session.pending_level_name = Some(next_level_name);
		return;
	}
}
