use crate::tile::TileKind;
use std::fs;

pub const BYTES_PER_ENTITY: usize = 20;

#[derive(Debug, Clone)]
pub struct Level {
	pub tile_width: u32,
	pub tile_height: u32,
	pub tiles: Vec<u8>,
	pub width: u32,
	pub height: u32,
	pub floor_y: f32,
	pub layer_count: u8,
	pub tiles_per_layer: usize,
	pub player_spawn_top: f32,
	pub player_spawn_left: f32,
	pub collision_layer: u8,
	pub render_layer: u8,
	pub entities: Vec<LevelEntity>,
	pub triggers: Vec<LevelTrigger>,
}

impl Level {
	pub fn is_solid_world_f32(&self, level_x: f32, level_y: f32) -> bool {
		let tile_w: f32 = self.tile_width as f32;
		let tile_h: f32 = self.tile_height as f32;

		let tile_x: i32 = (level_x / tile_w).floor() as i32;
		let tile_y: i32 = (level_y / tile_h).floor() as i32;

		let layer: u32 = self.collision_layer_index() as u32;
		let kind: TileKind = self.tile_at_layer(layer, tile_x, tile_y);
		return kind.is_solid();
	}

	pub fn tile_at(&self, tx: i32, ty: i32) -> TileKind {
		if tx < 0 || ty < 0 {
			return TileKind::Empty;
		}

		let x: usize = tx as usize;
		let y: usize = ty as usize;

		if x >= self.width as usize || y >= self.height as usize {
			return TileKind::Empty;
		}

		let idx: usize = y * (self.width as usize) + x;
		let v: u8 = self.tiles[idx];
		return TileKind::from_u8(v);
	}

	pub fn collision_layer_index(&self) -> u8 {
		return self.collision_layer;
	}

	pub fn tile_at_layer(&self, layer: u32, tx: i32, ty: i32) -> TileKind {
		if tx < 0 || ty < 0 {
			return TileKind::Empty;
		}

		let x: usize = tx as usize;
		let y: usize = ty as usize;

		if x >= self.width as usize || y >= self.height as usize {
			return TileKind::Empty;
		}

		let layer_usize: usize = layer as usize;
		let idx_in_layer: usize = y * (self.width as usize) + x;
		let idx: usize = layer_usize * self.tiles_per_layer + idx_in_layer;

		if idx >= self.tiles.len() {
			return TileKind::Empty;
		}

		let v: u8 = self.tiles[idx];
		return TileKind::from_u8(v);
	}

