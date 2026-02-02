#[cfg(feature = "gba")]
extern crate alloc;

#[cfg(feature = "gba")]
use alloc::string::String;

use crate::{
	State, debugln,
	runtime::{
		Settings,
		book::{ActiveBookReader, ActiveBookTextSource, reading_state::BookReadingState},
		inventory::Inventory,
		level::Level,
		message_table::MessageTable,
		music::MusicId,
	},
};

pub const MAX_PLAYERS: usize = 4;

#[derive(Clone, Debug)]
pub struct PlayerPersistentState {
	pub hit_points: u16,
}

impl PlayerPersistentState {
	pub fn new_default() -> PlayerPersistentState {
		return PlayerPersistentState { hit_points: 5 };
	}
}

pub struct Session {
	pub players: [PlayerPersistentState; MAX_PLAYERS],
	pub current_level_name: Option<String>,
	pub pending_level_name: Option<String>,
	pub settings: Settings,
	pub inventory: Inventory,
	#[allow(dead_code)]
	pub random_state_u32: u32,
	pub random_state_u16: u16,
	pub message_table: MessageTable,
	pub book_reader: ActiveBookReader,
	pub book_reading: BookReadingState,
	pub active_music_id: MusicId,
}

impl Session {
	pub fn new() -> Session {
		const LINES_PER_PAGE: usize = 25;

		let settings: Settings = Settings::new();
		let message_table: MessageTable = MessageTable::load(settings.language_code.as_str()).unwrap_or_else(|e| {
			debugln!("message table load failed: {}", e);
			return MessageTable::load("en-us").unwrap();
		});

		return Session {
			players: [
				PlayerPersistentState::new_default(),
				PlayerPersistentState::new_default(),
				PlayerPersistentState::new_default(),
				PlayerPersistentState::new_default(),
			],
			current_level_name: None,
			pending_level_name: None,
			settings: settings,
			inventory: Inventory::new(),
			random_state_u32: 0x1234_5678,
			random_state_u16: 0xACE1,
			message_table,
			book_reader: ActiveBookReader::new(ActiveBookTextSource::new(), LINES_PER_PAGE),
			book_reading: BookReadingState::closed(),
			active_music_id: MusicId::None,
		};
	}

	pub fn transition_to_level<FLoad>(&mut self, state: &mut State, level_name: &str, load_level: FLoad) -> bool
	where
		FLoad: Fn(&str) -> Result<Level, String>,
	{
		// 1) save current runtime -> persistent
		state.save_player_to_persistent(self);

		// 2) load next level (pc now; gba later via assets wrapper)
		let next_level: Level = match load_level(level_name) {
			Ok(l) => l,
			Err(e) => {
				debugln!("level load failed: {}", e);
				return false;
			}
		};

		let next_music_id: MusicId = next_level.music_id;

		// 3) move audio backend into the new state (can't clone Box<dyn AudioEngine>)
		let audio = state.take_audio();

		// 4) make a fresh state
		let mut new_state = State::new(next_level, audio);

		// 5) spawn entities + apply player persistent
		new_state.spawn_level_entities();
		new_state.apply_player_from_persistent(self);

		// 7) swap
		*state = new_state;

		// self.current_level_name = Some(level_name.to_string());
		self.current_level_name = Some(String::from(level_name));

		if self.active_music_id != next_music_id {
			if self.settings.is_background_music_enabled {
				state.audio.play_music(next_music_id, true);
				self.active_music_id = next_music_id;
			}
		}

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

/*
pub fn build_level_binary_path(level_ref: LevelReference) -> PathBuf {
	let world_folder: String = format!("world_{:02}", level_ref.world_id);
	let level_file: String = format!("level_{:02}.lvlb", level_ref.level_id);
	let path: PathBuf = PathBuf::from("assets").join("levels").join(world_folder).join(level_file);
	return path;
}
*/
