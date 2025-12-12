use chrono::{DateTime, Utc};

/// Time abstraction to make services testable.
///
/// Never use `Utc::now()` directly in services - always inject Clock.
/// This allows tests to use FixedClock for deterministic behavior.
pub trait Clock: Send + Sync {
	fn now(&self) -> DateTime<Utc>;
}

// --------------------------------------------------------------------------------
// Concrete Implementations
// --------------------------------------------------------------------------------

/// System clock that returns actual current time
pub struct SystemClock;

impl SystemClock {
	pub fn new() -> Self {
		Self
	}
}

impl Default for SystemClock {
	fn default() -> Self {
		Self::new()
	}
}

impl Clock for SystemClock {
	fn now(&self) -> DateTime<Utc> {
		Utc::now()
	}
}

// --------------------------------------------------------------------------------
// Test Utilities
// --------------------------------------------------------------------------------

/// Fixed clock for testing - allows controlled time progression.
///
/// Note: Not marked with #[cfg(test)] so it's accessible in integration tests.
pub struct FixedClock {
	time: std::sync::Mutex<DateTime<Utc>>,
}

impl FixedClock {
	pub fn new(time: DateTime<Utc>) -> Self {
		Self {
			time: std::sync::Mutex::new(time),
		}
	}

	/// Advance the clock by the specified duration
	pub fn advance(&self, duration: chrono::Duration) {
		let mut time = self.time.lock().unwrap();
		*time += duration;
	}

	/// Set the clock to a specific time
	pub fn set(&self, new_time: DateTime<Utc>) {
		let mut time = self.time.lock().unwrap();
		*time = new_time;
	}
}

impl Clock for FixedClock {
	fn now(&self) -> DateTime<Utc> {
		*self.time.lock().unwrap()
	}
}
