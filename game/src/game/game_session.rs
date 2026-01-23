use crate::{
	GameState,
	game::{
		Settings,
		level::{Level, LevelReference},
	},
};

use std::path::PathBuf; // this is not going to work on the gba

pub const MAX_PLAYERS: usize = 4;

#[derive(Clone, Debug)]
pub struct PlayerPersistentState {
	pub hit_points: u16,
	pub max_hit_points: u16,
	pub inventory: Inventory,
	// add more later: keys, coins, upgrades, etc.
}

impl PlayerPersistentState {
	pub fn new_default() -> PlayerPersistentState {
		return PlayerPersistentState {
			hit_points: 5,
			max_hit_points: 5,
			inventory: Inventory::new(),
		};
	}
}

#[derive(Clone, Debug)]
pub struct Inventory {
	// keep it simple for now
	pub coins: u16,
	pub keys: u8,
}

impl Inventory {
	pub fn new() -> Inventory {
		return Inventory { coins: 0, keys: 0 };
	}
}

pub struct GameSession {
	pub players: [PlayerPersistentState; MAX_PLAYERS],
	pub current_level_name: Option<String>,
	pub pending_level_name: Option<String>,
	// keep settings here too if you want them to persist
	pub settings: Settings,
}

impl GameSession {
	pub fn new() -> GameSession {
		return GameSession {
			players: [
				PlayerPersistentState::new_default(),
				PlayerPersistentState::new_default(),
				PlayerPersistentState::new_default(),
				PlayerPersistentState::new_default(),
			],
			current_level_name: None,
			pending_level_name: None,
			settings: Settings::new(),
		};
	}
	pub fn transition_to_level(&mut self, game_state: &mut GameState, level_name: &str) -> bool {
		// 1) save current runtime -> persistent
		game_state.save_player_to_persistent(self);

		// 2) load next level (pc now; gba later via assets wrapper)
		let next_level: Level = match Level::load_binary(level_name) {
			Ok(l) => l,
			Err(e) => {
				println!("level load failed: {}", e);
				return false;
			}
		};

		// 3) move audio backend into the new state (can't clone Box<dyn AudioEngine>)
		let audio = game_state.take_audio();

		// 4) make a fresh state
		let mut new_state = GameState::new(next_level, audio);

		// 5) settings persist
		new_state.settings = self.settings.clone();

		// 6) spawn entities + apply player persistent
		new_state.spawn_level_entities();
		new_state.apply_player_from_persistent(self);

		// 7) swap
		*game_state = new_state;

		self.current_level_name = Some(level_name.to_string());
		return true;
	}

	#[inline(always)]
	pub fn player(&self, index: usize) -> &PlayerPersistentState {
		return &self.players[index];
	}

	#[inline(always)]
	pub fn player_mut(&mut self, index: usize) -> &mut PlayerPersistentState {
		return &mut self.players[index];
	}
}

pub fn build_level_binary_path(level_ref: LevelReference) -> PathBuf {
	let world_folder: String = format!("world_{:02}", level_ref.world_id);
	let level_file: String = format!("level_{:02}.lvlb", level_ref.level_id);
	let path: PathBuf = PathBuf::from("assets").join("levels").join(world_folder).join(level_file);
	return path;
}
