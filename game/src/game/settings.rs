pub struct Settings {
	pub gravity: f32,
	pub coyote_frames_max: u8,
	pub jump_buffer_frames_max: u8,
	pub jump_cut_multiplier: f32,
	pub jump_velocity: f32,
	pub stomp_bounce_multiplier: f32,
	pub bounce_separator: f32,
	pub camera_bottom_padding_tiles: u8,
	pub is_background_music_enabled: bool,
	pub are_sound_effects_enabled: bool,
}

impl Settings {
	pub fn new() -> Self {
		return Self {
			coyote_frames_max: 15,
			jump_buffer_frames_max: 6,
			jump_cut_multiplier: 0.4,
			jump_velocity: -6.0,
			stomp_bounce_multiplier: 0.6,
			gravity: 0.35,
			bounce_separator: 0.5,
			camera_bottom_padding_tiles: 2,
			is_background_music_enabled: false,
			are_sound_effects_enabled: true,
		};
	}
}
