use crate::game::game_state::{EntityId, GameState};

pub fn update(game_state: &mut GameState) {
	let ids: Vec<EntityId> = game_state.enemy_ids.clone();
	// println!("ai update enemy={}", ids.len());

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
		let speed: f32 = (speed_u8 as f32) * game_state.level.tile_width as f32;

		let min_x: f32 = game_state.range_mins.get(id).copied().unwrap_or(pos.x);
		let max_x: f32 = game_state.range_maxes.get(id).copied().unwrap_or(pos.x);

		// println!("speed={}, min_x={}, max_x={}", speed, min_x, max_x);

		let dir: f32;

		if pos.x <= min_x {
			dir = 1.0;
		} else if pos.x >= max_x {
			dir = -1.0;
		} else {
			// keep existing direction if you already store facing; otherwise keep moving right
			dir = 1.0;
		}

		vel.x = dir * speed;
		// println!("vel.x={}", vel.x)
	}

	return;
}
