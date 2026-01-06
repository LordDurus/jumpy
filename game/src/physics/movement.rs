use crate::{
	game::game_state::{EntityId, GameState},
	physics::{
		collision::{resolve_ceiling_collision, resolve_floor_collision, resolve_wall_collision},
		constants::JUMP_VELOCITY,
	},
};

#[inline(always)]
pub fn move_and_collide(game_state: &mut GameState) {
	let do_debug: bool = (game_state.tick % 60) == 0;

	let ids: Vec<EntityId> = game_state.positions.keys().copied().collect();

	let tile_width: f32 = game_state.level.tile_width as f32;
	let tile_height: f32 = game_state.level.tile_height as f32;

	let level_width_pixels: f32 = (game_state.level.width as f32) * tile_width;
	let level_height_pixels: f32 = (game_state.level.height as f32) * tile_height;

	let margin: f32 = 64.0;
	let player_id: EntityId = game_state.get_player_id();

	for id in ids {
		let is_player: bool = id == player_id;
		let mut hit_wall: bool = false;
		let old_vx: f32;
		// do movement + collision inside a scope so &mut borrows drop
		{
			let (half_width, half_height) = game_state.get_entity_half_values(id);

			let Some(postion) = game_state.positions.get_mut(&id) else {
				continue;
			};
			let Some(velocity) = game_state.velocities.get_mut(&id) else {
				continue;
			};

			postion.x += velocity.x;
			postion.y += velocity.y;

			old_vx = velocity.x;

			if do_debug && id == player_id {
				let tile_w: f32 = game_state.level.tile_width as f32;
				let tile_h: f32 = game_state.level.tile_height as f32;

				let tx: i32 = (postion.x / tile_w).floor() as i32;
				let ty: i32 = (postion.y / tile_h).floor() as i32;

				let layer: u32 = game_state.level.get_action_layer_index() as u32;
				let k = game_state.level.get_tile_at_layer(layer, tx, ty);

				println!("  player tile tx={} ty={} kind={:?} pos=({}, {})", tx, ty, k, postion.x, postion.y);
				println!("  feet tile tx={} ty={} id={}", tx, ty, id);
			}

			resolve_wall_collision(&game_state.level, postion, velocity, half_width, half_height, do_debug);

			if old_vx != 0.0 && velocity.x == 0.0 {
				hit_wall = true;
			}

			resolve_ceiling_collision(&game_state.level, postion, velocity, half_width, half_height);
			resolve_floor_collision(&game_state.level, postion, velocity, half_width, half_height);
		} // <- pos/vel borrows end here

		// ai response: flip slimes on wall hit
		if hit_wall {
			let kind: u8 = *game_state.entity_kinds.get(&id).unwrap_or(&0);

			// kind==2 is slime in your data
			if kind == 2 {
				if let Some(vel) = game_state.velocities.get_mut(&id) {
					vel.x = -old_vx;
				}
			}
		}

		// now it's legal to query game_state immutably
		if is_player && game_state.on_ground(id) && game_state.on_ground_safe(id) {
			let Some(pos) = game_state.positions.get(&id) else {
				continue;
			};
			game_state.last_grounded_pos = Some(*pos);
		}

		let (half_width, half_height) = game_state.get_entity_half_values(id);
		// clamp to world bounds (x only for now)
		{
			let Some(pos) = game_state.positions.get_mut(&id) else {
				continue;
			};
			let Some(vel) = game_state.velocities.get_mut(&id) else {
				continue;
			};

			let min_x: f32 = half_width;
			let max_x: f32 = (level_width_pixels - half_width).max(min_x);

			if pos.x < min_x {
				pos.x = min_x;
				if !is_player {
					vel.x = vel.x.abs();
				} else {
					vel.x = 0.0;
				}
			}

			if pos.x > max_x {
				pos.x = max_x;
				if !is_player {
					vel.x = -vel.x.abs();
				} else {
					vel.x = 0.0;
				}
			}
		}

		// compute out-of-bounds without holding &mut borrows
		let Some(pos) = game_state.positions.get(&id) else {
			continue;
		};

		let left: f32 = pos.x - half_width;
		let right: f32 = pos.x + half_width;
		let top: f32 = pos.y - half_height;
		let bottom: f32 = pos.y + half_height;

		let out: bool = right < -margin || left > level_width_pixels + margin || bottom < -margin || top > level_height_pixels + margin;

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
