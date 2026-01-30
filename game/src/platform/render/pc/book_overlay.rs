use crate::game::game_session::GameSession;

use sdl2::rect::Rect;

use super::{BOOK_BAR_COLOR, BOOK_BAR_TEXT_TOP_OFFSET_PIXELS, BOOK_DIVIDER_COLOR, BOOK_FOOTER_HEIGHT_PIXELS, BOOK_HEADER_HEIGHT_PIXELS, BOOK_PANEL_COLOR, PcRenderer};

impl PcRenderer {
	pub fn copy_book_page_to_clipboard(&self, text: &str) {
		let clipboard = self.video.clipboard();
		let _ = clipboard.set_clipboard_text(text);
		return;
	}

	fn draw_book_footer(&mut self, panel_left: i32, panel_top: i32, panel_width_pixels: u32, panel_height_pixels: u32) {
		let padding_pixels: i32 = 16;

		let footer_left: i32 = panel_left;
		let top: i32 = panel_top + (panel_height_pixels as i32) - BOOK_FOOTER_HEIGHT_PIXELS;
		let footer_width_pixels: u32 = panel_width_pixels;

		self.canvas.set_draw_color(BOOK_BAR_COLOR);
		let _ = self
			.canvas
			.fill_rect(Rect::new(footer_left, top, footer_width_pixels, BOOK_FOOTER_HEIGHT_PIXELS as u32));

		self.canvas.set_draw_color(BOOK_DIVIDER_COLOR);
		let _ = self.canvas.draw_line((footer_left, top), (footer_left + (footer_width_pixels as i32), top));

		let text_top: i32 = top + BOOK_BAR_TEXT_TOP_OFFSET_PIXELS;

		let left_left: i32 = footer_left + padding_pixels;
		let left_text: &str = "X: close";

		let right_text: &str = "B: copy page";
		let right_left: i32 = footer_left + (footer_width_pixels as i32) - padding_pixels - (right_text.len() as i32 * 8);

		self.draw_book_text_line(left_left, text_top, left_text);
		self.draw_book_text_line(right_left, text_top, right_text);

		return;
	}

	fn draw_book_header(&mut self, panel_left: i32, panel_top: i32, panel_width_pixels: u32) {
		let padding_pixels: i32 = 16;

		let header_left: i32 = panel_left;
		let top: i32 = panel_top;
		let header_width_pixels: u32 = panel_width_pixels;

		self.canvas.set_draw_color(BOOK_BAR_COLOR);
		let _ = self
			.canvas
			.fill_rect(Rect::new(header_left, top, header_width_pixels, BOOK_HEADER_HEIGHT_PIXELS as u32));

		self.canvas.set_draw_color(BOOK_DIVIDER_COLOR);
		let y = top + (BOOK_HEADER_HEIGHT_PIXELS - 1);
		let _ = self.canvas.draw_line((header_left, y), (header_left + (header_width_pixels as i32), y));

		let text_top: i32 = top + BOOK_BAR_TEXT_TOP_OFFSET_PIXELS;
		let text_left: i32 = header_left + padding_pixels;

		self.draw_book_text_line(text_left, text_top, "library");

		return;
	}

	fn draw_book_text_line(&mut self, left: i32, top: i32, text: &str) {
		let surface = self.font.render(text).blended(sdl2::pixels::Color::RGBA(240, 240, 255, 255)).unwrap();
		let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();

		let width_pixels: u32 = surface.width();
		let height_pixels: u32 = surface.height();

		let src = Rect::new(0, 0, width_pixels, height_pixels);
		let dst = Rect::new(left, top, width_pixels, height_pixels);

		let _ = self.canvas.copy(&texture, src, dst);

		return;
	}

	pub fn draw_book_overlay(&mut self, session: &GameSession) {
		let state = &session.book_reading;
		if !state.is_open {
			return;
		}

		let (screen_width_pixels, screen_height_pixels) = self.screen_size_pixels();

		let panel_width_pixels: u32 = (screen_width_pixels as f32 * 0.90) as u32;
		let panel_height_pixels: u32 = (screen_height_pixels as f32 * 0.90) as u32;

		let panel_left: i32 = ((screen_width_pixels - panel_width_pixels) / 2) as i32;
		let panel_top: i32 = ((screen_height_pixels - panel_height_pixels) / 2) as i32;

		self.canvas.set_draw_color(BOOK_PANEL_COLOR);
		let _ = self.canvas.fill_rect(Rect::new(panel_left, panel_top, panel_width_pixels, panel_height_pixels));

		self.draw_book_header(panel_left, panel_top, panel_width_pixels);
		self.draw_book_footer(panel_left, panel_top, panel_width_pixels, panel_height_pixels);

		let text_left: i32 = panel_left + 20;
		let text_top: i32 = panel_top + BOOK_HEADER_HEIGHT_PIXELS + 12;

		let mut y: i32 = text_top;
		for line in state.page_text.lines() {
			self.draw_book_text_line(text_left, y, line);
			y += 14;
		}

		return;
	}
}
