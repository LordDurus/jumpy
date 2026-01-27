use crate::{binary_writer::serialize_level, message_registry::MessageRegistry, runtime::*, source::*, text_parse::TriggerActivationMode};

use std::{
	collections::HashMap,
	path::{Path, PathBuf},
};

fn clamp_u8(value: i32) -> u8 {
	if value < 0 {
		return 0;
	}
	if value > 255 {
		return 255;
	}
	return value as u8;
}

pub fn compile_and_serialize(source: &LevelSource) -> Result<Vec<u8>, String> {
	let compiled = compile_level(source)?;
	let bytes = serialize_level(&compiled)?;
	return Ok(bytes);
}

fn resolve_world_id(text: &str) -> Result<u16, String> {
	let id: u16 = text.trim().parse::<u16>().map_err(|_| format!("invalid target world id '{}'", text))?;
	return Ok(id);
}

fn resolve_world_level_id(text: &str) -> Result<u16, String> {
	let id: u16 = text.trim().parse::<u16>().map_err(|_| format!("invalid target level id '{}'", text))?;
	return Ok(id);
}

pub fn compile_level(source: &LevelSource) -> Result<CompiledLevel, String> {
	if source.header.width == 0 || source.header.height == 0 {
		return Err("level width and height must be > 0".to_string());
	}

	let width = source.header.width as usize;
	let height = source.header.height as usize;

	if source.layers.is_empty() {
		return Err("level must have at least one layer".to_string());
	}

	for (i, layer) in source.layers.iter().enumerate() {
		if layer.rows.len() != height {
			return Err(format!("layer {} has {} rows, expected {}", i, layer.rows.len(), height));
		}

		for (y, row) in layer.rows.iter().enumerate() {
			let count = row.chars().count();
			if count != width {
				return Err(format!("layer {} row {} has {} columns, expected {}", i, y, count, width));
			}
		}
	}

	let message_ids_path: PathBuf = get_asset_root().join("messages").join("messages.ids.txt");
	let message_registry: MessageRegistry = MessageRegistry::load_from_file(message_ids_path.to_str().ok_or("invalid message ids path")?)?;
	let pickup_ids_path: PathBuf = get_asset_root().join("pickup-maps").join("pickups.ids.txt");
	let pickup_ids: HashMap<String, u16> = load_ids_map(&pickup_ids_path)?;

	let tile_palette = build_tile_palette();
	let layer_count = source.layers.len() as u8;
	let tiles_per_layer = (width * height) as u32;
	let tile_count_total = tiles_per_layer * layer_count as u32;

	let mut layers_runtime = Vec::with_capacity(source.layers.len());
	for layer in &source.layers {
		let runtime = LayerRuntime {
			collision: if layer.collision { 1 } else { 0 },
			gravity_multiplier: 0,
			reserved1: 0,
			reserved2: 0,
		};
		layers_runtime.push(runtime);
	}

	let mut tiles = Vec::with_capacity(tile_count_total as usize);
	for layer in &source.layers {
		for row in &layer.rows {
			for ch in row.chars() {
				let tile_id = match tile_palette.get(&ch) {
					Some(id) => *id,
					None => {
						return Err(format!("unknown tile character '{}'", ch));
					}
				};
				tiles.push(tile_id);
			}
		}
	}

	let mut entities_runtime = Vec::with_capacity(source.entities.len());
	for entity in &source.entities {
		let top = entity.top as u16;
		let left = entity.left as u16;
		let gravity = gravity_multiplier_to_q4_4(entity.gravity_multiplier)?;
		let jump = entity.jump_multiplier.round().clamp(0.0, 15.0) as u8;
		let attack_power = u8::try_from(entity.attack_power).map_err(|_| format!("attack_power out of range: {}", entity.attack_power))?;

		let hit_points = u16::try_from(entity.hit_points).map_err(|_| format!("hit_points out of range: {}", entity.hit_points))?;

		let runtime = match &entity.kind {
			EntityKindSource::PlayerStart => EntityRuntime {
				kind: EntityKind::Player as u8,
				gravity_multiplier: gravity,
				hit_points: hit_points,
				jump_multiplier: jump,
				attack_power: attack_power,
				top,
				left,
				health_regen_rate: 0,
				invulnerability_time: 0,
				render_style: 0,
				width: 16,
				height: 16,
				speed: 10,
				luck: 5,
				strength: 5,
				range_min: 0,
				range_max: 0,
			},
			EntityKindSource::MovingPlatform {
				platform_kind,
				size: _,
				speed,
				range_min,
				range_max,
			} => {
				let rm: u16 = u16::try_from(*range_min).map_err(|_| format!("range_min out of range"))?;
				let rx: u16 = u16::try_from(*range_max).map_err(|_| format!("range_max out of range"))?;

				EntityRuntime {
					kind: EntityKind::MovingPlatform as u8,
					render_style: entity.render_style,
					gravity_multiplier: 0,
					hit_points: 0,
					jump_multiplier: 0,
					attack_power: 0,
					top,
					left,
					health_regen_rate: 0,
					invulnerability_time: 0,
					width: clamp_u8((entity.width * 16.0).round() as i32).max(1),
					height: clamp_u8((entity.height * 16.0).round() as i32).max(1),
					speed: clamp_u8(*speed),
					strength: resolve_platform_type(platform_kind)?,
					luck: 0,
					range_min: rm,
					range_max: rx,
				}
			}
			EntityKindSource::Enemy {
				enemy_kind,
				range_min,
				range_max,
			} => {
				let resolved_kind: u8 = match enemy_kind.as_str() {
					"slime_blue" => EntityKind::SlimeBlue as u8,
					"slime_undead" => EntityKind::SlimeUndead as u8,
					"slime_lava" => EntityKind::SlimeLava as u8,
					"imp" => EntityKind::Imp as u8,
					_ => return Err(format!("unknown enemy kind '{}'", enemy_kind)),
				};

				let rm: u16 = u16::try_from(*range_min).map_err(|_| format!("range_min out of range"))?;
				let rx: u16 = u16::try_from(*range_max).map_err(|_| format!("range_max out of range"))?;

				EntityRuntime {
					kind: resolved_kind,
					render_style: entity.render_style,
					gravity_multiplier: gravity,
					hit_points,
					jump_multiplier: jump,
					attack_power,
					top,
					left,
					health_regen_rate: entity.health_regen_rate,
					invulnerability_time: entity.invulnerability_time,
					width: clamp_u8((entity.width * 16.0).round() as i32).max(1),
					height: clamp_u8((entity.height * 16.0).round() as i32).max(1),
					speed: entity.speed,
					strength: entity.strength,
					luck: entity.luck,
					/*
					range_min: entity.range_min as u16,
					range_max: entity.range_max as u16,
					*/
					range_min: rm,
					range_max: rx,
				}
			}
		};

		entities_runtime.push(runtime);
	}

	let mut triggers_runtime = Vec::with_capacity(source.triggers.len());
	for trigger in &source.triggers {
		let top = trigger.top as u16;
		let left = trigger.left as u16;
		let width = trigger.width as u16;
		let height = trigger.height as u16;

		let runtime = match &trigger.kind {
			TriggerKindSource::LevelExit { target, level, activation_mode } => {
				let world_id: u16 = resolve_world_id(target)?;
				let level_id: u16 = resolve_world_level_id(level)?;
				TriggerRuntime {
					kind: TriggerKind::LevelExit as u8,
					gravity_multiplier: 0,
					left,
					top,
					width,
					height,
					p0: world_id,
					p1: level_id,
					activation_mode: *activation_mode,
				}
			}
			TriggerKindSource::Pickup { pickup, amount, activation_mode } => {
				let pickup_type_id: u16 = resolve_pickup_type_id(pickup)?;
				let amount_u16: u16 = u16::try_from(*amount).map_err(|_| format!("pickup amount out of range: {}", amount))?;

				let value: u16 = if pickup_type_id == 1 || pickup_type_id == 4 {
					// coin/random -> p1 is amount
					amount_u16
				} else {
					// key/book -> p1 is the mapped id for the full string ("book:tom_sawyer", "key:w01:l01")
					*pickup_ids.get(pickup.trim()).ok_or_else(|| format!("unknown pickup id '{}'", pickup))?
				};

				TriggerRuntime {
					kind: TriggerKind::Pickup as u8,
					gravity_multiplier: 0,
					left,
					top,
					width,
					height,
					p0: pickup_type_id,
					p1: value,
					activation_mode: *activation_mode,
				}
			}
			TriggerKindSource::Message { text_id, activation_mode } => {
				let msg_id: u16 = message_registry.resolve_message_id(text_id)?;
				TriggerRuntime {
					kind: TriggerKind::Message as u8,
					gravity_multiplier: 0,
					left,
					top,
					width,
					height,
					p0: *activation_mode as u16,
					p1: msg_id,
					activation_mode: TriggerActivationMode::Action as u8,
				}
			}
		};

		triggers_runtime.push(runtime);
	}

	let background_id = resolve_background_id(&source.header.background)?;
	let gravity_fixed = gravity_to_fixed(source.header.gravity);

	let header = FileHeader {
		magic: *b"JLVL",
		version: 1,
		header_size: 0,
		width: source.header.width as u16,
		height: source.header.height as u16,
		tile_width: source.header.tile_width as u16,
		tile_height: source.header.tile_height as u16,
		layer_count,
		entity_count: entities_runtime.len() as u16,
		trigger_count: triggers_runtime.len() as u16,
		gravity_fixed,
		background_id,
		gravity: source.header.gravity as u8,
		health_regen_rate: 1,
		invulnerability_time: 1,
		tiles_per_layer,
		tile_count_total,
		offset_layers: 0,
		offset_entities: 0,
		offset_triggers: 0,
		offset_tiles: 0,
	};

	let compiled = CompiledLevel {
		header,
		layers: layers_runtime,
		entities: entities_runtime,
		triggers: triggers_runtime,
		tiles,
	};

	return Ok(compiled);
}

