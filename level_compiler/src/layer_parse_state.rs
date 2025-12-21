use crate::source::*;

pub struct LayerParseState {
	pub name: String,
	pub collision: bool,
	pub rows: Vec<String>,
}

impl LayerParseState {
	pub fn new() -> LayerParseState {
		return LayerParseState {
			name: String::new(),
			collision: false,
			rows: Vec::new(),
		};
	}

	pub fn clear(&mut self) {
		self.name = String::new();
		self.collision = false;
		self.rows.clear();
	}

	pub fn to_layer_source(&mut self, line_number: usize) -> Result<LayerSource, String> {
		if self.name.is_empty() {
			return Err(format!("layer missing name at line {}", line_number));
		}

		let layer = LayerSource {
			name: self.name.clone(),
			collision: self.collision,
			rows: self.rows.clone(),
		};

		return Ok(layer);
	}
}
