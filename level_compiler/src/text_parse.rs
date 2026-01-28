use crate::{entity_parse_state::EntityParseState, layer_parse_state::LayerParseState, source::*, trigger_parse_state::TriggerParseState};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriggerActivationMode {
	Auto = 0,
	Action = 1,
	Up = 2,
	Down = 3,
	Left = 4,
	Right = 5,
}

#[derive(Debug, PartialEq)]
enum Section {
	None,
	Header,
	Layers,
	LayerBody,
	Entities,
	EntityBody,
	Triggers,
	TriggerBody,
}

pub fn load_level_from_str(text: &str) -> Result<LevelSource, String> {
	let mut section = Section::None;
	let mut reading_tiles = false;
	let mut entities: Vec<EntitySource> = Vec::new();
	let mut header_opt: Option<LevelHeader> = None;
	let mut layers: Vec<LayerSource> = Vec::new();
	let mut ent = EntityParseState::new();
	let mut trigger = TriggerParseState::new();
	let mut layer = LayerParseState::new();
	let mut line_number = 0;
	let mut triggers: Vec<TriggerSource> = Vec::new();

	for raw_line in text.lines() {
		line_number += 1;
		let line = raw_line.trim();

		if line.is_empty() {
			continue;
		}

		if line == "header" {
			section = Section::Header;
			continue;
		}

		if line == "layers" {
			section = Section::Layers;
			continue;
		}

		if line == "entities" {
			section = Section::Entities;
			continue;
		}

		if line == "triggers" {
			section = Section::Triggers;
			continue;
		}

		if line == "{" || line == "}" {
			if line == "}" {
				match section {
					Section::LayerBody => {
						let layer_source = layer.to_layer_source(line_number)?;
						layers.push(layer_source);
						section = Section::Layers;
					}
					Section::EntityBody => {
						let e = ent.to_entity_source(line_number)?;
						entities.push(e);
						section = Section::Entities;
					}
					Section::TriggerBody => {
						let t = trigger.to_trigger_source(line_number)?;
						triggers.push(t);
						section = Section::Triggers;
					}
					Section::Header => {
						section = Section::None;
					}
					_ => {}
				}
			}

			continue;
		}

		match section {
			Section::Header => {
				let header = parse_header_line(line, header_opt)?;
				header_opt = Some(header);
			}
			Section::Layers => {
				if line.starts_with("layer ") {
					layer.clear();
					let (name, _has_brace) = parse_layer_declaration(line, line_number)?;
					layer.name = name;
					reading_tiles = false;
					section = Section::LayerBody;

					// has_brace already handled '{' in the same line
				} else {
					return Err(format!("unexpected line in layers section at {}: {}", line_number, line));
				}
			}
			Section::LayerBody => {
				if line.starts_with("collision") {
					let value = parse_bool_value(line, "collision", line_number)?;
					layer.collision = value;
				} else if line.starts_with("tiles") {
					if line.ends_with("[") {
						reading_tiles = true;
					} else if line == "tiles =" || line == "tiles=" {
						reading_tiles = false;
					} else {
						return Err(format!("invalid tiles declaration at line {}", line_number));
					}
				} else if line == "[" && !reading_tiles {
					reading_tiles = true;
				} else if reading_tiles {
					if line.starts_with("]") {
						reading_tiles = false;
					} else {
						layer.rows.push(parse_tile_row(line, line_number)?);
					}
				} else {
					return Err(format!("unexpected line in layer body at {}: {}", line_number, line));
				}
			}
			Section::Entities => {
				ent.clear();
				if line.starts_with("player_start") {
					ent.kind = Some(EntityKindSource::PlayerStart);
					section = Section::EntityBody;
				} else if line.starts_with("enemy") {
					ent.clear();
					let enemy_kind = if line.starts_with("enemy ") {
						Some(parse_kind_string_after_keyword(line, "enemy", line_number)?)
					} else {
						None
					};
					ent.kind = Some(EntityKindSource::Enemy {
						enemy_kind: enemy_kind.unwrap_or_else(|| "".to_string()),
						range_min: 0,
						range_max: 0,
					});
					section = Section::EntityBody;
					continue;
				} else if line.starts_with("platform ") {
					ent.clear();
					let platform_kind = parse_kind_string_after_keyword(line, "platform", line_number)?;
					ent.kind = Some(EntityKindSource::MovingPlatform {
						platform_kind,
						size: 1,
						speed: 1,
						range_min: 0,
						range_max: 0,
					});
					section = Section::EntityBody;
				} else {
					return Err(format!("unexpected line in entities section at {}: {}", line_number, line));
				}
			}
			Section::EntityBody => {
				if line.starts_with("top") {
					ent.top = parse_i32_value(line, "top", line_number)?;
				} else if line.starts_with("left") {
					ent.left = parse_i32_value(line, "left", line_number)?;
				} else if line.starts_with("range_min") {
					let value = parse_i32_value(line, "range_min", line_number)?;

					match ent.kind.as_mut() {
						Some(EntityKindSource::Enemy { range_min, .. }) => {
							*range_min = value;
						}
						Some(EntityKindSource::MovingPlatform { range_min, .. }) => {
							*range_min = value;
						}

						_ => ent.range_min = value,
					}
				} else if line.starts_with("range_max") {
					let value = parse_i32_value(line, "range_max", line_number)?;
					match ent.kind.as_mut() {
						Some(EntityKindSource::Enemy { range_max, .. }) => {
							*range_max = value;
						}
						Some(EntityKindSource::MovingPlatform { range_max, .. }) => {
							*range_max = value;
						}

						_ => ent.range_max = value,
					}
				} else if line.starts_with("size") {
					let value = parse_i32_value(line, "size", line_number)?;
					match ent.kind.as_mut() {
						Some(EntityKindSource::MovingPlatform { size, .. }) => {
							*size = value;
						}
						_ => {
							return Err(format!("size not allowed for this entity at line {}", line_number));
						}
					}
				} else if line.starts_with("speed") {
					ent.speed = parse_i32_value(line, "speed", line_number)?;
				} else if line.starts_with("gravity_multiplier") {
					ent.gravity_multiplier = parse_f32_value(line, "gravity_multiplier", line_number)?;
				} else if line.starts_with("jump_multiplier") {
					ent.jump_multiplier = parse_f32_value(line, "jump_multiplier", line_number)?;
				} else if line.starts_with("attack_power") {
					ent.attack_power = parse_i32_value(line, "attack_power", line_number)?;
				} else if line.starts_with("hit_points") {
					ent.hit_points = parse_i32_value(line, "hit_points", line_number)?;
				} else if line.starts_with("enemy_kind") {
					let value = parse_string_value(line, "enemy_kind", line_number)?;
					match ent.kind.as_mut() {
						Some(EntityKindSource::Enemy { enemy_kind, .. }) => {
							*enemy_kind = value;
						}
						_ => {
							return Err(format!("enemy_kind not allowed for this entity at line {}", line_number));
						}
					}
				} else if line.starts_with("render_style") {
					ent.render_style = parse_u8_value(line, "render_style", line_number)?;
				} else if line.starts_with("width") {
					ent.width = parse_f32_value(line, "width", line_number)?;
				} else if line.starts_with("height") {
					ent.height = parse_f32_value(line, "height", line_number)?;
				} else if line.starts_with("speed") {
					ent.speed = parse_i32_value(line, "speed", line_number)?;
				} else if line.starts_with("strength") {
					ent.strength = parse_i32_value(line, "strength", line_number)?;
				} else if line.starts_with("luck") {
					ent.luck = parse_i32_value(line, "luck", line_number)?;
				} else if line.starts_with("health_regen_rate") {
					ent.health_regen_rate = parse_i32_value(line, "health_regen_rate", line_number)?;
				} else if line.starts_with("invulnerability_time") {
					ent.invulnerability_time = parse_i32_value(line, "invulnerability_time", line_number)?;
				} else {
					return Err(format!("Error: unexpected line in entity body at {}: {}", line_number, line));
				}
			}
			Section::Triggers => {
				if line.starts_with("trigger ") {
					trigger.clear();
					let trigger_kind = parse_kind_string_after_keyword(line, "trigger", line_number)?;

					let kind_enum = if trigger_kind == "level_exit" {
						TriggerKindSource::LevelExit {
							target: String::new(),
							level: String::new(),
							activation_mode: 0,
						}
					} else if trigger_kind == "message" {
						TriggerKindSource::Message {
							text_id: String::new(),
							activation_mode: 0,
						}
					} else if trigger_kind == "pickup" {
						TriggerKindSource::Pickup {
							pickup: String::new(),
							amount: 0,
							activation_mode: 0,
						}
					} else {
						return Err(format!("unknown trigger kind '{}' at line {}", trigger_kind, line_number));
					};
					trigger.kind = Some(kind_enum);
					section = Section::TriggerBody;
				} else {
					return Err(format!("unexpected line in triggers section at {}: {}", line_number, line));
				}
			}
			Section::TriggerBody => {
				if line.starts_with("top") {
					trigger.top = parse_i32_value(line, "top", line_number)?;
				} else if line.starts_with("left") {
					trigger.left = parse_i32_value(line, "left", line_number)?;
				} else if line.starts_with("width") {
					trigger.width = parse_i32_value(line, "width", line_number)?;
				} else if line.starts_with("icon_id") {
					trigger.icon_id = parse_i32_value(line, "icon_id", line_number)?;
				} else if line.starts_with("height") {
					trigger.height = parse_i32_value(line, "height", line_number)?;
				} else if line.starts_with("level") {
					let s: String = parse_string_value(line, "level", line_number)?;
					match trigger.kind.as_mut() {
						Some(TriggerKindSource::LevelExit { target: _, level, .. }) => {
							*level = s;
						}
						_ => {
							return Err(format!("level not allowed for this trigger at line {}", line_number));
						}
					}
				} else if line.starts_with("target") {
					let s = parse_string_value(line, "target", line_number)?;
					match trigger.kind.as_mut() {
						Some(TriggerKindSource::LevelExit { target, level: _, .. }) => {
							*target = s;
						}
						_ => {
							return Err(format!("target not allowed for this trigger at line {}", line_number));
						}
					}
				} else if line.starts_with("text_id") {
					let s = parse_string_value(line, "text_id", line_number)?;
					match trigger.kind.as_mut() {
						Some(TriggerKindSource::Message { text_id, .. }) => {
							*text_id = s;
						}
						_ => {
							return Err(format!("text_id not allowed for this trigger at line {}", line_number));
						}
					}
				} else if line.starts_with("mode") {
					let s: String = parse_string_value(line, "mode", line_number)?;
					let mode: u8 = match s.as_str() {
						"auto" => TriggerActivationMode::Auto as u8,
						"action" => TriggerActivationMode::Action as u8,
						"up" => TriggerActivationMode::Up as u8,
						"down" => TriggerActivationMode::Down as u8,
						"left" => TriggerActivationMode::Left as u8,
						"right" => TriggerActivationMode::Right as u8,
						_ => {
							return Err(format!("Invalid trigger mode '{}' at line {}", s, line_number));
						}
					};

					match trigger.kind.as_mut() {
						Some(TriggerKindSource::Message { activation_mode, .. }) => {
							*activation_mode = mode;
						}
						Some(TriggerKindSource::LevelExit { activation_mode, .. }) => {
							*activation_mode = mode;
						}
						Some(TriggerKindSource::Pickup { activation_mode, .. }) => {
							*activation_mode = mode;
						}
						_ => {
							return Err(format!("mode not allowed for this trigger at line {}", line_number));
						}
					}
				} else if line.starts_with("mode") {
					let s: String = parse_string_value(line, "mode", line_number)?;
					let mode: u8 = match s.as_str() {
						"auto" => TriggerActivationMode::Auto as u8,
						"action" => TriggerActivationMode::Action as u8,
						"up" => TriggerActivationMode::Up as u8,
						"down" => TriggerActivationMode::Down as u8,
						"left" => TriggerActivationMode::Left as u8,
						"right" => TriggerActivationMode::Right as u8,
						_ => {
							return Err(format!("Invalid trigger mode '{}' at line {}", s, line_number));
						}
					};

					match trigger.kind.as_mut() {
						Some(TriggerKindSource::Message { activation_mode, .. }) => {
							*activation_mode = mode;
						}
						Some(TriggerKindSource::LevelExit { activation_mode, .. }) => {
							*activation_mode = mode;
						}
						Some(TriggerKindSource::Pickup { activation_mode, .. }) => {
							*activation_mode = mode;
						}
						_ => {
							return Err(format!("mode not allowed for this trigger at line {}", line_number));
						}
					}
				} else if line.starts_with("pickup") {
					let s: String = parse_string_value(line, "pickup", line_number)?;
					match trigger.kind.as_mut() {
						Some(TriggerKindSource::Pickup { pickup, .. }) => {
							*pickup = s;
						}
						_ => {
							return Err(format!("pickup not allowed for this trigger at line {}", line_number));
						}
					}
				} else if line.starts_with("amount") {
					let value_i32: i32 = parse_i32_value(line, "amount", line_number)?;
					if value_i32 < 0 || value_i32 > (u16::MAX as i32) {
						return Err(format!("amount out of range at line {}", line_number));
					}

					match trigger.kind.as_mut() {
						Some(TriggerKindSource::Pickup { amount, .. }) => {
							*amount = value_i32 as u16;
						}
						_ => {
							return Err(format!("amount not allowed for this trigger at line {}", line_number));
						}
					}
				} else {
					return Err(format!("unexpected line in trigger body at {}: {}", line_number, line));
				}
			}
			Section::None => {
				return Err(format!("unexpected content outside of sections at line {}: {}", line_number, line));
			}
		}
	}

	let header = match header_opt {
		Some(h) => h,
		None => {
			return Err("missing header section".to_string());
		}
	};

	let level = LevelSource {
		header,
		layers,
		entities,
		triggers,
	};

	return Ok(level);
}

