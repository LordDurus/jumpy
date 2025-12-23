use crate::{GameState, platform::render::input::InputState};

pub trait RenderBackend {
	fn screen_size(&self) -> (i32, i32);
	fn render_scale(&self) -> f32;

	fn new() -> Self
	where
		Self: Sized;
	fn init(&mut self);

	fn poll_input(&mut self) -> InputState;
	fn begin_frame(&mut self);

	// top-level: stays 1-parameter
	fn draw_world(&mut self, world: &GameState);

	fn commit(&mut self);

	// primitive: used by RenderCommon
	fn draw_tile(&mut self, sheet_id: u16, x: i32, y: i32, w: u32, h: u32);
}
