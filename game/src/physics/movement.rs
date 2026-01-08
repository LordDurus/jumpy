use crate::{
	engine_math::Vec2,
	game::{
		game_state::{EntityId, EntityKind, GameState},
		level::Level,
	},
	physics::{
		collision::{resolve_ceiling_collision, resolve_floor_collision, resolve_wall_collision},
		constants::JUMP_VELOCITY,
	},
};

#[derive(Clone, Copy)]
struct PlatformAabb {
	left: f32,
	right: f32,
	top: f32,
	bottom: f32,
	vx: f32,
}

#[inline(always)]
fn resolve_platform_pushout(level: &Level, pos: &mut Vec2, vel: &mut Vec2, prev_pos: Vec2, half_width: f32, half_height: f32, platforms: &[PlatformAabb]) {
	let left: f32 = pos.x - half_width;
	let right: f32 = pos.x + half_width;
	let top: f32 = pos.y - half_height;
	let bottom: f32 = pos.y + half_height;

	let prev_left: f32 = prev_pos.x - half_width;
	let prev_right: f32 = prev_pos.x + half_width;
	let prev_top: f32 = prev_pos.y - half_height;
	let prev_bottom: f32 = prev_pos.y + half_height;

	for p in platforms {
		// broadphase
		if right <= p.left || left >= p.right || bottom <= p.top || top >= p.bottom {
			continue;
		}

		// platform swept bounds (where it was last frame)
		let p_prev_left: f32 = p.left - p.vx;
		let p_prev_right: f32 = p.right - p.vx;

		// --- side push from moving platform (this is what you’re missing) ---
		if p.vx > 0.0 {
			// platform moved right into entity
			if prev_left >= p_prev_right && left < p.right {
				pos.x = p.right + half_width;
				if vel.x < p.vx {
					vel.x = p.vx;
				}
				continue;
			}
		} else if p.vx < 0.0 {
			// platform moved left into entity
			if prev_right <= p_prev_left && right > p.left {
				pos.x = p.left - half_width;
				if vel.x > p.vx {
					vel.x = p.vx;
				}
				continue;
			}
		}

		// --- regular side hits (entity walks into platform) ---
		if prev_right <= p.left {
			pos.x = p.left - half_width;
			if vel.x > 0.0 {
				vel.x = 0.0;
			}
			continue;
		}
		if prev_left >= p.right {
			pos.x = p.right + half_width;
			if vel.x < 0.0 {
				vel.x = 0.0;
			}
			continue;
		}

		// --- ceiling / underside ---
		if prev_top >= p.bottom {
			pos.y = p.bottom + half_height;
			if vel.y < 0.0 {
				vel.y = 0.0;
			}
			continue;
		}

		// --- landing on top (only if falling) ---
		if vel.y > 0.0 && prev_bottom <= p.top {
			pos.y = p.top - half_height;
			vel.y = 0.0;

			// carry
			pos.x += p.vx;

			// IMPORTANT: re-resolve wall collision after carry so you don’t shove into tiles
			resolve_wall_collision(level, pos, vel, half_width, half_height, false);

			continue;
		}
	}
}