fn parse_header_line(line: &str, existing: Option<LevelHeader>) -> Result<LevelHeader, String> {
	let mut header = match existing {
		Some(h) => h,
		None => LevelHeader {
			version: 1,
			name: String::new(),
			author: String::new(),
			width: 0,
			height: 0,
			tile_height: 0,
			tile_width: 0,
			gravity: 0.0,
			background: String::new(),
		},
	};

	let (key, value_str) = split_key_value(line)?;

	match key {
		"version" => {
			header.version = value_str.parse::<u32>().map_err(|e| format!("invalid version value '{}': {}", value_str, e))?;
		}
		"name" => {
			header.name = parse_quoted(value_str)?;
		}
		"author" => {
			header.author = parse_quoted(value_str)?;
		}
		"width" => {
			header.width = value_str.parse::<u32>().map_err(|e| format!("invalid width value '{}': {}", value_str, e))?;
		}
		"height" => {
			header.height = value_str.parse::<u32>().map_err(|e| format!("invalid height value '{}': {}", value_str, e))?;
		}
		"tile_width" => {
			header.tile_width = value_str.parse::<u32>().map_err(|e| format!("invalid tile_width value '{}': {}", value_str, e))?;
		}
		"tile_height" => {
			header.tile_height = value_str.parse::<u32>().map_err(|e| format!("invalid tile_height value '{}': {}", value_str, e))?;
		}
		"gravity" => {
			header.gravity = value_str.parse::<f32>().map_err(|e| format!("invalid gravity value '{}': {}", value_str, e))?;
		}
		"background" => {
			header.background = parse_quoted(value_str)?;
		}
		_ => {
			return Err(format!("unknown header key '{}'", key));
		}
	}

	return Ok(header);
}

