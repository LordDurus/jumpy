use crate::{game::game_state::GameState, platform::render::input::InputState};

pub trait Renderer {
	fn new() -> Self
	where
		Self: Sized;
	fn init(&mut self);

	fn poll_input(&mut self) -> InputState;

	fn begin_frame(&mut self);
	fn draw_world(&mut self, world: &GameState);
	fn commit(&mut self);
}