fn build_tile_palette() -> HashMap<char, u8> {
	let mut map = HashMap::new();
	map.insert('.', 0); // Empty
	map.insert('#', 1); // Stone
	map.insert('^', 2); // SpikeUp
	map.insert('~', 3); // Water Surface
	map.insert('=', 4); // GrassTop
	map.insert('v', 5); // SpikeDown
	map.insert('<', 6); // SpikeLeft
	map.insert('>', 7); // SpikeRight
	map.insert('w', 8); // Water Body
	map.insert('[', 9); // moving platform left
	map.insert('-', 10); // moving platform middle
	map.insert(']', 11); // moving platform right
	map.insert('d', 12); // Dirt right
	map.insert('b', 13); // Sign Begin
	map.insert('e', 14); // Sign End
	map.insert('(', 15); // platform left
	map.insert('_', 16); // platform middle
	map.insert(')', 17); // platform right
	map.insert('B', 255); // Black
	map.insert('G', 254); // Torch Glow
	map.insert('D', 253); // Dark Brown Rock
	return map;
}

fn resolve_background_id(name: &str) -> Result<u8, String> {
	if name.to_ascii_lowercase().eq_ignore_ascii_case("bg_library_stone") {
		return Ok(1);
	}

	if name.to_ascii_lowercase().eq_ignore_ascii_case("bg_parallax_forest") {
		return Ok(2);
	}

	return Err(format!("unknown background '{}'", name));
}