fn parse_layer_declaration(line: &str, line_number: usize) -> Result<(String, bool), String> {
	let rest = line.trim_start_matches("layer").trim();
	let mut has_brace = false;
	let name_part: String;

	if let Some(pos) = rest.find('{') {
		has_brace = true;
		name_part = rest[..pos].trim().to_string();
	} else {
		name_part = rest.to_string();
	}

	let name = parse_quoted(name_part.trim()).map_err(|e| format!("invalid layer name at line {}: {}", line_number, e))?;
	return Ok((name, has_brace));
}

fn parse_tile_row(line: &str, line_number: usize) -> Result<String, String> {
	let trimmed = line.trim();
	let s = parse_quoted(trimmed).map_err(|e| format!("invalid tile row at line {}: {}", line_number, e))?;
	return Ok(s);
}

fn parse_bool_value(line: &str, expected_key: &str, line_number: usize) -> Result<bool, String> {
	let (key, value_str) = split_key_value(line)?;
	if key != expected_key {
		return Err(format!("expected key '{}' at line {}, got '{}'", expected_key, line_number, key));
	}

	if value_str == "true" {
		return Ok(true);
	}

	if value_str == "false" {
		return Ok(false);
	}

	return Err(format!("invalid bool value '{}' at line {}", value_str, line_number));
}

