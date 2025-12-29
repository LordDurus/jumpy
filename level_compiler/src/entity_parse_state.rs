use crate::source::{EntityKindSource, EntitySource};
pub struct EntityParseState {
	pub kind: Option<EntityKindSource>,
	pub top: i32,
	pub left: i32,
	pub render_style: u8,
	pub gravity_multiplier: f32,
	pub jump_multiplier: f32,
	pub attack_power: i32,
	pub hit_points: i32,
	pub width: f32,
	pub height: f32,
	pub speed: i32,
	pub strength: i32,
	pub luck: i32,
}

impl EntityParseState {
	pub fn new() -> EntityParseState {
		return EntityParseState {
			kind: None,
			top: 0,
			left: 0,
			render_style: 0,
			gravity_multiplier: 1.0,
			jump_multiplier: 1.0,
			attack_power: 1,
			hit_points: 1,
			width: 1.0,
			height: 1.0,
			speed: 0,
			strength: 0,
			luck: 0,
		};
	}

	pub fn clear(&mut self) {
		self.kind = None;
		self.top = 0;
		self.left = 0;
		self.render_style = 0;
		self.gravity_multiplier = 1.0;
		self.jump_multiplier = 1.0;
		self.attack_power = 1;
		self.hit_points = 1;
	}

	pub fn to_entity_source(&mut self, line_number: usize) -> Result<EntitySource, String> {
		let kind = match self.kind.take() {
			Some(k) => k,
			None => {
				return Err(format!("entity body closed without kind at line {}", line_number));
			}
		};

		let e = EntitySource {
			top: self.top,
			left: self.left,
			render_style: self.render_style,
			jump_multiplier: self.jump_multiplier,
			attack_power: self.attack_power,
			hit_points: self.hit_points,
			gravity_multiplier: self.gravity_multiplier,
			kind,
			// defaults (tile units)
			width: 1.0,
			height: 1.0,

			// defaults (stats)
			speed: 0,
			strength: 0,
			luck: 0,
		};
		return Ok(e);
	}
}
