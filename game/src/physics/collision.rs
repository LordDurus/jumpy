use crate::{engine_math::MyVector2 as Vec2, game::level::Level};

pub fn resolve_floor_collision(level: &Level, pos: &mut Vec2, vel: &mut Vec2, half_w: f32, half_h: f32) {
	// only care if moving downward (positive y is down)
	if vel.y <= 0.0 {
		return;
	}

	let foot_y = pos.y + half_h;
	let left_x = pos.x - half_w + 1.0;
	let right_x = pos.x + half_w - 1.0;

	if level.is_solid_world_f32(left_x, foot_y) || level.is_solid_world_f32(right_x, foot_y) {
		let ty = (foot_y / level.tile_height as f32).floor();
		let top_of_tile = ty * level.tile_height as f32;

		pos.y = top_of_tile - half_h;
		vel.y = 0.0;
	}
}
