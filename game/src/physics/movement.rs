extern crate alloc;
use alloc::collections::BTreeMap;

#[cfg(feature = "gba")]
use alloc::vec::Vec;

use crate::{
	debugln,
	engine_math::{Vec2, aabb_overlaps_solid_tiles},
	physics::collision::{HitSide, classify_aabb_hit_side, resolve_ceiling_collision, resolve_floor_collision, resolve_wall_collision},
	platform::audio::SfxId,
	runtime::{
		level::Level,
		session::Session,
		state::{DeathAnim, EntityId, EntityKind, State},
	},
};

#[allow(dead_code)]
pub enum CollisionOutcome {
	None,
	Stomped(EntityId),
	Damaged { source: EntityId },
	HitWall,
	HitWallEnemy,
	Crushed { source: EntityId },
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

pub struct Collider {
	pub id: EntityId,
	pub kind: EntityKind,
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
	// pub velocity_x: f32,
	// pub shape: ColliderShape,
	pub profile: CollisionProfile,
	pub delta_x: f32,
}

#[inline(always)]
fn dead_profile() -> CollisionProfile {
	return CollisionProfile {
		top: face(false, 0, false),
		right: face(false, 0, false),
		bottom: face(false, 0, false),
		left: face(false, 0, false),
		stompable: false,
	};
}

pub fn move_and_collide(state: &mut State, session: &Session) {
	let tile_width: f32 = state.level.tile_width as f32;
	let tile_height: f32 = state.level.tile_height as f32;
	let level_width_pixels: f32 = (state.level.width as f32) * tile_width;
	let level_height_pixels: f32 = (state.level.height as f32) * tile_height;
	let margin: f32 = 64.0;
	let player_id: EntityId = state.get_player_id();
	let mut colliders: Vec<Collider> = Vec::new();

	// let mut delta_x_by_ids: HashMap<EntityId, f32> = HashMap::new();
	let mut delta_x_by_ids: BTreeMap<EntityId, f32> = BTreeMap::new();

	let entity_ids: Vec<EntityId> = state.positions.keys().collect();

	for entity_id in &entity_ids {
		let kind_u8: u8 = *state.entity_kinds.get(*entity_id).unwrap_or(&0);
		let kind: EntityKind = EntityKind::from_u8(kind_u8);

		if kind != EntityKind::MovingPlatform {
			continue;
		}

		let (half_width, half_height) = state.get_entity_half_values(*entity_id);

		let position: &mut Vec2 = state.positions.get_mut(*entity_id).unwrap();
		let velocity: &mut Vec2 = state.velocities.get_mut(*entity_id).unwrap();

		velocity.y = 0.0;

		let old_x: f32 = position.x;
		let old_vx: f32 = velocity.x;

		position.x += velocity.x;
		resolve_wall_collision(&state.level, position, velocity, half_width, half_height, false);

		let delta_x: f32 = position.x - old_x;
		delta_x_by_ids.insert(*entity_id, delta_x);

		let hit_wall: bool = old_vx != 0.0 && velocity.x == 0.0;
		if hit_wall {
			velocity.x = -old_vx;
		}
	}

	for (id, pos) in state.positions.iter() {
		let kind_u8: u8 = *state.entity_kinds.get(id).unwrap_or(&0);
		let kind: EntityKind = EntityKind::from_u8(kind_u8);
		let (half_width, half_height) = state.get_entity_half_values(id);
		let delta_x: f32 = *delta_x_by_ids.get(&id).unwrap_or(&0.0);
		let dying: bool = state.is_dying(id);

		colliders.push(Collider {
			id,
			kind,
			left: pos.x - half_width,
			right: pos.x + half_width,
			top: pos.y - half_height,
			bottom: pos.y + half_height,
			profile: if dying { dead_profile() } else { profile_for_kind(kind) },
			delta_x,
		});
	}

	for entity_id in entity_ids {
		let is_player: bool = entity_id == player_id;

		let kind: EntityKind = EntityKind::from_u8(*state.entity_kinds.get(entity_id).unwrap_or(&0));
		if kind == EntityKind::MovingPlatform {
			continue;
		}

		// do movement + collision inside a scope so &mut borrows drop
		{
			let (half_width, half_height) = state.get_entity_half_values(entity_id);

			let Some(position) = state.positions.get_mut(entity_id) else {
				continue;
			};

			let Some(velocity) = state.velocities.get_mut(entity_id) else {
				continue;
			};

			let prev_pos: Vec2 = position.clone();
			let prev_bottom_level: f32 = position.y + half_height;

			position.x += velocity.x;
			position.y += velocity.y;

			resolve_wall_collision(&state.level, position, velocity, half_width, half_height, false);

			resolve_ceiling_collision(&state.level, position, velocity, half_width, half_height);
			resolve_floor_collision(&state.level, position, velocity, half_width, half_height, prev_bottom_level);

			let pos_before_entities: Vec2 = position.clone();
			let is_patrolling: bool = state.patrolling.get(entity_id).copied().unwrap_or(false);
			let outcome: CollisionOutcome = resolve_entity_collisions(
				&state.level,
				&session.settings,
				entity_id,
				kind,
				is_patrolling,
				prev_pos,
				position,
				velocity,
				half_width,
				half_height,
				&colliders,
			);

			let external_dx: f32 = position.x - pos_before_entities.x;
			if external_dx != 0.0 {
				let old_vx: f32 = velocity.x;
				velocity.x = external_dx;
				resolve_wall_collision(&state.level, position, velocity, half_width, half_height, false);
				velocity.x = old_vx;
			}

			match outcome {
				CollisionOutcome::None => {}
				CollisionOutcome::Crushed { source: _ } => {
					if is_player {
						debugln!("Crushed");
						state.kill_player(session, player_id);
					}
				}
				CollisionOutcome::Stomped(target_id) => {
					if kind == EntityKind::Player {
						let chain: u16 = state.stomp_chains.get(entity_id).copied().unwrap_or(0);
						state.stomp_chains.set(entity_id, chain.saturating_add(1));
					}

					let chain: u16 = state.stomp_chains.get(player_id).copied().unwrap_or(0);
					let bonus: u16 = stomp_bonus(chain, session.settings.stomp_chain_gain_per_stomp as u16).min(session.settings.stomp_bonus_cap as u16);
					let base_stomp_damage = state.base_stomp_damages.get(player_id).copied().unwrap_or(2);
					let damage: u16 = base_stomp_damage + bonus;
					let hit_points = state.hit_points.get(target_id).copied().unwrap_or(1);

					if damage >= hit_points {
						state.start_enemy_death(target_id, DeathAnim::SlimeFlatten);

						if session.settings.are_sound_effects_enabled {
							state.audio.play_sfx(SfxId::Stomp);
						}

						if session.settings.are_sound_effects_enabled {
							state.audio.play_sfx(SfxId::Stomp);
						}
					} else {
						state.hit_points.set(target_id, hit_points - damage);
					}
				}
				CollisionOutcome::Damaged { source: _ } => {
					if is_player {
						//TODO: Calc Damage kill player if needed.
						debugln!("Damaged");
						state.kill_player(session, player_id);
					}
				}
				CollisionOutcome::HitWall => {
					// request a patrol flip for this entity
					let cool_down: u8 = state.bump_cooldowns.get(entity_id).copied().unwrap_or(0);
					if cool_down == 0 {
						state.patrol_flips.set(entity_id, true);
						state.bump_cooldowns.set(entity_id, 6);
					}
				}
				CollisionOutcome::HitWallEnemy => {
					// enemy bump: debounce
					if kind != EntityKind::Player && kind != EntityKind::MovingPlatform {
						let cool_down: u8 = state.bump_cooldowns.get(entity_id).copied().unwrap_or(0);
						if cool_down == 0 {
							state.patrol_flips.set(entity_id, true);
							state.bump_cooldowns.set(entity_id, 6);
						}
					}
				}
			} // <- pos/vel borrows end here

			if is_player {
				let on_wall_left = state.on_wall_left(entity_id);
				let on_wall_right = state.on_wall_right(entity_id);
				let grounded_now: bool = state.is_grounded_now(entity_id);

				if grounded_now {
					if state.camera_baseline_max_bottom_world.is_none() {
						let (_half_width, half_height) = state.get_entity_half_values(entity_id);
						if let Some(pos) = state.positions.get(entity_id) {
							let tile_height_world: f32 = state.level.tile_height as f32;
							let pad_world: f32 = session.settings.camera_bottom_padding_tiles as f32 * tile_height_world;

							let ground_world_y: f32 = pos.y + half_height;
							state.camera_baseline_max_bottom_world = Some(ground_world_y + pad_world);
						}
					}
				}

				if let Some(jump_state) = state.jump_states.get_mut(entity_id) {
					// coyote update (your existing code)
					if grounded_now {
						jump_state.coyote_frames_left = session.settings.coyote_frames_max;
					} else if jump_state.coyote_frames_left > 0 {
						jump_state.coyote_frames_left -= 1;
					}

					jump_state.was_grounded = grounded_now;

					// jump buffer update + consume
					if jump_state.jump_buffer_frames_left > 0 {
						jump_state.jump_buffer_frames_left -= 1;

						let can_jump_now: bool = grounded_now || jump_state.coyote_frames_left > 0 || on_wall_left || on_wall_right;

						if can_jump_now {
							let should_fire: bool = true;

							if should_fire {
								let jumped = try_jump(state, session, entity_id);
								if jumped {
									if session.settings.are_sound_effects_enabled {
										state.audio.play_sfx(SfxId::Jump);
									}
								}
							}
						}
					}
				}

				// tick respawn cooldown every frame
				if let Some(respawn_state) = state.respawn_states.get_mut(entity_id) {
					if respawn_state.respawn_cooldown_frames > 0 {
						respawn_state.respawn_cooldown_frames -= 1;
					}

					// update last grounded pos only when grounded
					if grounded_now {
						if let Some(position) = state.positions.get(entity_id).copied() {
							respawn_state.last_grounded_pos = position;
							respawn_state.has_last_grounded_pos = true;
						}
					}
				}
			}

			let (half_width, half_height) = &state.get_entity_half_values(entity_id);

			let Some(position) = state.positions.get_mut(entity_id) else {
				continue;
			};

			// clamp to world bounds (x only for now)
			{
				let Some(velocity) = state.velocities.get_mut(entity_id) else {
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
					state.kill_player(session, player_id);
					continue;
				} else {
					state.remove_entity(entity_id);
					continue;
				}
			}
		}
	}
}

pub fn try_jump(state: &mut State, session: &Session, entity_id: EntityId) -> bool {
	let grounded: bool = state.is_grounded_now(entity_id);
	let on_left: bool = state.on_wall_left(entity_id);
	let on_right: bool = state.on_wall_right(entity_id);

	let coyote_frames_left: u8 = state.jump_states.get(entity_id).map(|js| js.coyote_frames_left).unwrap_or(0);

	let coyote_ok: bool = coyote_frames_left > 0;

	if !grounded && !coyote_ok && !on_left && !on_right {
		return false;
	}

	let jump_multiplier_u8: u8 = state.jump_multipliers.get(entity_id).copied().unwrap_or(1);
	let jump_multiplier: f32 = jump_multiplier_u8 as f32;

	let jump_velocity: f32 = session.settings.jump_velocity * jump_multiplier;

	if let Some(velocity) = state.velocities.get_mut(entity_id) {
		velocity.y = jump_velocity;

		// wall jump push (only if not grounded/coyote jump)
		if !grounded && !coyote_ok {
			let wall_push: f32 = 2.5;
			if on_left {
				velocity.x = wall_push;
			} else if on_right {
				velocity.x = -wall_push;
			}
		}

		// consume grace/buffer state when a jump actually fires
		if let Some(js) = state.jump_states.get_mut(entity_id) {
			js.coyote_frames_left = 0;
			js.jump_buffer_frames_left = 0;
		}
		return true;
	}

	return false;
}

pub fn patrol(state: &mut State) {
	let ids = state.patrolling.keys();

	for id in ids {
		let position = match state.positions.get(id) {
			Some(p) => *p,
			None => continue,
		};

		let velocity = match state.velocities.get_mut(id) {
			Some(v) => v,
			None => continue,
		};

		if let Some(cd) = state.bump_cooldowns.get(id).copied() {
			if cd > 1 {
				state.bump_cooldowns.set(id, cd - 1);
			} else {
				// cd == 1 → expire now
				state.bump_cooldowns.remove(id);
			}
		}

		// normalize range ordering
		let mut min_x: f32 = state.range_mins.get(id).copied().unwrap_or(position.x);
		let mut max_x: f32 = state.range_maxes.get(id).copied().unwrap_or(position.x);
		let speed = state.speeds.get(id).copied().unwrap_or(0) as f32;

		let flip_now: bool = state.patrol_flips.take(id).unwrap_or(false);

		if flip_now {
			velocity.x = -velocity.x;
			if velocity.x == 0.0 {
				let mid_x: f32 = (min_x + max_x) * 0.5;
				velocity.x = if position.x >= mid_x { -speed } else { speed };
			}
			continue;
		}

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

		// re-read x after clamping
		let pos_x: f32 = match state.positions.get(id) {
			Some(p) => p.x,
			None => continue,
		};

		// pick direction from current velocity
		let mut dir: f32 = if velocity.x < 0.0 { -1.0 } else { 1.0 };

		// if collision stopped us, choose direction based on range
		if velocity.x == 0.0 {
			let mid_x: f32 = (min_x + max_x) * 0.5;
			dir = if pos_x >= mid_x { -1.0 } else { 1.0 };
		}

		// flip cleanly at patrol bounds
		if pos_x <= min_x {
			dir = 1.0;
		} else if pos_x >= max_x {
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
			let mut profile: CollisionProfile = solid_profile(0, false);
			profile.bottom.blocks = false;
			return profile;
		}
		_ => {
			// enemies: solid + damage on sides/bottom, stompable
			return solid_profile(1, true);
		}
	}
}

#[inline(always)]
fn resolve_entity_collisions(
	level: &Level,
	settings: &crate::runtime::Settings,
	entity_id: EntityId,
	kind: EntityKind,
	is_patrolling: bool,
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

		for collider in colliders {
			if moved_up && prev_top >= collider.bottom - 0.01 {
				continue;
			}

			if collider.id == entity_id {
				continue;
			}
			if is_ghost(&collider.profile) {
				continue;
			}

			if right <= collider.left || left >= collider.right || bottom <= collider.top || top >= collider.bottom {
				continue;
			}

			let mut side: HitSide = classify_aabb_hit_side(
				prev_left,
				prev_right,
				prev_top,
				prev_bottom,
				collider.left,
				collider.right,
				collider.top,
				collider.bottom,
			);
			if moved_up && prev_top >= collider.bottom - 0.01 && top < collider.bottom {
				side = HitSide::Bottom;
			} else if moved_down && prev_bottom <= collider.top + 0.01 && bottom > collider.top {
				side = HitSide::Top;
			}

			// -------- block resolution --------
			match side {
				HitSide::Top => {
					if collider.profile.top.blocks && moved_down && prev_bottom <= collider.top + 0.01 {
						// stomp: player landing on a stompable target while falling
						if kind == EntityKind::Player && collider.profile.stompable && velocity.y > 0.0 {
							position.y = collider.top - half_height;
							velocity.y = settings.jump_velocity * settings.stomp_bounce_multiplier; // bounce up (JUMP_VELOCITY is negative)
							return CollisionOutcome::Stomped(collider.id);
						}

						// normal landing/blocking
						position.y = collider.top - half_height;
						velocity.y = 0.0;

						if collider.delta_x != 0.0 {
							position.x += collider.delta_x;
						}

						continue 'pass;
					}
				}
				HitSide::Bottom => {
					if collider.profile.bottom.blocks && moved_up && prev_top >= collider.bottom - 0.01 {
						position.y = collider.bottom + half_height;
						velocity.y = 0.0;
						continue 'pass;
					}
				}

				HitSide::Left => {
					if collider.profile.left.blocks && prev_right <= collider.left + 0.01 {
						position.x = collider.left - half_width - settings.bounce_separator;

						// only moving platforms should "carry/push" via delta_x
						if collider.kind == EntityKind::MovingPlatform && collider.delta_x < 0.0 {
							let proposed_x: f32 = position.x + collider.delta_x;
							let proposed_left: f32 = proposed_x - half_width;
							let proposed_right: f32 = proposed_x + half_width;
							let proposed_top: f32 = position.y - half_height;
							let proposed_bottom: f32 = position.y + half_height;

							if kind == EntityKind::Player {
								if aabb_overlaps_solid_tiles(level, proposed_left, proposed_right, proposed_top, proposed_bottom) {
									return CollisionOutcome::Crushed { source: collider.id };
								}
							}

							position.x = proposed_x;
							velocity.x = 0.0;
							continue 'pass;
						}
						let actor_is_enemy: bool = kind != EntityKind::Player && kind != EntityKind::MovingPlatform;
						let other_is_enemy: bool = collider.kind != EntityKind::Player && collider.kind != EntityKind::MovingPlatform;

						if is_patrolling && actor_is_enemy && other_is_enemy {
							return CollisionOutcome::HitWallEnemy;
						}

						velocity.x = 0.0;
						return CollisionOutcome::HitWall;
					}
				}

				HitSide::Right => {
					if collider.profile.right.blocks && prev_left >= collider.right - 0.01 {
						position.x = collider.right + half_width + settings.bounce_separator;

						// only moving platforms should "carry/push" via delta_x
						if collider.kind == EntityKind::MovingPlatform && collider.delta_x > 0.0 {
							let proposed_x: f32 = position.x + collider.delta_x;
							let proposed_left: f32 = proposed_x - half_width;
							let proposed_right: f32 = proposed_x + half_width;
							let proposed_top: f32 = position.y - half_height;
							let proposed_bottom: f32 = position.y + half_height;

							if kind == EntityKind::Player {
								if aabb_overlaps_solid_tiles(level, proposed_left, proposed_right, proposed_top, proposed_bottom) {
									return CollisionOutcome::Crushed { source: collider.id };
								}
							}

							position.x = proposed_x;
						}

						velocity.x = 0.0;
						return CollisionOutcome::HitWall;
					}
				}
			}

			// let on_top: bool = prev_bottom <= collider.top + 0.02 && bottom <= collider.top + 0.05;

			let was_above: bool = prev_bottom <= collider.top + 0.01;
			let was_below: bool = prev_top >= collider.bottom - 0.01;
			let side_overlap: bool = !was_above && !was_below;

			if side_overlap && collider.delta_x != 0.0 && (collider.profile.left.blocks || collider.profile.right.blocks) {
				// shove actor out sideways...
				if collider.delta_x > 0.0 {
					position.x = collider.right + half_width;
					velocity.x = 0.0;
					continue 'pass;
				}

				if collider.delta_x < 0.0 {
					position.x = collider.left - half_width;
					velocity.x = 0.0;
					continue 'pass;
				}
			}

			let overlap_left: f32 = collider.right - left;
			let overlap_right: f32 = right - collider.left;
			let overlap_top: f32 = collider.bottom - top;
			let overlap_bottom: f32 = bottom - collider.top;

			let push_left: f32 = if overlap_left < overlap_right { overlap_left } else { -overlap_right };
			let push_top: f32 = if overlap_top < overlap_bottom { overlap_top } else { -overlap_bottom };

			if push_left.abs() < push_top.abs() {
				position.x += push_left;

				if push_left > 0.0 {
					position.x += settings.bounce_separator;
				} else {
					position.x -= settings.bounce_separator;
				}

				// if we had to resolve along X, and the actor is patrolling, this should count as a wall hit
				if is_patrolling {
					// do not zero x for enemy-enemy patrol bumps (prevents twitch)
					let actor_is_enemy: bool = kind != EntityKind::Player && kind != EntityKind::MovingPlatform;
					let other_is_enemy: bool = collider.kind != EntityKind::Player && collider.kind != EntityKind::MovingPlatform;

					if actor_is_enemy && other_is_enemy {
						return CollisionOutcome::HitWallEnemy;
					}

					velocity.x = 0.0;
					return CollisionOutcome::HitWall;
				}

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

		// one-way moving platform behavior for player
		if kind == EntityKind::Player && c.kind == EntityKind::MovingPlatform {
			// if we were below the top last frame, we are not allowed to collide from below/sides while rising
			let was_below_top: bool = prev_bottom > c.top + 0.01;

			// rising (jumping up into it)
			if moved_up && was_below_top {
				continue;
			}

			// also ignore side shoves while we're coming up from below
			// (prevents "snap to edge" when platform is moving)
			if was_below_top && (moved_up || velocity.y < 0.0) {
				continue;
			}
		}

		let mut side: HitSide = classify_aabb_hit_side(prev_left, prev_right, prev_top, prev_bottom, c.left, c.right, c.top, c.bottom);

		if moved_up && prev_top >= c.bottom - 0.01 && top < c.bottom {
			side = HitSide::Bottom;
		} else if moved_down && prev_bottom <= c.top + 0.01 && bottom > c.top {
			side = HitSide::Top;
		}

		let actor_is_enemy: bool = kind != EntityKind::Player && kind != EntityKind::MovingPlatform;
		let other_is_enemy: bool = c.kind != EntityKind::Player && c.kind != EntityKind::MovingPlatform;

		// enemies should not damage enemies when patrolling
		if is_patrolling && actor_is_enemy && other_is_enemy {
			debugln!("enemies should not damage enemies");
			continue;
		}

		// otherwise apply face damage (players will have 0 in profile later)
		let damage: u8 = match side {
			HitSide::Top => c.profile.top.damage,
			HitSide::Right => c.profile.right.damage,
			HitSide::Bottom => c.profile.bottom.damage,
			HitSide::Left => c.profile.left.damage,
		};

		if damage > 0 {
			return CollisionOutcome::Damaged { source: c.id };
		}
	}

	return CollisionOutcome::None;
}

#[inline(always)]
pub fn stomp_bonus(chain: u16, stomp_chain_gain_per_stomp: u16) -> u16 {
	let scaled: u32 = (chain as u32).saturating_mul(stomp_chain_gain_per_stomp as u32);
	return isqrt_u32(scaled) as u16;
	//isqrt_u32(chain as u32) as u16
}

#[inline(always)]
fn isqrt_u32(n: u32) -> u32 {
	let mut x: u32 = n;
	let mut y: u32 = (x + 1) >> 1;
	while y < x {
		x = y;
		y = (x + n / x) >> 1;
	}
	return x;
}

#[inline(always)]
fn is_ghost(profile: &CollisionProfile) -> bool {
	return !profile.top.blocks
		&& !profile.right.blocks
		&& !profile.bottom.blocks
		&& !profile.left.blocks
		&& profile.top.damage == 0
		&& profile.right.damage == 0
		&& profile.bottom.damage == 0
		&& profile.left.damage == 0
		&& !profile.stompable;
}