#[inline(always)]
pub fn move_and_collide(game_state: &mut GameState) {
	let max_id: usize = game_state.positions.len();
	let tile_width: f32 = game_state.level.tile_width as f32;
	let tile_height: f32 = game_state.level.tile_height as f32;
	let level_width_pixels: f32 = (game_state.level.width as f32) * tile_width;
	let level_height_pixels: f32 = (game_state.level.height as f32) * tile_height;
	let margin: f32 = 64.0;
	let player_id: EntityId = game_state.get_player_id();

	let mut platform_tops: Vec<(f32, f32, f32, f32)> = Vec::new(); // left, right, top, vx

	let mut platforms: Vec<PlatformAabb> = Vec::new();

	for (pid, ppos) in game_state.positions.iter() {
		let kind_u8: u8 = *game_state.entity_kinds.get(pid).unwrap_or(&0);
		if EntityKind::from_u8(kind_u8) != EntityKind::MovingPlatform {
			continue;
		}

		let (ph_width, ph_height) = game_state.get_entity_half_values(pid);
		let velocity_x: f32 = game_state.velocities.get(pid).map(|v| v.x).unwrap_or(0.0);

		platforms.push(PlatformAabb {
			left: ppos.x - ph_width,
			right: ppos.x + ph_width,
			top: ppos.y - ph_height,
			bottom: ppos.y + ph_height,
			vx: velocity_x,
		});
	}

	for (entity_id, position) in game_state.positions.iter() {
		let kind_u8: u8 = *game_state.entity_kinds.get(entity_id).unwrap_or(&0);
		if EntityKind::from_u8(kind_u8) != EntityKind::MovingPlatform {
			continue;
		}

		let (phw, phh) = game_state.get_entity_half_values(entity_id);
		let vx: f32 = game_state.velocities.get(entity_id).map(|v| v.x).unwrap_or(0.0);

		let left: f32 = position.x - phw;
		let right: f32 = position.x + phw;
		let top: f32 = position.y - phh;

		platform_tops.push((left, right, top, vx));
	}

	for index in 0..max_id {
		let id: EntityId = index as EntityId;

		if !game_state.positions.has(id) {
			continue;
		}

		let is_player: bool = id == player_id;

		let mut hit_wall: bool = false;
		let old_vx: f32;
		// do movement + collision inside a scope so &mut borrows drop
		{
			let (half_width, half_height) = game_state.get_entity_half_values(id);

			let Some(postion) = game_state.positions.get_mut(id) else {
				continue;
			};
			let Some(velocity) = game_state.velocities.get_mut(id) else {
				continue;
			};

			let prev_pos: Vec2 = *postion;
			let prev_bottom_world: f32 = postion.y + half_height;
			postion.x += velocity.x;
			postion.y += velocity.y;

			old_vx = velocity.x;

			resolve_wall_collision(&game_state.level, postion, velocity, half_width, half_height, false);

			if old_vx != 0.0 && velocity.x == 0.0 {
				hit_wall = true;
			}

			resolve_ceiling_collision(&game_state.level, postion, velocity, half_width, half_height);
			resolve_floor_collision(&game_state.level, postion, velocity, half_width, half_height, prev_bottom_world);

			let kind_u8: u8 = *game_state.entity_kinds.get(id).unwrap_or(&0);
			let kind: EntityKind = EntityKind::from_u8(kind_u8);

			// don’t push a platform out of itself
			if kind != EntityKind::MovingPlatform {
				resolve_platform_pushout(&game_state.level, postion, velocity, prev_pos, half_width, half_height, &platforms);
			}

			if velocity.y > 0.0 {
				let inset_x: f32 = 0.5;

				let bottom_world: f32 = postion.y + half_height;
				let ent_left: f32 = postion.x - half_width + inset_x;
				let ent_right: f32 = postion.x + half_width - inset_x;

				for (plat_left, plat_right, plat_top, plat_vx) in &platform_tops {
					if ent_right < *plat_left || ent_left > *plat_right {
						continue;
					}

					if prev_bottom_world <= *plat_top && bottom_world >= *plat_top {
						postion.y = *plat_top - half_height;
						velocity.y = 0.0;

						// carry along horizontally
						postion.x += *plat_vx;

						break;
					}
				}
			}
		} // <- pos/vel borrows end here

		if hit_wall {
			let kind: u8 = *game_state.entity_kinds.get(id).unwrap_or(&0);

			// kind==2 is slime in your data
			if kind == 2 {
				if let Some(vel) = game_state.velocities.get_mut(id) {
					vel.x = -old_vx;
				}
			}
		}

		// now it's legal to query game_state immutably
		if is_player && game_state.on_ground(id) && game_state.on_ground_safe(id) {
			let Some(pos) = game_state.positions.get(id) else {
				continue;
			};
			game_state.last_grounded_pos = Some(*pos);
		}

		let (half_width, half_height) = game_state.get_entity_half_values(id);
		// clamp to world bounds (x only for now)
		{
			let Some(pos) = game_state.positions.get_mut(id) else {
				continue;
			};
			let Some(vel) = game_state.velocities.get_mut(id) else {
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
		let Some(pos) = game_state.positions.get(id) else {
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
	let grounded: bool = game_state.on_ground(entity_id) || game_state.on_moving_platform(entity_id);
	let on_left: bool = game_state.on_wall_left(entity_id);
	let on_right: bool = game_state.on_wall_right(entity_id);

	if !grounded && !on_left && !on_right {
		return false;
	}

	if let Some(vel) = game_state.velocities.get_mut(entity_id) {
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

pub fn patrol(game_state: &mut GameState) {
	let ids = game_state.patrolling.keys();

	for id in ids {
		let pos = match game_state.positions.get(id) {
			Some(p) => *p,
			None => continue,
		};

		let vel = match game_state.velocities.get_mut(id) {
			Some(v) => v,
			None => continue,
		};

		let kind_u8: u8 = *game_state.entity_kinds.get(id).unwrap_or(&0);
		let kind = EntityKind::from_u8(kind_u8);
		let speed_u8: u8 = game_state.speeds.get(id).copied().unwrap_or(0);

		let speed: f32 = match kind {
			EntityKind::MovingPlatform => speed_u8 as f32, // keep as-is (platform feels right)
			EntityKind::Imp => (speed_u8 as f32) * 0.25,   // slow imps down
			_ => (speed_u8 as f32) * 0.25,                 // default slow speed
		};

		let mut min_x: f32 = game_state.range_mins.get(id).copied().unwrap_or(pos.x);
		let mut max_x: f32 = game_state.range_maxes.get(id).copied().unwrap_or(pos.x);

		// normalize range ordering
		if min_x > max_x {
			let t: f32 = min_x;
			min_x = max_x;
			max_x = t;
		}

		// degenerate range => stand still
		if (max_x - min_x) < 1.0 {
			vel.x = 0.0;
			continue;
		}

		// clamp position to range
		if let Some(p) = game_state.positions.get_mut(id) {
			if p.x < min_x {
				p.x = min_x;
			}
			if p.x > max_x {
				p.x = max_x;
			}
		}

		// keep last direction (from vel) unless we hit an end
		let mut dir: f32 = if vel.x < 0.0 { -1.0 } else { 1.0 };

		if pos.x <= min_x {
			dir = 1.0;
		} else if pos.x >= max_x {
			dir = -1.0;
		}

		vel.x = dir * speed;
	}

	return;
}
