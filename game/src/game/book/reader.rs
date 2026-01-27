use crate::{BookSlug, game::book::reading_state::BookReadingState};

#[derive(Debug, Clone, Copy)]
pub struct BookPage {
	pub page_index: u16,
	pub total_pages: u16,
}

pub trait BookTextSource {
	fn load_book_text(&self, book_slug: &str) -> Result<String, String>;
}

pub struct BookReader<S: BookTextSource> {
	source: S,
	lines_per_page: usize,
}

impl<S: BookTextSource> BookReader<S> {
	pub fn new(source: S, lines_per_page: usize) -> BookReader<S> {
		return BookReader { source, lines_per_page };
	}

	pub fn open_book(&self, state: &mut BookReadingState, book_slug: BookSlug, start_page: u16) -> Result<(), String> {
		let (page, text) = self.read_page(book_slug, start_page)?;

		println!("Open Book: book_slug={}", book_slug);

		state.is_open = true;
		state.book_slug = book_slug.to_string();
		state.page_index = page.page_index;
		state.total_pages = page.total_pages;
		state.page_text = text;

		return Ok(());
	}

	pub fn close_book(&self, state: &mut BookReadingState) {
		*state = BookReadingState::closed();
	}

	pub fn turn_book_page(&self, state: &mut BookReadingState, delta: i16) -> Result<(), String> {
		if !state.is_open {
			return Ok(()); // ignore silently
		}

		let mut new_page: i32 = state.page_index as i32 + delta as i32;

		if new_page < 0 {
			new_page = 0;
		}

		if new_page >= state.total_pages as i32 {
			new_page = (state.total_pages as i32) - 1;
		}

		if new_page as u16 == state.page_index {
			return Ok(()); // no-op
		}

		let (page, text) = self.read_page(&state.book_slug, new_page as u16)?;

		state.page_index = page.page_index;
		state.total_pages = page.total_pages;
		state.page_text = text;

		return Ok(());
	}

	pub fn read_page(&self, book_slug: &str, page_index: u16) -> Result<(BookPage, String), String> {
		let raw: String = self.source.load_book_text(book_slug)?;
		let cleaned: String = strip_gutenberg_header_footer(&raw);

		let lines: Vec<&str> = cleaned.lines().collect();
		let total_pages_usize: usize = (lines.len() + self.lines_per_page - 1) / self.lines_per_page;

		let total_pages: u16 = if total_pages_usize > (u16::MAX as usize) {
			u16::MAX
		} else {
			total_pages_usize as u16
		};

		let clamped_page: u16 = if total_pages == 0 {
			0
		} else if page_index >= total_pages {
			total_pages - 1
		} else {
			page_index
		};

		let start_line: usize = (clamped_page as usize) * self.lines_per_page;
		let end_line: usize = core::cmp::min(start_line + self.lines_per_page, lines.len());

		let mut out: String = String::new();
		for line in &lines[start_line..end_line] {
			out.push_str(line);
			out.push('\n');
		}

		let page = BookPage {
			page_index: clamped_page,
			total_pages,
		};

		return Ok((page, out));
	}
}

fn strip_gutenberg_header_footer(text: &str) -> String {
	let start_marker: &str = "*** START OF";
	let end_marker: &str = "*** END OF";

	let mut start_index: usize = 0;
	if let Some(pos) = text.find(start_marker) {
		if let Some(nl) = text[pos..].find('\n') {
			start_index = pos + nl + 1;
		}
	}

	let mut end_index: usize = text.len();
	if let Some(pos) = text.find(end_marker) {
		end_index = pos;
	}

	if start_index >= end_index {
		return text.to_string();
	}

	return text[start_index..end_index].to_string();
}
