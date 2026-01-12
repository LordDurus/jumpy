use crate::{engine_math::Vec2, game::level::Level, tile::TileCollision};

pub fn resolve_ceiling_collision(level: &Level, position: &mut Vec2, velocity: &mut Vec2, half_width: f32, half_height: f32) {
	if velocity.y >= 0.0 {
		return;
	}

	let layer: u32 = level.get_action_layer_index() as u32;
	let tile_w: f32 = level.tile_width as f32;
	let tile_h: f32 = level.tile_height as f32;

	let top_y: f32 = position.y - half_height;

	// probe slightly above head to detect ceiling reliably
	let probe_top: f32 = top_y - 0.5;

	let ty: i32 = (probe_top / tile_h).floor() as i32;

	// inset so we don't catch tiles when just barely touching corners
	let inset_x: f32 = 0.5;
	let left_x: f32 = position.x - half_width + inset_x;
	let right_x: f32 = position.x + half_width - inset_x;

	let tx_left: i32 = (left_x / tile_w).floor() as i32;
	let tx_right: i32 = (right_x / tile_w).floor() as i32;
	let tile_bottom: f32 = ((ty + 1) as f32) * tile_h;
	let hit: bool = level.get_tile_at_layer(layer, tx_left, ty).is_solid() || level.get_tile_at_layer(layer, tx_right, ty).is_solid();

	if hit {
		// snap player just below the ceiling tile
		position.y = tile_bottom + half_height;
		velocity.y = 0.0;
	}

	return;
}

pub fn resolve_floor_collision(level: &Level, pos: &mut Vec2, vel: &mut Vec2, half_width: f32, half_height: f32, prev_bottom_level: f32) {
	if vel.y <= 0.0 {
		return;
	}

	let layer: u32 = level.get_action_layer_index() as u32;
	let tile_width: f32 = level.tile_width as f32;
	let tile_height: f32 = level.tile_height as f32;
	let bottom: f32 = pos.y + half_height;
	let probe_bottom: f32 = bottom + 0.5;
	let tile_top: i32 = (probe_bottom / tile_height).floor() as i32;
	let inset: f32 = 0.5;
	let tile_left: i32 = ((pos.x - half_width + inset) / tile_width).floor() as i32;
	let tile_right: i32 = ((pos.x + half_width - inset) / tile_width).floor() as i32;

	let mut hit_ground: bool = false;
	let mut ground_top: f32 = 0.0;

	for tx in tile_left..=tile_right {
		let tile = level.get_tile_at_layer(layer, tx, tile_top);

		let kind: TileCollision = tile.get_collision_kind();
		if kind == TileCollision::None {
			continue;
		}

		let tile_surface: f32 = (tile_top as f32) * tile_height;

		if kind == TileCollision::Solid {
			hit_ground = true;
			ground_top = tile_surface;
			break;
		}

		if kind == TileCollision::OneWay {
			// only land if we crossed the platform surface this frame
			if prev_bottom_level <= tile_surface && bottom >= tile_surface {
				hit_ground = true;
				ground_top = tile_surface;
				break;
			}
		}
	}

	if hit_ground {
		pos.y = ground_top - half_height;
		vel.y = 0.0;
		return;
	}

	return;
}

pub fn resolve_wall_collision(level: &Level, position: &mut Vec2, velocity: &mut Vec2, half_width: f32, half_h: f32, _is_player: bool) {
	if velocity.x == 0.0 {
		return;
	}

	let layer: u32 = level.get_action_layer_index() as u32;

	let tile_width: f32 = level.tile_width as f32;
	let tile_height: f32 = level.tile_height as f32;

	let inset_left: f32 = 0.5;

	let top_left: f32 = position.y - half_h + inset_left;
	let middle_left: f32 = position.y;
	let bottom_left: f32 = position.y + half_h - inset_left;

	let probe_x: f32 = if velocity.x > 0.0 {
		position.x + half_width + 0.5
	} else {
		position.x - half_width - 0.5
	};

	let tx: i32 = (probe_x / tile_width).floor() as i32;
	let ty_top: i32 = (top_left / tile_height).floor() as i32;
	let ty_middle: i32 = (middle_left / tile_height).floor() as i32;
	let ty_bottom: i32 = (bottom_left / tile_height).floor() as i32;
	let hit: bool;
	let kind = level.get_tile_at_layer(layer, tx, ty_middle).get_collision_kind();

	if kind == TileCollision::None {
		hit = false;
	} else {
		hit = level.get_tile_id_at_layer(layer, tx, ty_top) != 0
			|| level.get_tile_id_at_layer(layer, tx, ty_middle) != 0
			|| level.get_tile_id_at_layer(layer, tx, ty_bottom) != 0;
	}

	if hit {
		if velocity.x > 0.0 {
			// snap to left edge of that tile
			let tile_left: f32 = (tx as f32) * tile_width;
			position.x = tile_left - half_width;
		} else {
			// snap to right edge of that tile
			let tile_right: f32 = ((tx + 1) as f32) * tile_width;
			position.x = tile_right + half_width;
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HitSide {
	Top,
	Right,
	Bottom,
	Left,
}

#[inline(always)]
pub fn classify_aabb_hit_side(prev_left: f32, prev_right: f32, prev_top: f32, prev_bottom: f32, left: f32, right: f32, top: f32, bottom: f32) -> HitSide {
	// prefer deterministic classification based on prior frame
	if prev_bottom <= top {
		return HitSide::Top;
	}
	if prev_top >= bottom {
		return HitSide::Bottom;
	}
	if prev_right <= left {
		return HitSide::Left;
	}
	if prev_left >= right {
		return HitSide::Right;
	}

	// fallback: smallest penetration (rare: spawns/teleports)
	let overlap_left: f32 = right - prev_left;
	let overlap_right: f32 = prev_right - left;
	let overlap_top: f32 = bottom - prev_top;
	let overlap_bottom: f32 = prev_bottom - top;

	let push_x: f32 = if overlap_left < overlap_right { overlap_left } else { -overlap_right };
	let push_y: f32 = if overlap_top < overlap_bottom { overlap_top } else { -overlap_bottom };

	if push_x.abs() < push_y.abs() {
		if push_x > 0.0 {
			return HitSide::Left;
		}
		return HitSide::Right;
	}

	if push_y > 0.0 {
		return HitSide::Top;
	}
	return HitSide::Bottom;
}
