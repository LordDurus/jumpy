use crate::{engine_math::Vec2, game::level::Level};

pub fn resolve_ceiling_collision(level: &Level, pos: &mut Vec2, vel: &mut Vec2, half_w: f32, half_h: f32) {
	if vel.y >= 0.0 {
		return;
	}

	let layer: u32 = level.collision_layer_index() as u32;

	let tile_w: f32 = level.tile_width as f32;
	let tile_h: f32 = level.tile_height as f32;

	let top_y: f32 = pos.y - half_h;

	// probe slightly above head to detect ceiling reliably
	let probe_y: f32 = top_y - 0.5;

	let ty: i32 = (probe_y / tile_h).floor() as i32;

	// inset so we don't catch tiles when just barely touching corners
	let inset_x: f32 = 0.5;

	let left_x: f32 = pos.x - half_w + inset_x;
	let right_x: f32 = pos.x + half_w - inset_x;

	let tx_left: i32 = (left_x / tile_w).floor() as i32;
	let tx_right: i32 = (right_x / tile_w).floor() as i32;

	let hit: bool = level.tile_at_layer(layer, tx_left, ty).is_solid() || level.tile_at_layer(layer, tx_right, ty).is_solid();

	if hit {
		// snap player just below the ceiling tile
		let tile_bottom: f32 = ((ty + 1) as f32) * tile_h;
		pos.y = tile_bottom + half_h;
		vel.y = 0.0;
	}

	return;
}

pub fn resolve_floor_collision(level: &Level, pos: &mut Vec2, vel: &mut Vec2, half_w: f32, half_h: f32) {
	if vel.y < 0.0 {
		return;
	}

	let layer: u32 = level.collision_layer_index() as u32;

	let tile_w: f32 = level.tile_width as f32;
	let tile_h: f32 = level.tile_height as f32;

	let bottom_y: f32 = pos.y + half_h;

	// probe slightly below feet to detect ground reliably
	let probe_y: f32 = bottom_y + 0.5;

	let ty: i32 = (probe_y / tile_h).floor() as i32;
	let inset: f32 = 0.5;
	let test_left: i32 = ((pos.x - half_w + inset) / tile_w).floor() as i32;
	let test_right: i32 = ((pos.x + half_w - inset) / tile_w).floor() as i32;

	let hit: bool = level.tile_at_layer(layer, test_left, ty).is_solid() || level.tile_at_layer(layer, test_right, ty).is_solid();

	if !hit {
		return;
	}

	let tile_top: f32 = (ty as f32) * tile_h;
	pos.y = tile_top - half_h;
	vel.y = 0.0;
}

pub fn resolve_wall_collision(level: &Level, pos: &mut Vec2, vel: &mut Vec2, half_w: f32, half_h: f32) {
	if vel.x == 0.0 {
		return;
	}

	let layer: u32 = level.collision_layer_index() as u32;

	let tile_w: f32 = level.tile_width as f32;
	let tile_h: f32 = level.tile_height as f32;

	// sample three points along the side
	let inset_y: f32 = 0.5;
	let y_top: f32 = pos.y - half_h + inset_y;
	let y_mid: f32 = pos.y;
	let y_bot: f32 = pos.y + half_h - inset_y;

	let ty_top: i32 = (y_top / tile_h).floor() as i32;
	let ty_mid: i32 = (y_mid / tile_h).floor() as i32;
	let ty_bot: i32 = (y_bot / tile_h).floor() as i32;

	let inset_x: f32 = 0.5;

	if vel.x > 0.0 {
		// moving right
		let probe_x: f32 = pos.x + half_w + inset_x;
		let tx: i32 = (probe_x / tile_w).floor() as i32;

		let hit: bool =
			level.tile_at_layer(layer, tx, ty_top).is_solid() || level.tile_at_layer(layer, tx, ty_mid).is_solid() || level.tile_at_layer(layer, tx, ty_bot).is_solid();

		/*
		println!(
			"wall hit: pos=({}, {}) vel=({}, {}) half=({}, {}) tx={} ty_top={} ty_mid={} ty_bot={}",
			pos.x, pos.y, vel.x, vel.y, half_w, half_h, tx, ty_top, ty_mid, ty_bot
		);
		*/

		if hit {
			let tile_left: f32 = (tx as f32) * tile_w;
			pos.x = tile_left - half_w;
			vel.x = 0.0;
		}

		return;
	}

	// moving left
	let probe_x: f32 = pos.x - half_w - inset_x;
	let tx: i32 = (probe_x / tile_w).floor() as i32;

	let hit: bool =
		level.tile_at_layer(layer, tx, ty_top).is_solid() || level.tile_at_layer(layer, tx, ty_mid).is_solid() || level.tile_at_layer(layer, tx, ty_bot).is_solid();

	if hit {
		let tile_right: f32 = ((tx + 1) as f32) * tile_w;
		pos.x = tile_right + half_w;
		vel.x = 0.0;
	}

	return;
}

pub fn scan_down_to_ground(level: &Level, pos: &mut Vec2, half_width: f32, half_height: f32, max_scan_tiles: i32) -> bool {
	let layer: u32 = level.collision_layer_index() as u32;

	let tile_w: f32 = level.tile_width as f32;
	let tile_h: f32 = level.tile_height as f32;

	// start from the entity's feet (a tiny bit below so we don't miss due to float rounding)
	let start_y: f32 = pos.y + half_height + 0.5;
	let mut ty: i32 = (start_y / tile_h).floor() as i32;

	// match your existing inset style
	let inset_x: f32 = 0.5;
	let tx_left: i32 = ((pos.x - half_width + inset_x) / tile_w).floor() as i32;
	let tx_right: i32 = ((pos.x + half_width - inset_x) / tile_w).floor() as i32;

	let min_ty: i32 = 0;
	let max_ty: i32 = level.height as i32 - 1;

	let mut steps: i32 = 0;
	while steps <= max_scan_tiles && ty <= max_ty {
		if ty >= min_ty {
			let hit: bool = level.tile_at_layer(layer, tx_left, ty).is_solid() || level.tile_at_layer(layer, tx_right, ty).is_solid();

			if hit {
				// snap entity so its feet are on top of this tile row
				let tile_top: f32 = (ty as f32) * tile_h;
				pos.y = tile_top - half_height;
				return true;
			}
			ty += 1;
			steps += 1;
		}

		// ty += 1;
		// steps += 1;
	}

	return false;
}
