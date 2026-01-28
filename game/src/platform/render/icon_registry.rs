#[repr(u16)]
pub enum IconId {
	TomSawyer = 1,
}

pub struct IconSprite {
	pub atlas_id: u8,
	pub tile_index: u16,
	pub width_tiles: u8,
	pub height_tiles: u8,
}

pub fn resolve_icon_sprite(icon_id: u16) -> Option<IconSprite> {
	match icon_id {
		1 => Some(IconSprite {
			atlas_id: 0,
			tile_index: 0,
			width_tiles: 1,
			height_tiles: 1,
		}), // tom sawyer
		_ => None,
	}
}
