use crate::source::*;

pub struct TriggerParseState {
	pub kind: Option<TriggerKindSource>,
	pub top: i32,
	pub left: i32,
	pub width: f32,
	pub height: f32,
	pub icon_id: i32,
}

impl TriggerParseState {
	pub fn new() -> TriggerParseState {
		return TriggerParseState {
			kind: None,
			top: 0,
			left: 0,
			width: 1.0,
			height: 1.0,
			icon_id: 0,
		};
	}

	pub fn clear(&mut self) {
		self.kind = None;
		self.top = 0;
		self.left = 0;
		self.width = 1.0;
		self.height = 1.0;
		self.icon_id = 0;
	}

	pub fn to_trigger_source(&mut self, line_number: usize) -> Result<TriggerSource, String> {
		let kind = match self.kind.take() {
			Some(k) => k,
			None => {
				return Err(format!("trigger body closed without kind at line {}", line_number));
			}
		};

		let t = TriggerSource {
			top: self.top,
			left: self.left,
			width: self.width,
			height: self.height,
			icon_id: self.icon_id,
			kind,
		};
		return Ok(t);
	}
}