	pub fn load_binary(path: &str) -> Result<Level, String> {
		println!("loading file: {}", path);

		let bytes = fs::read(path).map_err(|e| e.to_string())?;
		if bytes.len() < 4 {
			return Err("file too small".to_string());
		}

		if &bytes[0..4] != b"JLVL" {
			return Err("bad magic (expected JLVL)".to_string());
		}

		let mut offset: usize = 4;

		// ---- header ----
		let _version = read_u16(&bytes, &mut offset)?;
		let header_size = read_u16(&bytes, &mut offset)? as usize;

		let width = read_u16(&bytes, &mut offset)? as u32;
		let height = read_u16(&bytes, &mut offset)? as u32;

		// new format: tile_width + tile_height are both u16
		let tile_width = read_u16(&bytes, &mut offset)? as u32;
		let tile_height = read_u16(&bytes, &mut offset)? as u32;

		let layer_count = read_u8(&bytes, &mut offset)? as u32;

		let entity_count = read_u16(&bytes, &mut offset)? as usize;
		let trigger_count = read_u16(&bytes, &mut offset)? as usize;

		let _gravity_fixed = read_i16(&bytes, &mut offset)?;
		let _background_id = read_u8(&bytes, &mut offset)?;
		let _gravity = read_u8(&bytes, &mut offset)?;

		let collision_layer = read_u8(&bytes, &mut offset)?;
		let render_layer = read_u8(&bytes, &mut offset)?;

		let tiles_per_layer = read_u32(&bytes, &mut offset)? as usize;
		let tile_count_total = read_u32(&bytes, &mut offset)? as usize;

		let _offset_layers = read_u32(&bytes, &mut offset)? as usize;
		let offset_entities = read_u32(&bytes, &mut offset)? as usize;
		let offset_triggers = read_u32(&bytes, &mut offset)? as usize;
		let offset_tiles = read_u32(&bytes, &mut offset)? as usize;

		// sanity: header_size should not point past file
		if header_size > bytes.len() {
			return Err(format!("header_size {} past file len {}", header_size, bytes.len()));
		}

		// tiles sanity
		let expected_tiles_per_layer = (width as usize) * (height as usize);
		if tiles_per_layer != expected_tiles_per_layer {
			return Err(format!(
				"tiles_per_layer mismatch: header {} vs computed {} ({}x{})",
				tiles_per_layer, expected_tiles_per_layer, width, height
			));
		}
		let expected_total = expected_tiles_per_layer * (layer_count as usize);
		if tile_count_total != expected_total {
			return Err(format!(
				"tile_count_total mismatch: header {} vs computed {} (tiles_per_layer {} * layers {})",
				tile_count_total, expected_total, expected_tiles_per_layer, layer_count
			));
		}

		if offset_tiles + tile_count_total > bytes.len() {
			return Err(format!(
				"tile section out of range: offset_tiles={} tile_count_total={} file_len={}",
				offset_tiles,
				tile_count_total,
				bytes.len()
			));
		}

		let tiles: Vec<u8> = bytes[offset_tiles..offset_tiles + tile_count_total].to_vec();

		let expected_len: usize = (layer_count as usize) * tiles_per_layer;
		if tiles.len() != expected_len {
			return Err(format!("invalid tile data: expected {} bytes, got {}", expected_len, tiles.len()));
		}

		// ---- entities ----
		let mut entities: Vec<LevelEntity> = Vec::new();

		let expected_entities_bytes: usize = entity_count * BYTES_PER_ENTITY;

		if offset_entities + expected_entities_bytes > bytes.len() {
			return Err(format!(
				"entity section out of range: offset_entities={} entity_count={} file_len={}",
				offset_entities,
				entity_count,
				bytes.len()
			));
		}

		entities.reserve(entity_count as usize);

		let mut ent_off: usize = offset_entities;

		for _ in 0..entity_count {
			entities.push(LevelEntity {
				kind: read_u8(&bytes, &mut ent_off)?,
				render_style: read_u8(&bytes, &mut ent_off)?,
				gravity_multiplier: read_u8(&bytes, &mut ent_off)?,
				jump_multiplier: read_u8(&bytes, &mut ent_off)?,
				attack_power: read_u8(&bytes, &mut ent_off)?,
				hit_points: read_u16(&bytes, &mut ent_off)?,
				top: read_u16(&bytes, &mut ent_off)?,
				left: read_u16(&bytes, &mut ent_off)?,
				a: read_i16(&bytes, &mut ent_off)?,
				b: read_i16(&bytes, &mut ent_off)?,
				width: read_u8(&bytes, &mut ent_off)?,
				height: read_u8(&bytes, &mut ent_off)?,
				speed: read_u8(&bytes, &mut ent_off)?,
				strength: read_u8(&bytes, &mut ent_off)?,
				luck: read_u8(&bytes, &mut ent_off)?,
			});
		}

		println!("-- entities loaded --");
		for (i, e) in entities.iter().enumerate() {
			println!(
				" {}: kind={} style={} top={} left={} a={} b={} width={} height={} speed={} strength={} luck={} hit_points={}",
				i, e.kind, e.render_style, e.top, e.left, e.a, e.b, e.width, e.height, e.speed, e.strength, e.luck, e.hit_points
			);
		}

		let mut player_spawn_top: f32 = 0.0;
		let mut player_spawn_left: f32 = 0.0;
		let mut found_spawn: bool = false;

		for e in &entities {
			// PlayerStart = 0 (matches your compiler runtime enum)
			if e.kind == 0 {
				player_spawn_left = (e.left as f32 + 0.5) * tile_width as f32;
				player_spawn_top = (e.top as f32 + 0.5) * tile_height as f32;
				found_spawn = true;
				break;
			}
		}

		if !found_spawn {
			return Err("level has no PlayerStart entity".to_string());
		}

		// entities
		let entities_bytes: usize = entity_count * BYTES_PER_ENTITY;

		if offset_entities + entities_bytes > bytes.len() {
			return Err(format!("entity section out of range"));
		}

		// triggers
		let bytes_per_trigger: usize = 15;
		let triggers_bytes: usize = trigger_count * bytes_per_trigger;

		if offset_triggers + triggers_bytes > bytes.len() {
			return Err(format!(
				"trigger section out of range: offset_triggers={} trigger_count={} file_len={}",
				offset_triggers,
				trigger_count,
				bytes.len()
			));
		}

		let mut trigger_offset: usize = offset_triggers;
		let mut triggers: Vec<LevelTrigger> = Vec::with_capacity(trigger_count);

		for _ in 0..trigger_count {
			triggers.push(LevelTrigger {
				kind: read_u8(&bytes, &mut trigger_offset)?,
				x: read_u16(&bytes, &mut trigger_offset)?,
				y: read_u16(&bytes, &mut trigger_offset)?,
				width: read_u16(&bytes, &mut trigger_offset)?,
				height: read_u16(&bytes, &mut trigger_offset)?,
				target: 0,
				text_id: 0,
			});
		}

		let mut level = Level {
			tile_width,
			tile_height,
			tiles,
			width,
			height,
			floor_y: 0.0,
			layer_count: layer_count as u8,
			tiles_per_layer: tiles_per_layer,
			player_spawn_top,
			player_spawn_left,
			entities: entities,
			triggers: triggers,
			collision_layer: collision_layer,
			render_layer: render_layer,
		};

		level.floor_y = level.compute_floor_y();

		return Ok(level);
	}

