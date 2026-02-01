use crate::runtime::assets::get_messages_root;
use std::{fs, path::PathBuf};

pub struct MessageTable {
	by_id: Vec<String>,
}

impl MessageTable {
	pub fn load(language_code: &str) -> Result<MessageTable, String> {
		let path: PathBuf = get_messages_root().join(format!("messages.{}.txt", language_code));
		let text: String = fs::read_to_string(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
		return MessageTable::parse(text.as_str());
	}

	pub fn get(&self, id: u16) -> &str {
		let index: usize = id as usize;
		if index >= self.by_id.len() {
			return "";
		}
		return self.by_id[index].as_str();
	}

	fn parse(text: &str) -> Result<MessageTable, String> {
		let mut max_id: u16 = 0;

		for raw_line in text.lines() {
			let line: &str = raw_line.trim();
			if line.is_empty() || line.starts_with('#') {
				continue;
			}
			let Some(eq_index) = line.find('=') else {
				continue;
			};
			let id_str: &str = line[..eq_index].trim();
			if let Ok(id) = id_str.parse::<u16>() {
				if id > max_id {
					max_id = id;
				}
			}
		}

		let mut by_id: Vec<String> = Vec::new();
		by_id.resize_with((max_id as usize) + 1, || String::new());

		for (line_index, raw_line) in text.lines().enumerate() {
			let line_number: usize = line_index + 1;
			let line: &str = raw_line.trim();

			if line.is_empty() || line.starts_with('#') {
				continue;
			}

			let Some(eq_index) = line.find('=') else {
				return Err(format!("invalid line {}: missing '='", line_number));
			};

			let id_str: &str = line[..eq_index].trim();
			let value: &str = line[eq_index + 1..].trim();

			let id: u16 = id_str.parse::<u16>().map_err(|_| format!("invalid id at line {}", line_number))?;
			by_id[id as usize] = value.to_string();
		}

		return Ok(MessageTable { by_id });
	}
}
