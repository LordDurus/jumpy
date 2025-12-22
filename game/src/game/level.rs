use crate::tile::TileKind;
use std::fs;

pub struct Level {
	pub tile_width: u32,
	pub tile_height: u32,
	pub tiles: Vec<u8>,
	pub width: u32,
	pub height: u32,
	pub floor_y: f32,
}

impl Level {
	pub fn is_solid_world_f32(&self, world_x: f32, world_y: f32) -> bool {
		let wx: i32 = world_x.floor() as i32;
		let wy: i32 = world_y.floor() as i32;
		return self.is_solid_world_i32(wx, wy);
	}

	pub fn is_solid_world_i32(&self, world_x: i32, world_y: i32) -> bool {
		let tile_x: i32 = world_x / self.tile_width as i32;
		let tile_y: i32 = world_y / self.tile_height as i32;

		let kind: TileKind = self.tile_at(tile_x, tile_y);
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
}

impl Level {
	pub fn load_binary(path: &str) -> Result<Level, String> {
		let bytes = fs::read(path).map_err(|e| e.to_string())?;
		if bytes.len() < 4 {
			return Err("file too small".to_string());
		}

		if &bytes[0..4] != b"JLVL" {
			return Err("bad magic (expected JLVL)".to_string());
		}

		let mut offset: usize = 4;

		// ---- header ----
		let version = read_u16(&bytes, &mut offset)?;
		let header_size = read_u16(&bytes, &mut offset)? as usize;

		let width = read_u16(&bytes, &mut offset)? as u32;
		let height = read_u16(&bytes, &mut offset)? as u32;

		// new format: tile_width + tile_height are both u16
		let tile_width = read_u16(&bytes, &mut offset)? as u32;
		let tile_height = read_u16(&bytes, &mut offset)? as u32;

		let layer_count = read_u8(&bytes, &mut offset)? as u32;

		let _entity_count = read_u16(&bytes, &mut offset)? as u32;
		let _trigger_count = read_u16(&bytes, &mut offset)? as u32;

		let _gravity_fixed = read_i16(&bytes, &mut offset)?;
		let _background_id = read_u8(&bytes, &mut offset)?;
		let _gravity_multiplier = read_u8(&bytes, &mut offset)?;
		let _reserved1 = read_u16(&bytes, &mut offset)?;

		let tiles_per_layer = read_u32(&bytes, &mut offset)? as usize;
		let tile_count_total = read_u32(&bytes, &mut offset)? as usize;

		let _offset_layers = read_u32(&bytes, &mut offset)? as usize;
		let _offset_entities = read_u32(&bytes, &mut offset)? as usize;
		let _offset_triggers = read_u32(&bytes, &mut offset)? as usize;
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

		let mut level = Level {
			tile_width,
			tile_height,
			tiles,
			width,
			height,
			floor_y: 0.0,
		};

		level.floor_y = level.compute_floor_y();

		return Ok(level);
	}

	#[allow(dead_code)]
	fn read_i16(bytes: &[u8], offset: &mut usize) -> Result<i16, String> {
		if *offset + 2 > bytes.len() {
			return Err("Unexpected eof reading i16".to_string());
		}
		let v = i16::from_le_bytes([bytes[*offset], bytes[*offset + 1]]);
		*offset += 2;
		return Ok(v);
	}

	fn compute_floor_y(&self) -> f32 {
		if self.width == 0 || self.height == 0 {
			return 0.0;
		}

		let expected = (self.width as usize) * (self.height as usize);
		if expected != self.tiles.len() {
			return 0.0;
		}

		for row in (0..self.height).rev() {
			for col in 0..self.width {
				let idx = (row * self.width + col) as usize;
				if self.tiles[idx] != 0 {
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
