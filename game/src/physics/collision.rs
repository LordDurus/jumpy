use crate::{engine_math::Vec2, game::level::Level};

pub fn resolve_ceiling_collision(level: &Level, pos: &mut Vec2, vel: &mut Vec2, half_w: f32, half_h: f32) {
	if vel.y >= 0.0 {
		return;
	}

	let layer: u32 = level.get_action_layer_index() as u32;

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

	let hit: bool = level.get_tile_at_layer(layer, tx_left, ty).is_solid() || level.get_tile_at_layer(layer, tx_right, ty).is_solid();

	if hit {
		// snap player just below the ceiling tile
		let tile_bottom: f32 = ((ty + 1) as f32) * tile_h;
		pos.y = tile_bottom + half_h;
		vel.y = 0.0;
	}

	return;
}

pub fn resolve_floor_collision(level: &Level, pos: &mut Vec2, vel: &mut Vec2, half_width: f32, half_height: f32) {
	if vel.y < 0.0 {
		return;
	}

	let layer: u32 = level.get_action_layer_index() as u32;

	let tile_w: f32 = level.tile_width as f32;
	let tile_h: f32 = level.tile_height as f32;

	let bottom_y: f32 = pos.y + half_height;

	// probe slightly below feet to detect ground reliably
	let probe_y: f32 = bottom_y + 0.5;

	let ty: i32 = (probe_y / tile_h).floor() as i32;
	let inset: f32 = 0.5;
	let test_left: i32 = ((pos.x - half_width + inset) / tile_w).floor() as i32;
	let test_right: i32 = ((pos.x + half_width - inset) / tile_w).floor() as i32;

	let hit: bool = level.get_tile_at_layer(layer, test_left, ty).is_solid() || level.get_tile_at_layer(layer, test_right, ty).is_solid();

	if !hit {
		return;
	}

	let tile_top: f32 = (ty as f32) * tile_h;
	pos.y = tile_top - half_height;
	vel.y = 0.0;
}

pub fn resolve_wall_collision(level: &Level, postion: &mut Vec2, velocity: &mut Vec2, half_width: f32, half_h: f32, _is_player: bool) {
	if velocity.x == 0.0 {
		return;
	}

	let layer: u32 = level.get_action_layer_index() as u32;

	let tile_width: f32 = level.tile_width as f32;
	let tile_height: f32 = level.tile_height as f32;

	let inset_left: f32 = 0.5;

	let top_left: f32 = postion.y - half_h + inset_left;
	let middle_left: f32 = postion.y;
	let bottom_left: f32 = postion.y + half_h - inset_left;

	let probe_x: f32 = if velocity.x > 0.0 {
		postion.x + half_width + 0.5
	} else {
		postion.x - half_width - 0.5
	};

	let tx: i32 = (probe_x / tile_width).floor() as i32;
	let ty_top: i32 = (top_left / tile_height).floor() as i32;
	let ty_middle: i32 = (middle_left / tile_height).floor() as i32;
	let ty_bottom: i32 = (bottom_left / tile_height).floor() as i32;

	let hit: bool = level.get_tile_id_at_layer(layer, tx, ty_top) != 0
		|| level.get_tile_id_at_layer(layer, tx, ty_middle) != 0
		|| level.get_tile_id_at_layer(layer, tx, ty_bottom) != 0;

	if hit {
		if velocity.x > 0.0 {
			// snap to left edge of that tile
			let tile_left: f32 = (tx as f32) * tile_width;
			postion.x = tile_left - half_width;
		} else {
			// snap to right edge of that tile
			let tile_right: f32 = ((tx + 1) as f32) * tile_width;
			postion.x = tile_right + half_width;
		}
		velocity.x = 0.0;
	}

	return;
}

pub fn scan_down_to_ground(level: &Level, pos: &mut Vec2, half_width: f32, half_height: f32, max_scan_tiles: i32) -> bool {
	let layer: u32 = level.get_action_layer_index() as u32;

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
			let hit: bool = level.get_tile_at_layer(layer, tx_left, ty).is_solid() || level.get_tile_at_layer(layer, tx_right, ty).is_solid();

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
