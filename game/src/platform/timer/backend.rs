pub trait TimerBackend {
	fn now_ticks(&self) -> u64;
	fn ticks_per_second(&self) -> u64;
	fn sleep_ms(&self, ms: u32);
}
