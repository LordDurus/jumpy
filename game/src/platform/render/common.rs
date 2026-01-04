use super::backend::RenderBackend;
use crate::{
	game::{game_state::GameState, level},
	tile::TileKind,
};

pub struct RenderCommon;

impl RenderCommon {
	pub fn new() -> RenderCommon {
		return RenderCommon;
	}

	// camera is in world pixels (unscaled)
	pub fn compute_camera<B: RenderBackend>(&self, backend: &B, world: &GameState) -> (i32, i32) {
		let (screen_w_px, screen_h_px) = backend.screen_size();
		let scale: f32 = backend.render_scale();

		let screen_w_world: i32 = ((screen_w_px as f32) / scale).round() as i32;
		let screen_h_world: i32 = ((screen_h_px as f32) / scale).round() as i32;

		let mut focus_x: f32 = 0.0;
		let mut focus_y: f32 = 0.0;

		let focus_id: Option<u32> = if world.get_player_id() != 0 {
			Some(world.get_player_id())
		} else {
			let mut best_id: Option<u32> = None;
			for id in world.positions.keys() {
				if best_id.is_none() || *id < best_id.unwrap() {
					best_id = Some(*id);
				}
			}
			best_id
		};

		if let Some(id) = focus_id {
			if let Some(p) = world.positions.get(&id) {
				focus_x = p.x;
				focus_y = p.y;
			}
		}

		let tile_w: i32 = world.level.tile_width as i32;
		let tile_h: i32 = world.level.tile_height as i32;

		let level_w_px: i32 = (world.level.width as i32) * tile_w;
		let level_h_px: i32 = (world.level.height as i32) * tile_h;

		let mut cam_x: i32 = focus_x as i32 - (screen_w_world / 2);
		let mut cam_y: i32 = focus_y as i32 - (screen_h_world / 2);

		let max_x: i32 = (level_w_px - screen_w_world).max(0);
		let max_y: i32 = (level_h_px - screen_h_world).max(0);

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

	pub fn draw_level<B: RenderBackend>(&self, backend: &mut B, game_state: &GameState, cam_x: i32, cam_y: i32, frame_index: u32, layer: u32) {
		let (screen_w_px, screen_h_px) = backend.screen_size();
		let scale: f32 = backend.render_scale();

		let screen_w_world: i32 = ((screen_w_px as f32) / scale).round() as i32;
		let screen_h_world: i32 = ((screen_h_px as f32) / scale).round() as i32;

		let tile_w_world: i32 = game_state.level.tile_width as i32;
		let tile_h_world: i32 = game_state.level.tile_height as i32;

		// visible tiles in world space
		let view_left: i32 = cam_x / tile_w_world;
		let view_top: i32 = cam_y / tile_h_world;
		let view_right: i32 = (cam_x + screen_w_world + tile_w_world - 1) / tile_w_world;
		let view_bottom: i32 = (cam_y + screen_h_world + tile_h_world - 1) / tile_h_world;

		let max_x: i32 = game_state.level.width as i32;
		let max_y: i32 = game_state.level.height as i32;

		let x0: i32 = view_left.max(0);
		let y0: i32 = view_top.max(0);
		let x1: i32 = view_right.min(max_x);
		let y1: i32 = view_bottom.min(max_y);

		for ty in y0..y1 {
			for tx in x0..x1 {
				let kind = game_state.level.get_tile_at_layer(layer, tx, ty);

				if kind == crate::tile::TileKind::Empty {
					continue;
				}

				let sheet_id: u16 = resolve_sheet_tile_id(kind, frame_index / 10, tx, ty);
				if sheet_id == 0 {
					continue;
				}

				//backend.draw_world(world);
				let world_x: i32 = tx * tile_w_world;
				let world_y: i32 = ty * tile_h_world;

				let sx: i32 = (((world_x - cam_x) as f32) * scale).round() as i32;
				let sy: i32 = (((world_y - cam_y) as f32) * scale).round() as i32;

				let w_px: u32 = ((game_state.level.tile_width as f32) * scale).round() as u32;
				let h_px: u32 = ((game_state.level.tile_height as f32) * scale).round() as u32;
				backend.draw_tile(sheet_id, sx, sy, w_px, h_px);
			}
		}
	}
}

pub fn resolve_sheet_tile_id(kind: TileKind, frame_index: u32, tile_x: i32, tile_y: i32) -> u16 {
	match kind {
		TileKind::Empty => return 0,
		TileKind::Dirt => return 24,

		TileKind::GrassTop => {
			let ids: [u16; 3] = [0, 1, 2];
			let idx: usize = ((tile_x + tile_y) as usize) % 3;
			return ids[idx];
		}

		TileKind::Water => {
			let ids: [u16; 4] = [14, 17, 38, 40];
			let idx: usize = (frame_index as usize) & 3;
			return ids[idx];
		}

		TileKind::SpikeUp => return 78,
		TileKind::SpikeDown => return 6,
		TileKind::SpikeLeft => return 30,
		TileKind::SpikeRight => return 54,
	}
}
