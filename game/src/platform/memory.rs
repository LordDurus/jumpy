#![allow(unused_macros)]

#[cfg(feature = "gba")]
macro_rules! fast_fn {
	($item:item) => {
		#[unsafe(link_section = ".iwram.text")]
		$item
	};
}

#[cfg(not(feature = "gba"))]
macro_rules! fast_fn {
	($item:item) => {
		$item
	};
}

#[cfg(feature = "gba")]
macro_rules! fast_data {
	($item:item) => {
		#[link_section = ".iwram.data"]
		$item
	};
}

#[cfg(not(feature = "gba"))]
macro_rules! fast_data {
	($item:item) => {
		$item
	};
}

// pub(crate) use fast_data;
pub(crate) use fast_fn;
