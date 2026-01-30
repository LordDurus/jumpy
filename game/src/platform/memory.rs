// platform/mem.rs

#[cfg(feature = "gba")]
macro_rules! fast_fn {
	($item:item) => {
		#[link_section = ".iwram"]
		$item
	};
}

#[cfg(not(feature = "gba"))]
macro_rules! fast_fn {
	($item:item) => {
		$item
	};
}

pub(crate) use fast_fn;

#[cfg(feature = "gba")]
macro_rules! fast_data {
	($item:item) => {
		#[link_section = ".iwram"]
		$item
	};
}

#[cfg(not(feature = "gba"))]
macro_rules! fast_data {
	($item:item) => {
		$item
	};
}

pub(crate) use fast_data;
