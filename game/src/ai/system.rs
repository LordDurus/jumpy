use crate::game::game_state::{EntityId, GameState};

pub fn update(game_state: &mut GameState) {
	let ids: Vec<EntityId> = game_state.enemy_ids.clone();

	for id in ids {
		let pos = match game_state.positions.get(&id) {
			Some(p) => *p,
			None => continue,
		};

		let vel = match game_state.velocities.get_mut(&id) {
			Some(v) => v,
			None => continue,
		};

		let speed_u8: u8 = game_state.speeds.get(&id).copied().unwrap_or(0);
		let speed: f32 = (speed_u8 as f32) * 0.25; // 10 -> 2.5 px/frame

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
