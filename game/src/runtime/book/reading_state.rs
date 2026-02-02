#[cfg(feature = "gba")]
extern crate alloc;

#[cfg(feature = "gba")]
use alloc::string::String;
//use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

#[derive(Clone, Debug)]
pub struct BookReadingState {
	pub is_open: bool,
	pub book_slug: String,
	pub page_index: u16,
	pub total_pages: u16,
	pub page_text: String,
}

impl BookReadingState {
	pub fn closed() -> BookReadingState {
		return BookReadingState {
			is_open: false,
			book_slug: String::new(),
			page_index: 0,
			total_pages: 0,
			page_text: String::new(),
		};
	}
}
