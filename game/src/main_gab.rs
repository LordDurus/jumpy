use crate::{
	game::{game_session::GameSession, game_state::GameState, music::MusicId},
	platform::{
		audio::{AudioEngine, pc::PcAudio},
		level_loader_pc::load_level_from_file,
		render::{backend::RenderBackend, pc::PcRenderer},
	},
};

#[cfg(feature = "gba")]
fn run() {
	while true {}
	return;
}
