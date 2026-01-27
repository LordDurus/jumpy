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
