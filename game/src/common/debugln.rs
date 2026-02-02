#[cfg(feature = "gba")]
#[macro_export]
macro_rules! debugln {
	($($arg:tt)*) => {
		agb::println!($($arg)*);
	};
}

#[cfg(not(feature = "gba"))]
#[macro_export]
macro_rules! debugln {
	($($arg:tt)*) => {
		println!($($arg)*);
	};
}
