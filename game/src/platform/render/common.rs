use super::backend::RenderBackend;
use crate::game::game_state::GameState;

pub struct RenderCommon;

impl RenderCommon {
	pub fn new() -> RenderCommon {
		return RenderCommon;
	}

	// camera is in world pixels (unscaled)
	pub fn compute_camera<B: RenderBackend>(&self, backend: &B, game_state: &GameState) -> (i32, i32) {
		let (screen_width_pixels, screen_height_pixels) = backend.screen_size();
		let scale: f32 = backend.render_scale();

		let screen_width: i32 = ((screen_width_pixels as f32) / scale).round() as i32;
		let screen_height: i32 = ((screen_height_pixels as f32) / scale).round() as i32;

		let mut focus_left: f32 = 0.0;
		let mut focus_top: f32 = 0.0;

		let focus_id: Option<u32> = if game_state.get_player_id() != 0 {
			Some(game_state.get_player_id())
		} else {
			let mut best_id: Option<u32> = None;
			for id in game_state.positions.keys() {
				if best_id.is_none() || id < best_id.unwrap() {
					best_id = Some(id);
				}
			}
			best_id
		};

		if let Some(id) = focus_id {
			if let Some(p) = game_state.positions.get(id) {
				focus_left = p.x;
				focus_top = p.y;
			}
		}

		let tile_width: i32 = game_state.level.tile_width as i32;
		let tile_height: i32 = game_state.level.tile_height as i32;

		let level_width_pixels: i32 = (game_state.level.width as i32) * tile_width;
		let level_height_pixels: i32 = (game_state.level.height as i32) * tile_height;

		let mut cam_left: i32 = focus_left as i32 - (screen_width / 2);
		let mut cam_top: i32 = focus_top as i32 - (screen_height / 2);

		let max_x: i32 = (level_width_pixels - screen_width).max(0);
		let max_y: i32 = (level_height_pixels - screen_height).max(0);

		if cam_left < 0 {
			cam_left = 0;
		}
		if cam_top < 0 {
			cam_top = 0;
		}
		if cam_left > max_x {
			cam_left = max_x;
		}
		if cam_top > max_y {
			cam_top = max_y;
		}

		/*
		println!(
			"camera: player_id={} focus_id={:?} level_w={} level_h={}",
			world.get_player_id(),
			focus_id,
			world.level.width,
			world.level.height
		);
		*/

		return (cam_left, cam_top);
	}
}
