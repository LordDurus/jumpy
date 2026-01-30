use sdl2::pixels::Color;

pub const BACKGROUND_ID_LIBRARY_STONE: u8 = 1;
pub const BACKGROUND_PARALLAX_FOREST: u8 = 2;

pub(crate) const BOOK_PANEL_COLOR: Color = Color::RGBA(20, 20, 28, 255);
pub(crate) const BOOK_BAR_COLOR: Color = Color::RGBA(28, 28, 40, 235);
pub(crate) const BOOK_DIVIDER_COLOR: Color = Color::RGBA(60, 60, 80, 110);
pub(crate) const BOOK_HEADER_HEIGHT_PIXELS: i32 = 34;
pub(crate) const BOOK_FOOTER_HEIGHT_PIXELS: i32 = 34;
pub(crate) const BOOK_BAR_TEXT_TOP_OFFSET_PIXELS: i32 = 8;

mod book_overlay;
mod draw;
mod platform_tiles;
mod renderer;
mod window;

pub use renderer::PcRenderer;
