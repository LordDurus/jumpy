use super::backend::RenderBackend;
use crate::{game::game_state::GameState, tile::resolve_sheet_tile_id};

pub struct RenderCommon;

impl RenderCommon {
	pub fn new() -> RenderCommon {
		return RenderCommon;
	}

	pub fn compute_camera<B: RenderBackend>(&self, backend: &B, world: &GameState) -> (i32, i32) {
		let (screen_w, screen_h) = backend.screen_size();

		let mut focus_x: f32 = 0.0;
		let mut focus_y: f32 = 0.0;

		// follow smallest entity id (your player is created first)
		let mut best_id: Option<u32> = None;
		for id in world.positions.keys() {
			if best_id.is_none() || *id < best_id.unwrap() {
				best_id = Some(*id);
			}
		}

		if let Some(id) = best_id {
			if let Some(p) = world.positions.get(&id) {
				focus_x = p.x;
				focus_y = p.y;
			}
		}

		let tile_w: i32 = world.level.tile_width as i32;
		let tile_h: i32 = world.level.tile_height as i32;

		let level_w_px: i32 = (world.level.width as i32) * tile_w;
		let level_h_px: i32 = (world.level.height as i32) * tile_h;

		let mut cam_x: i32 = focus_x as i32 - (screen_w / 2);
		let mut cam_y: i32 = focus_y as i32 - (screen_h / 2);

		let max_x: i32 = (level_w_px - screen_w).max(0);
		let max_y: i32 = (level_h_px - screen_h).max(0);

		if cam_x < 0 {
			cam_x = 0;
		}
		if cam_y < 0 {
			cam_y = 0;
		}
		if cam_x > max_x {
			cam_x = max_x;
		}
		if cam_y > max_y {
			cam_y = max_y;
		}

		return (cam_x, cam_y);
	}

	pub fn draw_level<B: RenderBackend>(&self, backend: &mut B, world: &GameState, cam_x: i32, cam_y: i32, frame_index: u32) {
		let (screen_w, screen_h) = backend.screen_size();

		let tile_w_i32: i32 = world.level.tile_width as i32;
		let tile_h_i32: i32 = world.level.tile_height as i32;
		let tile_w: u32 = world.level.tile_width;
		let tile_h: u32 = world.level.tile_height;

		let view_left: i32 = cam_x / tile_w_i32;
		let view_top: i32 = cam_y / tile_h_i32;
		let view_right: i32 = (cam_x + screen_w + tile_w_i32 - 1) / tile_w_i32;
		let view_bottom: i32 = (cam_y + screen_h + tile_h_i32 - 1) / tile_h_i32;

		let max_x: i32 = world.level.width as i32;
		let max_y: i32 = world.level.height as i32;

		let x0: i32 = view_left.max(0);
		let y0: i32 = view_top.max(0);
		let x1: i32 = view_right.min(max_x);
		let y1: i32 = view_bottom.min(max_y);

		for ty in y0..y1 {
			for tx in x0..x1 {
				let kind = world.level.tile_at(tx, ty);
				if kind == crate::tile::TileKind::Empty {
					continue;
				}

				let sheet_id: u16 = resolve_sheet_tile_id(kind, frame_index / 10, tx, ty);
				if sheet_id == 0 {
					continue;
				}

				let x = tx * tile_w_i32 - cam_x;
				let y = ty * tile_h_i32 - cam_y;

				backend.draw_sheet_tile(sheet_id, x, y, tile_w, tile_h);
			}
		}
	}
}
