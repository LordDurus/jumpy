use crate::assets::get_books_root;
use std::{fs, path::PathBuf};

pub struct BookReader;

impl BookReader {
	pub fn load_book_text(book_slug: &str) -> Result<String, String> {
		let path: PathBuf = Self::book_path(book_slug);
		let raw: String = fs::read_to_string(&path).map_err(|e| format!("failed to read book {:?}: {}", path, e))?;

		let cleaned: String = strip_gutenberg_header_footer(&raw);
		return Ok(cleaned);
	}

	fn book_path(book_slug: &str) -> PathBuf {
		// matches your level compiler root scheme
		let path = get_books_root().join(format!("{}.txt", book_slug));
		return path;
	}

	pub fn page_text(full_text: &str, page_index: u16, chars_per_page: usize) -> String {
		let start: usize = (page_index as usize) * chars_per_page;
		if start >= full_text.len() {
			return "(end)".to_string();
		}

		let end: usize = (start + chars_per_page).min(full_text.len());
		let slice: &str = &full_text[start..end];
		return slice.to_string();
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
