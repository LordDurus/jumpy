use crate::{
	game::game_state::{EntityId, GameState},
	physics::{
		collision::{resolve_ceiling_collision, resolve_floor_collision, resolve_wall_collision},
		constants::JUMP_VELOCITY,
	},
};

#[inline(always)]
pub fn move_and_collide(game_state: &mut GameState) {
	let ids: Vec<EntityId> = game_state.positions.keys().copied().collect();

	let tile_w: f32 = game_state.level.tile_width as f32;
	let tile_h: f32 = game_state.level.tile_height as f32;

	let level_w_px: f32 = (game_state.level.width as f32) * tile_w;
	let level_h_px: f32 = (game_state.level.height as f32) * tile_h;

	let margin: f32 = 64.0;

	let player_id: EntityId = game_state.get_player_id();

	for id in ids {
		let is_player: bool = id == player_id;

		// do movement + collision inside a scope so &mut borrows drop
		{
			let (half_w, half_h) = game_state.get_entity_half_values(id);

			let Some(pos) = game_state.positions.get_mut(&id) else {
				continue;
			};
			let Some(vel) = game_state.velocities.get_mut(&id) else {
				continue;
			};

			pos.x += vel.x;
			pos.y += vel.y;

			resolve_wall_collision(&game_state.level, pos, vel, half_w, half_h);
			resolve_ceiling_collision(&game_state.level, pos, vel, half_w, half_h);
			resolve_floor_collision(&game_state.level, pos, vel, half_w, half_h);
		} // <- pos/vel borrows end here

		// now it's legal to query game_state immutably
		if is_player && game_state.on_ground(id) && game_state.on_ground_safe(id) {
			let Some(pos) = game_state.positions.get(&id) else {
				continue;
			};
			game_state.last_grounded_pos = Some(*pos);
		}

		// compute out-of-bounds without holding &mut borrows
		let (half_w, half_h) = game_state.get_entity_half_values(id);
		let Some(pos) = game_state.positions.get(&id) else {
			continue;
		};

		let left: f32 = pos.x - half_w;
		let right: f32 = pos.x + half_w;
		let top: f32 = pos.y - half_h;
		let bottom: f32 = pos.y + half_h;

		let out: bool = right < -margin || left > level_w_px + margin || bottom < -margin || top > level_h_px + margin;

		if out {
			if is_player {
				game_state.respawn_player();
				continue;
			} else {
				game_state.remove_entity(id);
				continue;
			}
		}
	}
}

pub fn try_jump(game_state: &mut GameState, entity_id: EntityId) -> bool {
	let grounded: bool = game_state.on_ground(entity_id);
	let on_left: bool = game_state.on_wall_left(entity_id);
	let on_right: bool = game_state.on_wall_right(entity_id);

	if !grounded && !on_left && !on_right {
		return false;
	}

	if let Some(vel) = game_state.velocities.get_mut(&entity_id) {
		let jump_multiplier: f32 = *game_state.jump_multipliers.get(entity_id).unwrap_or(&1) as f32;

		vel.y = JUMP_VELOCITY * jump_multiplier;

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

	return false;
}
