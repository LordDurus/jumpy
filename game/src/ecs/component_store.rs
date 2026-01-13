use crate::game::game_state::EntityId;

pub struct ComponentStore<T> {
	data: Vec<Option<T>>,
}

impl<T> ComponentStore<T> {
	pub fn new() -> Self {
		return Self { data: Vec::new() };
	}

	pub fn take(&mut self, id: EntityId) -> Option<T> {
		let index: usize = id as usize;
		if index >= self.data.len() {
			return None;
		}

		return self.data[index].take();
	}

	#[allow(dead_code)]
	#[inline(always)]
	pub fn has(&self, id: EntityId) -> bool {
		let idx: usize = id as usize;
		if idx >= self.data.len() {
			return false;
		}
		return self.data[idx].is_some();
	}

	#[allow(dead_code)]
	#[inline(always)]
	pub fn len(&self) -> usize {
		return self.data.len();
	}

	#[inline]
	pub fn keys(&self) -> impl Iterator<Item = EntityId> {
		return self
			.data
			.iter()
			.enumerate()
			.filter_map(|(idx, opt)| if opt.is_some() { Some(idx as EntityId) } else { None });
	}

	#[inline]
	pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
		return self.data.iter().enumerate().filter_map(|(index, opt)| match opt {
			Some(value) => Some((index as EntityId, value)),
			None => None,
		});
	}

	#[inline]
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
		return self.data.iter_mut().enumerate().filter_map(|(index, opt)| match opt {
			Some(value) => Some((index as EntityId, value)),
			None => None,
		});
	}

	/// Updates or insert an entry
	#[inline(always)]
	pub fn set(&mut self, id: EntityId, value: T) {
		let idx: usize = id as usize;
		if idx >= self.data.len() {
			self.data.resize_with(idx + 1, || None);
		}
		self.data[idx] = Some(value);
		return;
	}

	#[inline(always)]
	pub fn get(&self, id: EntityId) -> Option<&T> {
		let idx: usize = id as usize;
		if idx >= self.data.len() {
			return None;
		}
		return self.data[idx].as_ref();
	}

	#[inline(always)]
	#[allow(dead_code)]
	pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
		let idx: usize = id as usize;
		if idx >= self.data.len() {
			return None;
		}
		return self.data[idx].as_mut();
	}

	#[inline(always)]
	pub fn remove(&mut self, id: EntityId) {
		let idx: usize = id as usize;
		if idx < self.data.len() {
			self.data[idx] = None;
		}
		return;
	}
}
