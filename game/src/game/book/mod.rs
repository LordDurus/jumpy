pub type BookId = u16;
pub type BookSlug<'a> = &'a str;

pub mod reader;

#[cfg(feature = "pc")]
pub mod reader_pc;

#[cfg(feature = "gba")]
pub mod book_reader_gba;

// ---- platform selection ----

#[cfg(feature = "pc")]
pub type ActiveBookTextSource = reader_pc::PcBookTextSource;

#[cfg(feature = "gba")]
pub type ActiveBookTextSource = book_reader_gba::GbaBookTextSource;

// ---- unified reader type ----

pub type ActiveBookReader = reader::BookReader<ActiveBookTextSource>;
