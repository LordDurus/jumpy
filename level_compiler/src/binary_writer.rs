use crate::runtime::*;

fn build_header_bytes(h: &FileHeader) -> Result<Vec<u8>, String> {
	let mut hdr: Vec<u8> = Vec::new();
	write_header(&mut hdr, h)?;
	return Ok(hdr);
}

pub fn serialize_level(compiled: &CompiledLevel) -> Result<Vec<u8>, String> {
	let hdr_probe = build_header_bytes(&compiled.header)?;
	let header_size: usize = hdr_probe.len();

	let mut buffer: Vec<u8> = Vec::new();
	buffer.resize(header_size, 0);

	let offset_layers: u32 = buffer.len() as u32;
	for layer in &compiled.layers {
		write_u8(&mut buffer, layer.collision)?;
		write_u8(&mut buffer, layer.gravity_multiplier)?;
		write_u8(&mut buffer, layer.reserved1)?;
		write_u8(&mut buffer, layer.reserved2)?;
	}

	let offset_entities: u32 = buffer.len() as u32;
	for entity in &compiled.entities {
		write_u8(&mut buffer, entity.kind)?;
		write_u8(&mut buffer, entity.render_style)?;
		write_u8(&mut buffer, entity.gravity_multiplier)?;
		write_u8(&mut buffer, entity.jump_multiplier)?;
		write_u8(&mut buffer, entity.attack_power)?;
		write_u16(&mut buffer, entity.hit_points)?;
		write_u16(&mut buffer, entity.top)?;
		write_u16(&mut buffer, entity.left)?;
		write_i16(&mut buffer, entity.a)?;
		write_i16(&mut buffer, entity.b)?;
		write_u8(&mut buffer, entity.width)?;
		write_u8(&mut buffer, entity.height)?;
		write_u8(&mut buffer, entity.speed)?;
		write_u8(&mut buffer, entity.strength)?;
		write_u8(&mut buffer, entity.luck)?;
	}

	let offset_triggers: u32 = buffer.len() as u32;
	for trigger in &compiled.triggers {
		write_u8(&mut buffer, trigger.kind)?;
		write_u8(&mut buffer, trigger.gravity_multiplier)?;
		write_u16(&mut buffer, trigger.left)?;
		write_u16(&mut buffer, trigger.top)?;
		write_u16(&mut buffer, trigger.width)?;
		write_u16(&mut buffer, trigger.height)?;
		write_u16(&mut buffer, trigger.p0)?;
		write_u16(&mut buffer, trigger.p1)?;
	}

	let offset_tiles: u32 = buffer.len() as u32;
	buffer.extend_from_slice(&compiled.tiles);

	// patch header last
	let mut header = compiled.header;

	let hdr_probe = build_header_bytes(&header)?;
	let header_size: usize = hdr_probe.len();

	header.offset_layers = offset_layers;
	header.offset_entities = offset_entities;
	header.offset_triggers = offset_triggers;
	header.offset_tiles = offset_tiles;

	let hdr_bytes = build_header_bytes(&header)?;
	if hdr_bytes.len() != header_size {
		return Err(format!(
			"header_size mismatch: header.header_size={} but write_header produced {} bytes",
			header_size,
			hdr_bytes.len()
		));
	}
	buffer[0..header_size].copy_from_slice(&hdr_bytes);

	let expected_entities_bytes = (compiled.entities.len() as u32) * EntityRuntime::BYTE_SIZE;
	let actual_entities_bytes = offset_triggers - offset_entities;
	if actual_entities_bytes != expected_entities_bytes {
		return Err(format!("entity bytes mismatch: expected {} got {}", expected_entities_bytes, actual_entities_bytes));
	}

	println!(
		"entity_count={} entity_bytes={} per_entity={}",
		compiled.entities.len(),
		actual_entities_bytes,
		actual_entities_bytes / (compiled.entities.len() as u32)
	);

	return Ok(buffer);
}

fn write_header(buffer: &mut Vec<u8>, h: &FileHeader) -> Result<(), String> {
	buffer.extend_from_slice(&h.magic);
	write_u16(buffer, h.version)?;
	write_u16(buffer, h.header_size)?;
	write_u16(buffer, h.width)?;
	write_u16(buffer, h.height)?;
	write_u16(buffer, h.tile_width)?;
	write_u16(buffer, h.tile_height)?;
	write_u8(buffer, h.layer_count)?;
	write_u16(buffer, h.entity_count)?;
	write_u16(buffer, h.trigger_count)?;
	write_i16(buffer, h.gravity_fixed)?;
	write_u8(buffer, h.background_id)?;
	write_u8(buffer, h.gravity_multiplier)?;
	write_u16(buffer, h.reserved1)?;
	write_u32(buffer, h.tiles_per_layer)?;
	write_u32(buffer, h.tile_count_total)?;
	write_u32(buffer, h.offset_layers)?;
	write_u32(buffer, h.offset_entities)?;
	write_u32(buffer, h.offset_triggers)?;
	write_u32(buffer, h.offset_tiles)?;
	return Ok(());
}

fn write_u8(buffer: &mut Vec<u8>, value: u8) -> Result<(), String> {
	buffer.push(value);
	return Ok(());
}

fn write_u16(buffer: &mut Vec<u8>, value: u16) -> Result<(), String> {
	buffer.extend_from_slice(&value.to_le_bytes());
	return Ok(());
}

fn write_i16(buffer: &mut Vec<u8>, value: i16) -> Result<(), String> {
	buffer.extend_from_slice(&value.to_le_bytes());
	return Ok(());
}

fn write_u32(buffer: &mut Vec<u8>, value: u32) -> Result<(), String> {
	buffer.extend_from_slice(&value.to_le_bytes());
	return Ok(());
}
