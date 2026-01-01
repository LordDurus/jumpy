pub struct ComponentStore<T> {
	data: Vec<Option<T>>,
}

impl<T> ComponentStore<T> {
	pub fn new() -> Self {
		return Self { data: Vec::new() };
	}

	#[inline(always)]
	pub fn insert(&mut self, id: u32, value: T) {
		let idx: usize = id as usize;
		if idx >= self.data.len() {
			self.data.resize_with(idx + 1, || None);
		}
		self.data[idx] = Some(value);
		return;
	}

	#[inline(always)]
	pub fn get(&self, id: u32) -> Option<&T> {
		let idx: usize = id as usize;
		if idx >= self.data.len() {
			return None;
		}
		return self.data[idx].as_ref();
	}

	#[inline(always)]
	pub fn get_mut(&mut self, id: u32) -> Option<&mut T> {
		let idx: usize = id as usize;
		if idx >= self.data.len() {
			return None;
		}
		return self.data[idx].as_mut();
	}

	#[inline(always)]
	pub fn remove(&mut self, id: u32) {
		let idx: usize = id as usize;
		if idx < self.data.len() {
			self.data[idx] = None;
		}
		return;
	}
}
