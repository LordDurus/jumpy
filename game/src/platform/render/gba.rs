#![cfg(feature = "gba")]

use crate::{
	RenderBackend, Session, State,
	engine_math::Vec2,
	platform::{input::InputState, render::common::RenderCommon},
	runtime::{
		assets::{get_font_path, get_gfx_root},
		state::EntityKind,
	},
};

pub struct GbaRenderer {
	frame_index: u32,
}

impl RenderBackend for GbaRenderer {
	fn screen_size(&self) -> (i32, i32) {
		return (240, 160);
	}

	fn get_render_scale(&self) -> f32 {
		return 1.0;
	}

	fn new() -> Self {
		return Self { frame_index: 0 };
	}

	fn init(&mut self) {}

	fn poll_input(&mut self) -> InputState {
		return crate::platform::input::gba::poll();
	}

	fn begin_frame(&mut self) {}

	fn draw_level(&mut self, state: &State, session: &Session) {
		self.frame_index = self.frame_index.wrapping_add(1);
	}

	fn draw_death_entity(
		&mut self,
		state: &State,
		session: &Session,
		entity_kind: EntityKind,
		pos: &Vec2,
		half_height: f32,
		camera_left: f32,
		camera_top: f32,
		scale: f32,
		death_timer: u16,
	) {
	}

	fn commit(&mut self) {
		// stub: don’t call agb unless you’ve got the crate enabled for gba builds
		// agb::wait_for_vblank();
	}
}
