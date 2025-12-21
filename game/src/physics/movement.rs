use crate::{
	game::game_state::{EntityId, GameState},
	physics::{
		collision::resolve_floor_collision,
		constants::{ENTITY_HALF_HEIGHT, ENTITY_HALF_W, JUMP_VELOCITY},
	},
};

#[inline(always)]
pub fn move_and_collide(world: &mut GameState) {
	let ids: Vec<EntityId> = world.positions.keys().copied().collect();

	for id in ids {
		let Some(pos) = world.positions.get_mut(&id) else {
			continue;
		};
		let Some(vel) = world.velocities.get_mut(&id) else {
			continue;
		};

		// integrate
		pos.x += vel.x;
		pos.y += vel.y;

		// collide
		resolve_floor_collision(&world.level, pos, vel, ENTITY_HALF_W, ENTITY_HALF_HEIGHT);
	}
}

pub fn try_jump(world: &mut GameState, entity_id: EntityId) -> bool {
	let pos = match world.positions.get(&entity_id) {
		Some(p) => *p,
		None => return false,
	};

	let (half_w, half_h) = world.entity_half_extents(entity_id);

	let foot_y = pos.y + half_h + 1.0;
	let left_x = pos.x - half_w + 1.0;
	let right_x = pos.x + half_w - 1.0;

	let grounded = world.level.is_solid_world_f32(left_x, foot_y) || world.level.is_solid_world_f32(right_x, foot_y);

	if !grounded {
		return false;
	}

	// IMPORTANT: borrow velocity ONLY at the end
	if let Some(vel) = world.velocities.get_mut(&entity_id) {
		vel.y = JUMP_VELOCITY;
		return true;
	}

	return false;
}