fn parse_f32_value(line: &str, expected_key: &str, line_number: usize) -> Result<f32, String> {
	let (key, value_str) = split_key_value(line)?;
	if key != expected_key {
		return Err(format!("expected key '{}' at line {}, got '{}'", expected_key, line_number, key));
	}

	let value = value_str
		.parse::<f32>()
		.map_err(|e| format!("invalid float value '{}' at line {}: {}", value_str, line_number, e))?;

	return Ok(value);
}

fn parse_i32_value(line: &str, expected_key: &str, line_number: usize) -> Result<i32, String> {
	let (key, value_str) = split_key_value(line)?;
	if key != expected_key {
		return Err(format!("expected key '{}' at line {}, got '{}'", expected_key, line_number, key));
	}

	let value = value_str
		.parse::<i32>()
		.map_err(|e| format!("invalid integer value '{}' at line {}: {}", value_str, line_number, e))?;
	return Ok(value);
}

fn parse_u8_value(line: &str, expected_key: &str, line_number: usize) -> Result<u8, String> {
	let (key, value_str) = split_key_value(line)?;
	if key != expected_key {
		return Err(format!("expected key '{}' at line {}, got '{}'", expected_key, line_number, key));
	}

	let value = value_str
		.parse::<u8>()
		.map_err(|e| format!("invalid integer value '{}' at line {}: {}", value_str, line_number, e))?;
	return Ok(value);
}

