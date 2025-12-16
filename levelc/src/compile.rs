use crate::{binary_writer::serialize_level, runtime::*, source::*};

use std::collections::HashMap;

pub fn compile_and_serialize(source: &LevelSource) -> Result<Vec<u8>, String> {
	let compiled = compile_level(source)?;
	let bytes = serialize_level(&compiled)?;
	return Ok(bytes);
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

	let tile_palette = build_tile_palette();

	let layer_count = source.layers.len() as u8;
	let tiles_per_layer = (width * height) as u32;
	let tile_count_total = tiles_per_layer * layer_count as u32;

	let mut layers_runtime = Vec::with_capacity(source.layers.len());
	for layer in &source.layers {
		let runtime = LayerRuntime {
			collision: if layer.collision { 1 } else { 0 },
			reserved0: 0,
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
		let x = entity.x as u16;
		let y = entity.y as u16;

		let runtime = match &entity.kind {
			EntityKindSource::PlayerStart => EntityRuntime {
				kind: EntityKind::PlayerStart as u8,
				reserved0: 0,
				x,
				y,
				a: 0,
				b: 0,
				extra_id: 0,
			},
			EntityKindSource::Enemy {
				enemy_kind,
				patrol_min,
				patrol_max,
			} => {
				let type_id = resolve_enemy_type(enemy_kind)?;
				EntityRuntime {
					kind: EntityKind::Enemy as u8,
					reserved0: 0,
					x,
					y,
					a: *patrol_min as i16,
					b: *patrol_max as i16,
					extra_id: type_id,
				}
			}
			EntityKindSource::Pickup { pickup_kind, value } => {
				let type_id = resolve_pickup_type(pickup_kind)?;
				EntityRuntime {
					kind: EntityKind::Pickup as u8,
					reserved0: 0,
					x,
					y,
					a: *value as i16,
					b: 0,
					extra_id: type_id,
				}
			}
		};

		entities_runtime.push(runtime);
	}

	let mut triggers_runtime = Vec::with_capacity(source.triggers.len());
	for trigger in &source.triggers {
		let x = trigger.x as u16;
		let y = trigger.y as u16;
		let width_tr = trigger.width as u16;
		let height_tr = trigger.height as u16;

		let runtime = match &trigger.kind {
			TriggerKindSource::LevelExit { target } => {
				let level_id = resolve_level_id(target)?;
				TriggerRuntime {
					kind: TriggerKind::LevelExit as u8,
					reserved0: 0,
					x,
					y,
					width: width_tr,
					height: height_tr,
					p0: level_id,
					p1: 0,
				}
			}
			TriggerKindSource::Message { text_id } => {
				let msg_id = resolve_message_id(text_id)?;
				TriggerRuntime {
					kind: TriggerKind::Message as u8,
					reserved0: 0,
					x,
					y,
					width: width_tr,
					height: height_tr,
					p0: msg_id,
					p1: 0,
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
		header_size: std::mem::size_of::<FileHeader>() as u16,
		width: source.header.width as u16,
		height: source.header.height as u16,
		tile_size: source.header.tile_size as u8,
		layer_count,
		entity_count: entities_runtime.len() as u16,
		trigger_count: triggers_runtime.len() as u16,
		gravity_fixed,
		background_id,
		reserved0: 0,
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
	map.insert('.', 0);
	map.insert('#', 1);
	map.insert('^', 2);
	map.insert('~', 3);
	return map;
}

fn resolve_enemy_type(kind: &str) -> Result<u16, String> {
	if kind.eq_ignore_ascii_case("slime") {
		return Ok(0);
	}

	return Err(format!("unknown enemy type '{}'", kind));
}

fn resolve_pickup_type(kind: &str) -> Result<u16, String> {
	if kind.eq_ignore_ascii_case("coin") {
		return Ok(0);
	}

	return Err(format!("unknown pickup type '{}'", kind));
}

fn resolve_background_id(name: &str) -> Result<u8, String> {
	if name.eq_ignore_ascii_case("sky_blue") {
		return Ok(0);
	}

	return Err(format!("unknown background '{}'", name));
}

fn resolve_level_id(target: &str) -> Result<u16, String> {
	if target.eq_ignore_ascii_case("level_02") {
		return Ok(2);
	}

	return Err(format!("unknown level target '{}'", target));
}

fn resolve_message_id(text_id: &str) -> Result<u16, String> {
	if text_id.eq_ignore_ascii_case("tutorial_press_jump") {
		return Ok(1);
	}

	return Err(format!("unknown message id '{}'", text_id));
}

fn gravity_to_fixed(g: f32) -> i16 {
	let scaled = g * 256.0;
	let rounded = scaled.round();
	return rounded as i16;
}
