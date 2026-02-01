pub type BookId = u16;
pub type BookSlug<'a> = &'a str;

pub mod reader;
pub mod reading_state;

#[cfg(feature = "pc")]
pub mod reader_pc;

#[cfg(feature = "gba")]
pub mod reader_gba;

// ---- platform selection ----

#[cfg(feature = "pc")]
pub type ActiveBookTextSource = reader_pc::PcBookTextSource;

#[cfg(feature = "gba")]
pub type ActiveBookTextSource = reader_gba::GbaBookTextSource;

// ---- unified reader type ----
pub type ActiveBookReader = reader::BookReader<ActiveBookTextSource>;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Book {
	pub book_id: BookId,
	pub current_page: u16,
	pub total_pages: u16,
}
