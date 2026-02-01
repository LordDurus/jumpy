#![cfg(feature = "gba")]

pub fn get_asset_root() -> &'static str {
	return "audio";
}

pub fn get_gfx_root() -> &'static str {
	return "assets";
}

pub fn get_audio_root() -> &'static str {
	return "assets/audio";
}

pub fn get_messages_root() -> &'static str {
	return "assets/messages";
}

pub fn get_books_root() -> &'static str {
	return "assets/books";
}

pub fn get_font_path() -> &'static str {
	return "assets/fonts";
}