fn gravity_to_fixed(g: f32) -> i16 {
	let scaled = g * 256.0;
	let rounded = scaled.round();
	return rounded as i16;
}

fn resolve_platform_type(kind: &str) -> Result<u8, String> {
	match kind {
		"horizontal" => return Ok(0),
		"vertical" => return Ok(1),
		_ => return Err(format!("unknown platform_kind '{}'", kind)),
	}
}

fn gravity_multiplier_to_q4_4(v: f32) -> Result<u8, String> {
	if !v.is_finite() {
		return Err("gravity multiplier must be finite".to_string());
	}

	if v < 0.0 || v > 15.9375 {
		return Err(format!("gravity multiplier {} out of range (0..15.9375)", v));
	}

	let scaled = (v * 16.0).round() as i32;
	return Ok(scaled as u8);
}

fn resolve_pickup_type_id(text: &str) -> Result<u16, String> {
	let trimmed: &str = text.trim();
	let base: &str = trimmed.split(':').next().unwrap_or(trimmed);

	match base {
		"coin" => return Ok(1),
		"key" => return Ok(2),
		"book" => return Ok(3),
		"random" => return Ok(4),
		_ => return Err(format!("unknown pickup type '{}'", text)),
	}
}

pub fn get_asset_root() -> PathBuf {
	let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("assets");
	return root;
}

fn load_ids_map(path: &PathBuf) -> Result<HashMap<String, u16>, String> {
	let text: String = std::fs::read_to_string(path).map_err(|e| format!("failed to read {:?}: {}", path, e))?;
	let mut map: HashMap<String, u16> = HashMap::new();

	for (i, raw) in text.lines().enumerate() {
		let line_no: usize = i + 1;
		let line: &str = raw.trim();

		if line.is_empty() || line.starts_with('#') {
			continue;
		}

		let mut parts = line.splitn(2, '=');
		let key: &str = parts.next().unwrap_or("").trim();
		let val: &str = parts.next().unwrap_or("").trim();

		if key.is_empty() || val.is_empty() {
			return Err(format!("invalid ids line at {:?}:{} -> '{}'", path, line_no, raw));
		}

		let id: u16 = val.parse::<u16>().map_err(|e| format!("invalid id at {:?}:{} -> '{}': {}", path, line_no, val, e))?;
		map.insert(key.to_string(), id);
	}

	return Ok(map);
}
