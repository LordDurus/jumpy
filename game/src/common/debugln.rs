#[cfg(feature = "pc")]
#[macro_export]
macro_rules! debugln {
	($($arg:tt)*) => {{
		println!($($arg)*);
	}};
}

#[cfg(not(feature = "pc"))]
#[macro_export]
macro_rules! debugln {
	($($arg:tt)*) => {{
		// no-op on gba / constrained builds
	}};
}
