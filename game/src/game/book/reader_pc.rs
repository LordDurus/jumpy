use super::reader::BookTextSource;
use crate::assets::get_books_root;
use std::{fs, path::PathBuf};

pub struct PcBookTextSource;

impl PcBookTextSource {
	pub fn new() -> PcBookTextSource {
		return PcBookTextSource;
	}

	fn book_path(book_slug: &str) -> PathBuf {
		let base_folder = get_books_root();
		return base_folder.join(format!("{}.txt", book_slug));
	}
}

impl BookTextSource for PcBookTextSource {
	fn load_book_text(&self, book_slug: &str) -> Result<String, String> {
		let path: PathBuf = Self::book_path(book_slug);
		let text: String = fs::read_to_string(&path).map_err(|e| format!("failed to read book {:?}: {}", path, e))?;
		return Ok(text);
	}
}
