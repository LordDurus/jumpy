use crate::{
	game::game_state::{EntityId, GameState},
	physics::{
		collision::{resolve_floor_collision, resolve_wall_collision},
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
		resolve_wall_collision(&world.level, pos, vel, ENTITY_HALF_W, ENTITY_HALF_HEIGHT);
		resolve_floor_collision(&world.level, pos, vel, ENTITY_HALF_W, ENTITY_HALF_HEIGHT);
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
