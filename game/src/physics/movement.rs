use crate::{
	engine_math::Vec2,
	game::game_state::{EntityId, EntityKind, GameState},
	physics::{
		collision::{HitSide, classify_aabb_hit_side, resolve_ceiling_collision, resolve_floor_collision, resolve_wall_collision},
		constants::JUMP_VELOCITY,
	},
};

#[allow(dead_code)]
pub enum CollisionOutcome {
	None,
	Stomped(EntityId),
	Damaged { source: EntityId },
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ColliderShape {
	Aabb,
	OneWayAabb, // for entities later if you want (not needed yet)
	TriangleUp,
	TriangleDown,
	TriangleLeft,
	TriangleRight,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct FaceProfile {
	pub blocks: bool,
	pub damage: u8,
	pub standable: bool,
}

#[derive(Clone, Copy)]
pub struct CollisionProfile {
	pub top: FaceProfile,
	pub right: FaceProfile,
	pub bottom: FaceProfile,
	pub left: FaceProfile,
	pub stompable: bool,
}

#[derive(Clone, Copy)]
pub struct Collider {
	pub id: EntityId,
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
	// 	pub velocity_x: f32,
	// 	pub shape: ColliderShape,
	pub profile: CollisionProfile,
	pub delta_x: f32,
}

#[inline(always)]
pub fn move_and_collide(game_state: &mut GameState) {
	let tile_width: f32 = game_state.level.tile_width as f32;
	let tile_height: f32 = game_state.level.tile_height as f32;
	let level_width_pixels: f32 = (game_state.level.width as f32) * tile_width;
	let level_height_pixels: f32 = (game_state.level.height as f32) * tile_height;
	let margin: f32 = 64.0;
	let player_id: EntityId = game_state.get_player_id();
	let mut colliders: Vec<Collider> = Vec::new();
	let mut delta_x_by_ids: std::collections::HashMap<EntityId, f32> = std::collections::HashMap::new();
	let entity_ids: Vec<EntityId> = game_state.positions.keys().collect();

	for entity_id in &entity_ids {
		let kind_u8: u8 = *game_state.entity_kinds.get(*entity_id).unwrap_or(&0);
		let kind: EntityKind = EntityKind::from_u8(kind_u8);

		if kind != EntityKind::MovingPlatform {
			continue;
		}

		let (half_width, half_height) = game_state.get_entity_half_values(*entity_id);

		let position: &mut Vec2 = game_state.positions.get_mut(*entity_id).unwrap();
		let velocity: &mut Vec2 = game_state.velocities.get_mut(*entity_id).unwrap();

		velocity.y = 0.0;

		let old_x: f32 = position.x;
		let old_vx: f32 = velocity.x;

		position.x += velocity.x;
		resolve_wall_collision(&game_state.level, position, velocity, half_width, half_height, false);

		let delta_x: f32 = position.x - old_x;
		delta_x_by_ids.insert(*entity_id, delta_x);

		let hit_wall: bool = old_vx != 0.0 && velocity.x == 0.0;
		if hit_wall {
			velocity.x = -old_vx;
		}
	}

	for (id, pos) in game_state.positions.iter() {
		let kind_u8: u8 = *game_state.entity_kinds.get(id).unwrap_or(&0);
		let kind: EntityKind = EntityKind::from_u8(kind_u8);
		let (half_width, half_height) = game_state.get_entity_half_values(id);
		let delta_x: f32 = *delta_x_by_ids.get(&id).unwrap_or(&0.0);

		colliders.push(Collider {
			id,
			left: pos.x - half_width,
			right: pos.x + half_width,
			top: pos.y - half_height,
			bottom: pos.y + half_height,
			// velocity_x: game_state.velocities.get(id).map(|v| v.x).unwrap_or(0.0),
			// shape: ColliderShape::Aabb,
			profile: profile_for_kind(kind),
			delta_x,
		});
	}

	for entity_id in entity_ids {
		let is_player: bool = entity_id == player_id;

		let kind: EntityKind = EntityKind::from_u8(*game_state.entity_kinds.get(entity_id).unwrap_or(&0));
		if kind == EntityKind::MovingPlatform {
			continue;
		}

		// do movement + collision inside a scope so &mut borrows drop
		{
			let (half_width, half_height) = game_state.get_entity_half_values(entity_id);

			let Some(position) = game_state.positions.get_mut(entity_id) else {
				continue;
			};

			let Some(velocity) = game_state.velocities.get_mut(entity_id) else {
				continue;
			};

			let prev_pos: Vec2 = position.clone();
			let prev_bottom_level: f32 = position.y + half_height;

			position.x += velocity.x;
			position.y += velocity.y;

			resolve_wall_collision(&game_state.level, position, velocity, half_width, half_height, false);

			resolve_ceiling_collision(&game_state.level, position, velocity, half_width, half_height);
			resolve_floor_collision(&game_state.level, position, velocity, half_width, half_height, prev_bottom_level);

			let pos_before_entities: Vec2 = position.clone();
			let outcome: CollisionOutcome = resolve_entity_collisions(entity_id, prev_pos, position, velocity, half_width, half_height, &colliders);

			let external_dx: f32 = position.x - pos_before_entities.x;
			if external_dx != 0.0 {
				let old_vx: f32 = velocity.x;
				velocity.x = external_dx;
				resolve_wall_collision(&game_state.level, position, velocity, half_width, half_height, false);
				velocity.x = old_vx;
			}

			match outcome {
				CollisionOutcome::None => {}
				CollisionOutcome::Stomped(target_id) => {
					//TODO: Calc Damage remove id needed
					println!("Stomped");
					game_state.remove_entity(target_id);
				}
				CollisionOutcome::Damaged { source: _ } => {
					//TODO: Calc Damage kill player if needed.
					println!("Damaged");
					game_state.respawn_player();
				}
			} // <- pos/vel borrows end here

			// now it's legal to query game_state immutably
			if is_player && game_state.on_ground(entity_id) && game_state.on_ground_safe(entity_id) {
				let Some(pos) = game_state.positions.get(entity_id) else {
					continue;
				};
				game_state.last_grounded_pos = Some(*pos);
			}

			let (half_width, half_height) = &game_state.get_entity_half_values(entity_id);

			let Some(position) = game_state.positions.get_mut(entity_id) else {
				continue;
			};

			// clamp to world bounds (x only for now)
			{
				let Some(velocity) = game_state.velocities.get_mut(entity_id) else {
					continue;
				};

				let min_width: f32 = *half_width;
				let max_width: f32 = (level_width_pixels - half_width).max(min_width);

				if position.x < min_width {
					position.x = min_width;
					if !is_player {
						velocity.x = velocity.x.abs();
					} else {
						velocity.x = 0.0;
					}
				}

				if position.x > max_width {
					position.x = max_width;
					if !is_player {
						velocity.x = -velocity.x.abs();
					} else {
						velocity.x = 0.0;
					}
				}
			}

			let left: f32 = position.x - half_width;
			let right: f32 = position.x + half_width;
			let top: f32 = position.y - half_height;
			let bottom: f32 = position.y + half_height;

			let out: bool = right < -margin || left > level_width_pixels + margin || bottom < -margin || top > level_height_pixels + margin;

			if out {
				if is_player {
					game_state.respawn_player();
					continue;
				} else {
					game_state.remove_entity(entity_id);
					continue;
				}
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

		let velocity = match game_state.velocities.get_mut(id) {
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
			velocity.x = 0.0;
			continue;
		}

		// clamp position to range
		if let Some(position) = game_state.positions.get_mut(id) {
			if position.x < min_x {
				position.x = min_x;
			}
			if position.x > max_x {
				position.x = max_x;
			}
		}

		// keep last direction (from vel) unless we hit an end
		let mut dir: f32 = if velocity.x < 0.0 { -1.0 } else { 1.0 };

		if pos.x <= min_x {
			dir = 1.0;
		} else if pos.x >= max_x {
			dir = -1.0;
		}

		velocity.x = dir * speed;
	}

	return;
}

#[inline(always)]
fn face(blocks: bool, damage: u8, standable: bool) -> FaceProfile {
	return FaceProfile { blocks, damage, standable };
}

#[inline(always)]
fn solid_profile(damage_sides: u8, stompable: bool) -> CollisionProfile {
	return CollisionProfile {
		top: face(true, 0, true),
		right: face(true, damage_sides, false),
		bottom: face(true, damage_sides, false),
		left: face(true, damage_sides, false),
		stompable,
	};
}

#[inline(always)]
fn profile_for_kind(kind: EntityKind) -> CollisionProfile {
	match kind {
		EntityKind::Player => {
			// player is fully solid, no damage, not stompable
			return solid_profile(0, false);
		}
		EntityKind::MovingPlatform => {
			// start with fully solid, then make it jump-through from below
			let mut p: CollisionProfile = solid_profile(0, false);
			p.bottom.blocks = false;
			return p;
		}
		_ => {
			// enemies: solid + damage on sides/bottom, stompable
			return solid_profile(1, true);
		}
	}
}

#[inline(always)]
fn resolve_entity_collisions(
	entity_id: EntityId,
	prev_pos: Vec2,
	position: &mut Vec2,
	velocity: &mut Vec2,
	half_width: f32,
	half_height: f32,
	colliders: &[Collider],
) -> CollisionOutcome {
	let prev_left: f32 = prev_pos.x - half_width;
	let prev_right: f32 = prev_pos.x + half_width;
	let prev_top: f32 = prev_pos.y - half_height;
	let prev_bottom: f32 = prev_pos.y + half_height;
	let mut moved_down: bool = false;
	let mut moved_up: bool = false;

	'pass: for _ in 0..3 {
		let left: f32 = position.x - half_width;
		let right: f32 = position.x + half_width;
		let top: f32 = position.y - half_height;
		let bottom: f32 = position.y + half_height;

		moved_down = bottom > prev_bottom + 0.001;
		moved_up = top < prev_top - 0.001;

		for c in colliders {
			if c.id == entity_id {
				continue;
			}

			if right <= c.left || left >= c.right || bottom <= c.top || top >= c.bottom {
				continue;
			}

			let mut side: HitSide = classify_aabb_hit_side(prev_left, prev_right, prev_top, prev_bottom, c.left, c.right, c.top, c.bottom);
			if moved_up && prev_top >= c.bottom - 0.01 && top < c.bottom {
				side = HitSide::Bottom;
			} else if moved_down && prev_bottom <= c.top + 0.01 && bottom > c.top {
				side = HitSide::Top;
			}

			// -------- block resolution --------
			match side {
				HitSide::Top => {
					if c.profile.top.blocks && moved_down && prev_bottom <= c.top + 0.01 {
						position.y = c.top - half_height;
						velocity.y = 0.0;

						if c.delta_x != 0.0 {
							position.x += c.delta_x;
						}

						continue 'pass;
					}
				}
				HitSide::Bottom => {
					if c.profile.bottom.blocks && moved_up && prev_top >= c.bottom - 0.01 {
						position.y = c.bottom + half_height;
						velocity.y = 0.0;
						continue 'pass;
					}
				}

				HitSide::Left => {
					if c.profile.left.blocks && prev_right <= c.left + 0.01 {
						position.x = c.left - half_width;

						if c.delta_x < 0.0 {
							position.x += c.delta_x;
						}

						velocity.x = 0.0;
						continue 'pass;
					}
				}
				HitSide::Right => {
					if c.profile.right.blocks && prev_left >= c.right - 0.01 {
						position.x = c.right + half_width;

						if c.delta_x < 0.0 {
							position.x += c.delta_x;
						}

						if c.delta_x != 0.0 {
							velocity.x = c.delta_x;
						} else {
							velocity.x = 0.0;
						}

						continue 'pass;
					}
				}
			}

			let on_top: bool = prev_bottom <= c.top + 0.02 && bottom <= c.top + 0.05;
			if on_top && c.delta_x != 0.0 && (c.profile.left.blocks || c.profile.right.blocks) {
				// if platform moved right this frame, shove actor right out of it
				if c.delta_x > 0.0 {
					// entity must end up to the right of platform
					position.x = c.right + half_width;
					velocity.x = 0.0;
					continue 'pass;
				}

				// moved left
				if c.delta_x < 0.0 {
					position.x = c.left - half_width;
					velocity.x = 0.0;
					continue 'pass;
				}
			}

			let overlap_left: f32 = c.right - left;
			let overlap_right: f32 = right - c.left;
			let overlap_top: f32 = c.bottom - top;
			let overlap_bottom: f32 = bottom - c.top;

			let push_left: f32 = if overlap_left < overlap_right { overlap_left } else { -overlap_right };
			let push_top: f32 = if overlap_top < overlap_bottom { overlap_top } else { -overlap_bottom };

			if push_left.abs() < push_top.abs() {
				position.x += push_left;
				velocity.x = 0.0;
			} else {
				position.y += push_top;
				velocity.y = 0.0;
			}

			continue 'pass;
		}

		break;
	}

	let left: f32 = position.x - half_width;
	let right: f32 = position.x + half_width;
	let top: f32 = position.y - half_height;
	let bottom: f32 = position.y + half_height;

	for c in colliders {
		if c.id == entity_id {
			continue;
		}
		if right <= c.left || left >= c.right || bottom <= c.top || top >= c.bottom {
			continue;
		}

		let mut side: HitSide = classify_aabb_hit_side(prev_left, prev_right, prev_top, prev_bottom, c.left, c.right, c.top, c.bottom);

		if moved_up && prev_top >= c.bottom - 0.01 && top < c.bottom {
			side = HitSide::Bottom;
		} else if moved_down && prev_bottom <= c.top + 0.01 && bottom > c.top {
			side = HitSide::Top;
		}

		/*
		if velocity.y < 0.0 && prev_top >= c.bottom - 0.01 && top < c.bottom {
			side = HitSide::Bottom;
		} else if velocity.y > 0.0 && prev_bottom <= c.top + 0.01 && bottom > c.top {
			side = HitSide::Top;
		}
		*/

		// stomp is safe + affects target if stompable
		if side == HitSide::Top && velocity.y > 0.0 && prev_bottom <= c.top + 0.01 && c.profile.stompable {
			// bounce
			velocity.y = -6.0;
			return CollisionOutcome::Stomped(c.id);
		}

		// otherwise apply face damage (players will have 0 in profile later)
		let dmg: u8 = match side {
			HitSide::Top => c.profile.top.damage,
			HitSide::Right => c.profile.right.damage,
			HitSide::Bottom => c.profile.bottom.damage,
			HitSide::Left => c.profile.left.damage,
		};

		if dmg > 0 {
			return CollisionOutcome::Damaged { source: c.id };
		}
	}

	return CollisionOutcome::None;
}
