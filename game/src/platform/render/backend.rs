use crate::{
	Session, State,
	engine_math::Vec2,
	platform::render::{BackgroundDrawParams, input::InputState},
	runtime::state::EntityKind,
};

pub trait RenderBackend {
	fn get_screen_size(&self) -> (i32, i32);
	fn get_render_scale(&self) -> f32;

	fn new() -> Self
	where
		Self: Sized;

	fn init(&mut self);

	fn poll_input(&mut self) -> InputState;
	fn begin_frame(&mut self);

	fn draw_level(&mut self, world: &State, session: &Session);

	// fn draw_background(&mut self, bg_id: BackgroundId, camera_left_world: i32, camera_top_world: i32, scale: f32);
	fn draw_background(&mut self, params: &BackgroundDrawParams);

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
	);

	fn commit(&mut self);
}
