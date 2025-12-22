use crate::GameState;

pub trait RenderBackend {
	fn screen_size(&self) -> (i32, i32);

	fn draw_sheet_tile(&mut self, sheet_tile_id: u16, x: i32, y: i32, tile_w: u32, tile_h: u32);

	fn draw_entities(&mut self, world: &GameState, cam_x: i32, cam_y: i32);
}
