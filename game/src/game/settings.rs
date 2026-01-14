pub struct Settings {
	pub coyote_frames_max: u8,

	pub jump_buffer_frames_max: u8,

	// variable jump height
	pub jump_cut_multiplier: f32,

	// core jump physics
	pub jump_velocity: f32,
	pub stomp_bounce_multiplier: f32,
}

impl Settings {
	pub fn new() -> Self {
		return Self {
			coyote_frames_max: 15,
			jump_buffer_frames_max: 6,
			jump_cut_multiplier: 0.4,
			jump_velocity: -6.0,
			stomp_bounce_multiplier: 0.6,
		};
	}
}
