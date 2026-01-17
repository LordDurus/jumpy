#![cfg(feature = "pc")]

use crate::platform::audio::{AudioEngine, SfxId};
use sdl2::mixer::{self, Channel, Chunk, Music};
use std::{collections::HashMap, path::PathBuf};

pub struct PcAudio {
	sfx: HashMap<SfxId, Chunk>,
	music: Option<Music<'static>>,
}

impl PcAudio {
	fn asset_path(file: &str) -> PathBuf {
		return crate::assets::get_audio_root().join("pc").join(file);
	}

	fn load_sfx(&mut self, id: SfxId, file: &str) {
		let path = Self::asset_path(file);
		let chunk = Chunk::from_file(&path).unwrap_or_else(|_| panic!("missing sfx file: {}", path.display()));
		self.sfx.insert(id, chunk);
	}
}

impl AudioEngine for PcAudio {
	fn new() -> Self {
		return Self {
			sfx: HashMap::new(),
			music: None,
		};
	}

	fn init(&mut self) {
		mixer::open_audio(44_100, mixer::DEFAULT_FORMAT, 2, 1_024).expect("failed to open audio");

		mixer::allocate_channels(16);

		// ---- load sfx ----
		self.load_sfx(SfxId::Jump, "jump.wav");
		self.load_sfx(SfxId::Stomp, "stomp.wav");
		self.load_sfx(SfxId::Player1Died, "player1_dead.wav");

		// ---- load music ----
		let music = Music::from_file(Self::asset_path("01_world_music.wav")).expect("01_world_music.wav missing");

		self.music = Some(music);
		self.music.as_ref().unwrap().play(-1).expect("failed to start music");
	}

	fn play_sfx(&mut self, id: SfxId) {
		if let Some(chunk) = self.sfx.get(&id) {
			let _ = Channel::all().play(chunk, 0);
		}
	}

	fn play_sfx_and_wait(&mut self, id: SfxId) {
		mixer::Music::pause();

		if let Some(chunk) = self.sfx.get(&id) {
			let channel = Channel::all().play(chunk, 0).expect("failed to play sfx");

			while channel.is_playing() {
				std::thread::sleep(std::time::Duration::from_millis(5));
			}
		}

		mixer::Music::resume();
	}

	fn update(&mut self) {
		// nothing required for SDL mixer
	}

	fn stop(&mut self, _id: SfxId) {
		Channel::all().halt();
	}
}
