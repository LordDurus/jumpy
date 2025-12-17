use crate::runtime::*;

pub fn serialize_level(compiled: &CompiledLevel) -> Result<Vec<u8>, String> {
    let mut buffer = Vec::new();

    let header_size = std::mem::size_of::<FileHeader>() as u32;
    let layers_size = (compiled.layers.len() * std::mem::size_of::<LayerRuntime>()) as u32;
    let entities_size = (compiled.entities.len() * std::mem::size_of::<EntityRuntime>()) as u32;
    let triggers_size = (compiled.triggers.len() * std::mem::size_of::<TriggerRuntime>()) as u32;
    let tiles_size = compiled.tiles.len() as u32;

    let offset_layers = header_size;
    let offset_entities = offset_layers + layers_size;
    let offset_triggers = offset_entities + entities_size;
    let offset_tiles = offset_triggers + triggers_size;

    let mut header = compiled.header;
    header.offset_layers = offset_layers;
    header.offset_entities = offset_entities;
    header.offset_triggers = offset_triggers;
    header.offset_tiles = offset_tiles;

    write_header(&mut buffer, &header)?;

    for layer in &compiled.layers {
        write_u8(&mut buffer, layer.collision)?;
        write_u8(&mut buffer, layer.gravity_multiplier)?;
        write_u8(&mut buffer, layer.reserved1)?;
        write_u8(&mut buffer, layer.reserved2)?;
    }

    for e in &compiled.entities {
        write_u8(&mut buffer, e.kind)?;
        write_u8(&mut buffer, e.gravity_multiplier)?;
        write_u16(&mut buffer, e.x)?;
        write_u16(&mut buffer, e.y)?;
        write_i16(&mut buffer, e.a)?;
        write_i16(&mut buffer, e.b)?;
        write_u16(&mut buffer, e.extra_id)?;
    }

    for t in &compiled.triggers {
        write_u8(&mut buffer, t.kind)?;
        write_u8(&mut buffer, t.gravity_multiplier)?;
        write_u16(&mut buffer, t.x)?;
        write_u16(&mut buffer, t.y)?;
        write_u16(&mut buffer, t.width)?;
        write_u16(&mut buffer, t.height)?;
        write_u16(&mut buffer, t.p0)?;
        write_u16(&mut buffer, t.p1)?;
    }

    buffer.extend_from_slice(&compiled.tiles);

    let expected_size = offset_tiles + tiles_size;
    if buffer.len() as u32 != expected_size {
        return Err(format!(
            "serialize_level: size mismatch, expected {}, got {}",
            expected_size,
            buffer.len()
        ));
    }

    return Ok(buffer);
}

fn write_header(buffer: &mut Vec<u8>, h: &FileHeader) -> Result<(), String> {
    buffer.extend_from_slice(&h.magic);
    write_u16(buffer, h.version)?;
    write_u16(buffer, h.header_size)?;
    write_u16(buffer, h.width)?;
    write_u16(buffer, h.height)?;
    write_u8(buffer, h.tile_size)?;
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
