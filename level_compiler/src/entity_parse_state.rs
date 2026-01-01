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
	pub range_min: i32,
	pub range_max: i32,
	pub health_regen_rate: i32,
	pub invulnerability_time: i32,
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
			range_min: 0,
			range_max: 0,
			health_regen_rate: 0,
			invulnerability_time: 0,
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
		self.width = 1.0;
		self.height = 1.0;
		self.speed = 0;
		self.strength = 0;
		self.luck = 0;
		self.range_min = 0;
		self.range_max = 0;
		self.health_regen_rate = 0;
		self.invulnerability_time = 0;
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

			// tile units
			width: self.width,
			height: self.height,

			// stats
			speed: self.speed as u8,
			strength: self.strength as u8,
			luck: self.luck as u8,

			// generic min/max (tiles)
			range_min: self.range_min,
			range_max: self.range_max,

			health_regen_rate: self.health_regen_rate as i16,
			invulnerability_time: self.invulnerability_time as i16,
		};

		return Ok(e);
	}
}
