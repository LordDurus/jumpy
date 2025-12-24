use crate::{
	game::game_state::{EntityId, GameState},
	physics::{
		collision::{resolve_ceiling_collision, resolve_floor_collision, resolve_wall_collision},
		constants::{ENTITY_HALF_HEIGHT, ENTITY_HALF_W, JUMP_VELOCITY},
	},
};

#[inline(always)]
pub fn move_and_collide(game_state: &mut GameState) {
	let ids: Vec<EntityId> = game_state.positions.keys().copied().collect();

	for id in ids {
		// immutable borrow FIRST
		let (half_width, half_height) = game_state.entity_half_extents(id);

		// now mutable borrows
		let Some(pos) = game_state.positions.get_mut(&id) else {
			continue;
		};
		let Some(vel) = game_state.velocities.get_mut(&id) else {
			continue;
		};

		// integrate
		pos.x += vel.x;
		pos.y += vel.y;

		// collide
		resolve_wall_collision(&game_state.level, pos, vel, half_width, half_height);
		resolve_ceiling_collision(&game_state.level, pos, vel, half_width, half_height);
		resolve_floor_collision(&game_state.level, pos, vel, half_width, half_height);
	}
}

pub fn try_jump(world: &mut GameState, entity_id: EntityId) -> bool {
	let grounded = world.on_ground(entity_id);
	let on_left = world.on_wall_left(entity_id);
	let on_right = world.on_wall_right(entity_id);

	if !grounded && !on_left && !on_right {
		return false;
	}

	if let Some(vel) = world.velocities.get_mut(&entity_id) {
		vel.y = JUMP_VELOCITY;

		if !grounded {
			let wall_push: f32 = 2.5;
			if on_left {
				vel.x = wall_push;
			} else if on_right {
				vel.x = -wall_push;
			}
		}

		return true;
	}

	false
}
