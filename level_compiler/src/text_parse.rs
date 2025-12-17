use crate::source::*;

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

    let mut header_opt: Option<LevelHeader> = None;
    let mut layers: Vec<LayerSource> = Vec::new();
    let mut current_layer_name = String::new();
    let mut current_layer_collision = false;
    let mut current_layer_rows: Vec<String> = Vec::new();
    let mut reading_tiles = false;
    let mut entities: Vec<EntitySource> = Vec::new();
    let mut current_entity_kind: Option<EntityKindSource> = None;
    let mut current_entity_x: i32 = 0;
    let mut current_entity_y: i32 = 0;
    let mut triggers: Vec<TriggerSource> = Vec::new();
    let mut current_trigger_kind: Option<TriggerKindSource> = None;
    let mut current_trigger_x: i32 = 0;
    let mut current_trigger_y: i32 = 0;
    let mut current_trigger_width: i32 = 0;
    let mut current_trigger_height: i32 = 0;
    let mut line_number = 0;
    let mut current_entity_gravity_multiplier: f32 = 1.0;

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
                        let layer = LayerSource {
                            name: current_layer_name.clone(),
                            collision: current_layer_collision,
                            rows: current_layer_rows.clone(),
                        };
                        layers.push(layer);

                        current_layer_name.clear();
                        current_layer_collision = false;
                        current_layer_rows.clear();
                        reading_tiles = false;

                        section = Section::Layers;
                    }
                    Section::EntityBody => {
                        let kind = match current_entity_kind.take() {
                            Some(k) => k,
                            None => {
                                return Err(format!(
                                    "entity body closed without kind at line {}",
                                    line_number
                                ));
                            }
                        };

                        let entity = EntitySource {
                            x: current_entity_x,
                            y: current_entity_y,
                            gravity_multiplier: current_entity_gravity_multiplier,
                            kind,
                        };
                        entities.push(entity);

                        section = Section::Entities;
                    }
                    Section::TriggerBody => {
                        let kind = match current_trigger_kind.take() {
                            Some(k) => k,
                            None => {
                                return Err(format!(
                                    "trigger body closed without kind at line {}",
                                    line_number
                                ));
                            }
                        };

                        let trigger = TriggerSource {
                            x: current_trigger_x,
                            y: current_trigger_y,
                            width: current_trigger_width,
                            height: current_trigger_height,
                            kind,
                        };
                        triggers.push(trigger);

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
                    let (name, has_brace) = parse_layer_declaration(line, line_number)?;
                    current_layer_name = name;
                    current_layer_collision = false;
                    current_layer_rows.clear();
                    reading_tiles = false;
                    section = Section::LayerBody;

                    if has_brace {
                        // already handled '{' in the same line
                    }
                } else {
                    return Err(format!(
                        "unexpected line in layers section at {}: {}",
                        line_number, line
                    ));
                }
            }
            Section::LayerBody => {
                if line.starts_with("collision") {
                    let value = parse_bool_value(line, "collision", line_number)?;
                    current_layer_collision = value;
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
                        let row = parse_tile_row(line, line_number)?;
                        current_layer_rows.push(row);
                    }
                } else {
                    return Err(format!(
                        "unexpected line in layer body at {}: {}",
                        line_number, line
                    ));
                }
            }
            Section::Entities => {
                if line.starts_with("player_start") {
                    current_entity_kind = Some(EntityKindSource::PlayerStart);
                    current_entity_x = 0;
                    current_entity_y = 0;
                    current_entity_gravity_multiplier = 1.0;
                    section = Section::EntityBody;
                } else if line.starts_with("enemy ") {
                    let enemy_kind = parse_kind_string_after_keyword(line, "enemy", line_number)?;
                    current_entity_kind = Some(EntityKindSource::Enemy {
                        enemy_kind,
                        patrol_min: 0,
                        patrol_max: 0,
                    });
                    current_entity_x = 0;
                    current_entity_y = 0;
                    section = Section::EntityBody;
                } else if line.starts_with("pickup ") {
                    let pickup_kind = parse_kind_string_after_keyword(line, "pickup", line_number)?;
                    current_entity_kind = Some(EntityKindSource::Pickup {
                        pickup_kind,
                        value: 0,
                    });
                    current_entity_x = 0;
                    current_entity_y = 0;
                    section = Section::EntityBody;
                } else if line.starts_with("platform ") {
                    let platform_kind =
                        parse_kind_string_after_keyword(line, "platform", line_number)?;

                    if platform_kind != "horizontal" && platform_kind != "vertical" {
                        return Err(format!(
                            "unknown platform kind '{}' at line {} (expected \"horizontal\" or \"vertical\")",
                            platform_kind, line_number
                        ));
                    }

                    current_entity_kind = Some(EntityKindSource::MovingPlatform {
                        platform_kind,
                        size: 1,
                        speed: 1,
                        min: 0,
                        max: 0,
                    });
                    current_entity_x = 0;
                    current_entity_y = 0;
                    section = Section::EntityBody;
                } else {
                    return Err(format!(
                        "unexpected line in entities section at {}: {}",
                        line_number, line
                    ));
                }
            }
            Section::EntityBody => {
                if line.starts_with("x") {
                    let x = parse_i32_value(line, "x", line_number)?;
                    current_entity_x = x;
                } else if line.starts_with("y") {
                    let y = parse_i32_value(line, "y", line_number)?;
                    current_entity_y = y;
                } else if line.starts_with("patrol_min") {
                    let value = parse_i32_value(line, "patrol_min", line_number)?;
                    match current_entity_kind {
                        Some(EntityKindSource::Enemy {
                            ref mut patrol_min, ..
                        }) => {
                            *patrol_min = value;
                        }
                        _ => {
                            return Err(format!(
                                "patrol_min not allowed for this entity at line {}",
                                line_number
                            ));
                        }
                    }
                } else if line.starts_with("patrol_max") {
                    let value = parse_i32_value(line, "patrol_max", line_number)?;
                    match current_entity_kind {
                        Some(EntityKindSource::Enemy {
                            ref mut patrol_max, ..
                        }) => {
                            *patrol_max = value;
                        }
                        _ => {
                            return Err(format!(
                                "patrol_max not allowed for this entity at line {}",
                                line_number
                            ));
                        }
                    }
                } else if line.starts_with("size") {
                    let value = parse_i32_value(line, "size", line_number)?;
                    match current_entity_kind {
                        Some(EntityKindSource::MovingPlatform { ref mut size, .. }) => {
                            *size = value;
                        }
                        _ => {
                            return Err(format!(
                                "size not allowed for this entity at line {}",
                                line_number
                            ));
                        }
                    }
                } else if line.starts_with("speed") {
                    let value = parse_i32_value(line, "speed", line_number)?;
                    match current_entity_kind {
                        Some(EntityKindSource::MovingPlatform { ref mut speed, .. }) => {
                            *speed = value;
                        }
                        _ => {
                            return Err(format!(
                                "speed not allowed for this entity at line {}",
                                line_number
                            ));
                        }
                    }
                } else if line.starts_with("min") {
                    let value = parse_i32_value(line, "min", line_number)?;
                    match current_entity_kind {
                        Some(EntityKindSource::MovingPlatform { ref mut min, .. }) => {
                            *min = value;
                        }
                        _ => {
                            return Err(format!(
                                "min not allowed for this entity at line {}",
                                line_number
                            ));
                        }
                    }
                } else if line.starts_with("max") {
                    let value = parse_i32_value(line, "max", line_number)?;
                    match current_entity_kind {
                        Some(EntityKindSource::MovingPlatform { ref mut max, .. }) => {
                            *max = value;
                        }
                        _ => {
                            return Err(format!(
                                "max not allowed for this entity at line {}",
                                line_number
                            ));
                        }
                    }
                } else if line.starts_with("value") {
                    let value = parse_i32_value(line, "value", line_number)?;
                    match current_entity_kind {
                        Some(EntityKindSource::Pickup {
                            value: ref mut v, ..
                        }) => {
                            *v = value;
                        }
                        _ => {
                            return Err(format!(
                                "Error: value not allowed for this entity at line {}",
                                line_number
                            ));
                        }
                    }
                } else if line.starts_with("gravity") {
                    let v = parse_f32_value(line, "gravity", line_number)?;
                    current_entity_gravity_multiplier = v;
                } else {
                    return Err(format!(
                        "Error: unexpected line in entity body at {}: {}",
                        line_number, line
                    ));
                }
            }
            Section::Triggers => {
                if line.starts_with("trigger ") {
                    let trigger_kind =
                        parse_kind_string_after_keyword(line, "trigger", line_number)?;
                    current_trigger_x = 0;
                    current_trigger_y = 0;
                    current_trigger_width = 0;
                    current_trigger_height = 0;

                    let kind_enum = if trigger_kind == "level_exit" {
                        TriggerKindSource::LevelExit {
                            target: String::new(),
                        }
                    } else if trigger_kind == "message" {
                        TriggerKindSource::Message {
                            text_id: String::new(),
                        }
                    } else {
                        return Err(format!(
                            "unknown trigger kind '{}' at line {}",
                            trigger_kind, line_number
                        ));
                    };

                    current_trigger_kind = Some(kind_enum);
                    section = Section::TriggerBody;
                } else {
                    return Err(format!(
                        "unexpected line in triggers section at {}: {}",
                        line_number, line
                    ));
                }
            }
            Section::TriggerBody => {
                if line.starts_with("x") {
                    let x = parse_i32_value(line, "x", line_number)?;
                    current_trigger_x = x;
                } else if line.starts_with("y") {
                    let y = parse_i32_value(line, "y", line_number)?;
                    current_trigger_y = y;
                } else if line.starts_with("width") {
                    let value = parse_i32_value(line, "width", line_number)?;
                    current_trigger_width = value;
                } else if line.starts_with("height") {
                    let value = parse_i32_value(line, "height", line_number)?;
                    current_trigger_height = value;
                } else if line.starts_with("target") {
                    let s = parse_string_value(line, "target", line_number)?;
                    match current_trigger_kind {
                        Some(TriggerKindSource::LevelExit { ref mut target, .. }) => {
                            *target = s;
                        }
                        _ => {
                            return Err(format!(
                                "target not allowed for this trigger at line {}",
                                line_number
                            ));
                        }
                    }
                } else if line.starts_with("text_id") {
                    let s = parse_string_value(line, "text_id", line_number)?;
                    match current_trigger_kind {
                        Some(TriggerKindSource::Message {
                            ref mut text_id, ..
                        }) => {
                            *text_id = s;
                        }
                        _ => {
                            return Err(format!(
                                "text_id not allowed for this trigger at line {}",
                                line_number
                            ));
                        }
                    }
                } else {
                    return Err(format!(
                        "unexpected line in trigger body at {}: {}",
                        line_number, line
                    ));
                }
            }
            Section::None => {
                return Err(format!(
                    "unexpected content outside of sections at line {}: {}",
                    line_number, line
                ));
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
            tile_size: 0,
            gravity: 0.0,
            background: String::new(),
        },
    };

    let (key, value_str) = split_key_value(line)?;

    match key {
        "version" => {
            header.version = value_str
                .parse::<u32>()
                .map_err(|e| format!("invalid version value '{}': {}", value_str, e))?;
        }
        "name" => {
            header.name = parse_quoted(value_str)?;
        }
        "author" => {
            header.author = parse_quoted(value_str)?;
        }
        "width" => {
            header.width = value_str
                .parse::<u32>()
                .map_err(|e| format!("invalid width value '{}': {}", value_str, e))?;
        }
        "height" => {
            header.height = value_str
                .parse::<u32>()
                .map_err(|e| format!("invalid height value '{}': {}", value_str, e))?;
        }
        "tile_size" => {
            header.tile_size = value_str
                .parse::<u32>()
                .map_err(|e| format!("invalid tile_size value '{}': {}", value_str, e))?;
        }
        "gravity" => {
            header.gravity = value_str
                .parse::<f32>()
                .map_err(|e| format!("invalid gravity value '{}': {}", value_str, e))?;
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

    let name = parse_quoted(name_part.trim())
        .map_err(|e| format!("invalid layer name at line {}: {}", line_number, e))?;
    return Ok((name, has_brace));
}

fn parse_tile_row(line: &str, line_number: usize) -> Result<String, String> {
    let trimmed = line.trim();
    let s = parse_quoted(trimmed)
        .map_err(|e| format!("invalid tile row at line {}: {}", line_number, e))?;
    return Ok(s);
}

fn parse_bool_value(line: &str, expected_key: &str, line_number: usize) -> Result<bool, String> {
    let (key, value_str) = split_key_value(line)?;
    if key != expected_key {
        return Err(format!(
            "expected key '{}' at line {}, got '{}'",
            expected_key, line_number, key
        ));
    }

    if value_str == "true" {
        return Ok(true);
    }

    if value_str == "false" {
        return Ok(false);
    }

    return Err(format!(
        "invalid bool value '{}' at line {}",
        value_str, line_number
    ));
}

fn parse_f32_value(line: &str, expected_key: &str, line_number: usize) -> Result<f32, String> {
    let (key, value_str) = split_key_value(line)?;
    if key != expected_key {
        return Err(format!(
            "expected key '{}' at line {}, got '{}'",
            expected_key, line_number, key
        ));
    }

    let value = value_str.parse::<f32>().map_err(|e| {
        format!(
            "invalid float value '{}' at line {}: {}",
            value_str, line_number, e
        )
    })?;

    return Ok(value);
}

fn parse_i32_value(line: &str, expected_key: &str, line_number: usize) -> Result<i32, String> {
    let (key, value_str) = split_key_value(line)?;
    if key != expected_key {
        return Err(format!(
            "expected key '{}' at line {}, got '{}'",
            expected_key, line_number, key
        ));
    }

    let value = value_str.parse::<i32>().map_err(|e| {
        format!(
            "invalid integer value '{}' at line {}: {}",
            value_str, line_number, e
        )
    })?;
    return Ok(value);
}

fn parse_string_value(
    line: &str,
    expected_key: &str,
    line_number: usize,
) -> Result<String, String> {
    let (key, value_str) = split_key_value(line)?;
    if key != expected_key {
        return Err(format!(
            "expected key '{}' at line {}, got '{}'",
            expected_key, line_number, key
        ));
    }

    let s = parse_quoted(value_str)
        .map_err(|e| format!("invalid string value at line {}: {}", line_number, e))?;
    return Ok(s);
}

fn parse_kind_string_after_keyword(
    line: &str,
    keyword: &str,
    line_number: usize,
) -> Result<String, String> {
    let rest = line.trim_start_matches(keyword).trim();
    let s = parse_quoted(rest).map_err(|e| {
        format!(
            "invalid kind string after '{}' at line {}: {}",
            keyword, line_number, e
        )
    })?;
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
