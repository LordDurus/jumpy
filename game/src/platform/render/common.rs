use super::backend::RenderBackend;
use crate::game::game_state::GameState;

pub struct RenderCommon;

impl RenderCommon {
	pub fn new() -> RenderCommon {
		return RenderCommon;
	}
	pub fn compute_camera<B: RenderBackend>(&self, backend: &B, game_state: &GameState) -> (i32, i32) {
		let (screen_width_pixels, screen_height_pixels) = backend.screen_size();
		let scale: f32 = backend.get_render_scale();

		let screen_width: f32 = (screen_width_pixels as f32) / scale;
		let screen_height: f32 = (screen_height_pixels as f32) / scale;

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

		let tile_width: f32 = game_state.level.tile_width as f32;
		let tile_height: f32 = game_state.level.tile_height as f32;

		let level_width_world: f32 = (game_state.level.width as f32) * tile_width;
		let level_height_world: f32 = (game_state.level.height as f32) * tile_height;

		let mut camera_left: f32 = focus_left - (screen_width * 0.5);
		let mut camera_top: f32 = focus_top - (screen_height * 0.5);

		// --- horizontal deadzone (player can drift before camera follows) ---
		let deadzone_half_width: f32 = tile_width * 3.0; // 3 tiles each side (tweak)
		let center_x: f32 = camera_left + (screen_width * 0.5);
		let dx: f32 = focus_left - center_x;

		if dx > deadzone_half_width {
			camera_left += dx - deadzone_half_width;
		} else if dx < -deadzone_half_width {
			camera_left += dx + deadzone_half_width;
		}

		// clamp left/top to level
		let max_x: f32 = (level_width_world - screen_width).max(0.0);
		let mut max_y: f32 = (level_height_world - screen_height).max(0.0);

		if let Some(baseline_max_bottom_world) = game_state.camera_baseline_max_bottom_world {
			let (_half_width, half_height) = game_state.get_entity_half_values(focus_id.unwrap());
			let player_bottom_world: f32 = focus_top + half_height;

			let pad_world: f32 = game_state.settings.camera_bottom_padding_tiles as f32 * tile_height;

			let effective_max_bottom_world: f32 = baseline_max_bottom_world.max(player_bottom_world + pad_world);
			let max_camera_top: f32 = (effective_max_bottom_world - screen_height).max(0.0);

			if max_y > max_camera_top {
				max_y = max_camera_top;
			}
		}

		camera_left = camera_left.clamp(0.0, max_x);
		camera_top = camera_top.clamp(0.0, max_y);

		return (camera_left.round() as i32, camera_top.round() as i32);
	}
}
