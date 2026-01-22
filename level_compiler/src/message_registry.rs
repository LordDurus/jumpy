use std::{collections::HashMap, fs};

pub struct MessageRegistry {
	key_to_id: HashMap<String, u16>,
}

impl MessageRegistry {
	pub fn load_from_file(path: &str) -> Result<MessageRegistry, String> {
		let text: String = fs::read_to_string(path).map_err(|e| e.to_string())?;
		let mut map: HashMap<String, u16> = HashMap::new();

		for (line_index, raw_line) in text.lines().enumerate() {
			let line_number: usize = line_index + 1;

			let line: &str = raw_line.trim();
			if line.is_empty() || line.starts_with('#') {
				continue;
			}

			let Some(eq_index) = line.find('=') else {
				return Err(format!("{}:{} invalid line (missing '='): {}", path, line_number, line));
			};

			let id_str: &str = line[..eq_index].trim();
			let key: &str = line[eq_index + 1..].trim();

			let id: u16 = id_str.parse::<u16>().map_err(|_| format!("{}:{} invalid id: {}", path, line_number, id_str))?;

			if map.contains_key(key) {
				return Err(format!("{}:{} duplicate key: {}", path, line_number, key));
			}

			map.insert(key.to_string(), id);
		}

		return Ok(MessageRegistry { key_to_id: map });
	}

	pub fn resolve_message_id(&self, key: &str) -> Result<u16, String> {
		let Some(id) = self.key_to_id.get(key) else {
			return Err(format!("unknown message key: {}", key));
		};
		return Ok(*id);
	}
}