	fn compute_floor_y(&self) -> f32 {
		if self.width == 0 || self.height == 0 {
			return 0.0;
		}

		let expected: usize = (self.width as usize) * (self.height as usize) * (self.layer_count as usize);

		if expected != self.tiles.len() {
			return 0.0;
		}

		let layer: u32 = self.collision_layer_index() as u32;

		for row in (0..self.height).rev() {
			for col in 0..self.width {
				let kind: TileKind = self.tile_at_layer(layer, col as i32, row as i32);
				if kind != TileKind::Empty {
					return row as f32 * self.tile_height as f32;
				}
			}
		}

		return 0.0;
	}
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> Result<u32, String> {
	if *offset + 4 > bytes.len() {
		return Err("Unexpected eof reading u32".to_string());
	}
	let v = u32::from_le_bytes([bytes[*offset], bytes[*offset + 1], bytes[*offset + 2], bytes[*offset + 3]]);
	*offset += 4;
	return Ok(v);
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> Result<u16, String> {
	if *offset + 2 > bytes.len() {
		return Err("Unexpected eof reading u16".to_string());
	}
	let v = u16::from_le_bytes([bytes[*offset], bytes[*offset + 1]]);
	*offset += 2;
	return Ok(v);
}

fn read_u8(bytes: &[u8], offset: &mut usize) -> Result<u8, String> {
	if *offset + 1 > bytes.len() {
		return Err("Unexpected eof reading u8".to_string());
	}
	let v = bytes[*offset];
	*offset += 1;
	return Ok(v);
}

fn read_i16(bytes: &[u8], offset: &mut usize) -> Result<i16, String> {
	if *offset + 2 > bytes.len() {
		return Err("Unexpected eof reading i16".to_string());
	}
	let v = i16::from_le_bytes([bytes[*offset], bytes[*offset + 1]]);
	*offset += 2;
	return Ok(v);
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LevelEntity {
	pub kind: u8,
	pub render_style: u8,
	pub gravity_multiplier: u8,
	pub jump_multiplier: u8,
	pub attack_power: u8,
	pub hit_points: u16,
	pub top: u16,
	pub left: u16,
	pub a: i16,
	pub b: i16,
	pub width: u8,
	pub height: u8,
	pub speed: u8,
	pub strength: u8,
	pub luck: u8,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LevelTrigger {
	pub kind: u8,
	pub x: u16,
	pub y: u16,
	pub width: u16,
	pub height: u16,
	pub target: u16,
	pub text_id: u16,
}
