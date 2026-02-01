extern crate alloc;
use alloc::string::String;

use super::reader::BookTextSource;

pub struct GbaBookTextSource;

impl GbaBookTextSource {
	pub fn new() -> GbaBookTextSource {
		return GbaBookTextSource;
	}
}

impl BookTextSource for GbaBookTextSource {
	fn load_book_text(&self, _book_slug: &str) -> Result<String, String> {
		return Err(String::from("book text not implemented on gba yet"));
	}
}
