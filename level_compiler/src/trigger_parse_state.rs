use crate::source::*;

pub struct TriggerParseState {
	pub kind: Option<TriggerKindSource>,
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

impl TriggerParseState {
	pub fn new() -> TriggerParseState {
		return TriggerParseState {
			kind: None,
			x: 0,
			y: 0,
			width: 1,
			height: 1,
		};
	}

	pub fn clear(&mut self) {
		self.kind = None;
		self.x = 0;
		self.y = 0;
		self.width = 1;
		self.height = 1;
	}

	pub fn to_trigger_source(&mut self, line_number: usize) -> Result<TriggerSource, String> {
		let kind = match self.kind.take() {
			Some(k) => k,
			None => {
				return Err(format!("trigger body closed without kind at line {}", line_number));
			}
		};

		let t = TriggerSource {
			x: self.x,
			y: self.y,
			width: self.width,
			height: self.height,
			kind,
		};
		return Ok(t);
	}
}
