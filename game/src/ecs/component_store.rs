pub struct ComponentStore<T> {
	data: Vec<Option<T>>,
}

impl<T> ComponentStore<T> {
	pub fn new() -> Self {
		return Self { data: Vec::new() };
	}

	#[inline(always)]
	pub fn has(&self, id: u32) -> bool {
		let idx: usize = id as usize;
		if idx >= self.data.len() {
			return false;
		}
		return self.data[idx].is_some();
	}

	#[inline(always)]
	pub fn len(&self) -> usize {
		return self.data.len();
	}

	#[inline]
	pub fn keys(&self) -> impl Iterator<Item = u32> {
		return self
			.data
			.iter()
			.enumerate()
			.filter_map(|(idx, opt)| if opt.is_some() { Some(idx as u32) } else { None });
	}

	#[inline]
	pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
		return self.data.iter().enumerate().filter_map(|(idx, opt)| match opt {
			Some(value) => Some((idx as u32, value)),
			None => None,
		});
	}

	#[inline]
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut T)> {
		return self.data.iter_mut().enumerate().filter_map(|(idx, opt)| match opt {
			Some(value) => Some((idx as u32, value)),
			None => None,
		});
	}

	#[inline(always)]
	pub fn push(&mut self, id: u32, value: T) {
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
	#[allow(dead_code)]
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