fn parse_string_value(line: &str, expected_key: &str, line_number: usize) -> Result<String, String> {
	let (key, value_str) = split_key_value(line)?;
	if key != expected_key {
		return Err(format!("expected key '{}' at line {}, got '{}'", expected_key, line_number, key));
	}

	let s = parse_quoted(value_str).map_err(|e| format!("invalid string value at line {}: {}", line_number, e))?;
	return Ok(s);
}

fn parse_kind_string_after_keyword(line: &str, keyword: &str, line_number: usize) -> Result<String, String> {
	let rest = line.trim_start_matches(keyword).trim();
	let s = parse_quoted(rest).map_err(|e| format!("invalid kind string after '{}' at line {}: {}", keyword, line_number, e))?;
	return Ok(s);
}

fn split_key_value(line: &str) -> Result<(&str, &str), String> {
	let parts: Vec<&str> = line.splitn(2, '=').collect();
	if parts.len() != 2 {
		return Err(format!("expected key = value, got '{}'", line));
	}

	let key = parts[0].trim();
	let value_str = parts[1].trim();
	return Ok((key, value_str));
}

fn parse_quoted(value_str: &str) -> Result<String, String> {
	let s = value_str.trim();
	let bytes = s.as_bytes();
	if bytes.len() < 2 || bytes[0] != b'"' || bytes[bytes.len() - 1] != b'"' {
		return Err(format!("expected quoted string, got '{}'", s));
	}

	let inner = &s[1..s.len() - 1];
	let result = inner.to_string();
	return Ok(result);
}

#[allow(dead_code)]
fn parse_enemy_kind(value: &str, line_number: usize) -> Result<u8, String> {
	match value {
		"slime" => Ok(1),
		"imp" => Ok(2),
		_ => Err(format!("invalid enemy_kind '{}' at line {}", value, line_number)),
	}
}
