pub const ICON_FRAME_WIDTH_PIXELS: u32 = 32;
pub const ICON_FRAME_HEIGHT_PIXELS: u32 = 32;

#[derive(Copy, Clone, Debug)]
pub struct IconDef {
	pub frame_count: u8,          // 1 means static
	pub frame_duration_ticks: u8, // 0 means no animation
}

pub fn resolve_icon(icon_id: u16) -> Option<IconDef> {
	match icon_id {
		// row 1: book (static)
		1 => Some(IconDef {
			frame_count: 1,
			frame_duration_ticks: 0,
		}),
		// row 2: gold coin (animated strip)
		2 => Some(IconDef {
			frame_count: 14,
			frame_duration_ticks: 3,
		}),
		// row 3: silver coin (animated strip)
		3 => Some(IconDef {
			frame_count: 14,
			frame_duration_ticks: 3,
		}),
		_ => None,
	}
}

// helper: compute source rect in pixels for a given icon_id + frame_index
pub fn get_icon_src_rect_pixels(icon_id: u16, frame_index: u16) -> (i32, i32, u32, u32) {
	let left_pixels: i32 = (frame_index as u32 * ICON_FRAME_WIDTH_PIXELS) as i32;
	let top_pixels: i32 = (icon_id as u32 * ICON_FRAME_HEIGHT_PIXELS) as i32;
	return (left_pixels, top_pixels, ICON_FRAME_WIDTH_PIXELS, ICON_FRAME_HEIGHT_PIXELS);
}
