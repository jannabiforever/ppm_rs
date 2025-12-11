use chrono::{DateTime, Utc};

/// Clock abstraction for testability
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

#[cfg(test)]
pub struct FixedClock {
	time: DateTime<Utc>,
}

#[cfg(test)]
impl FixedClock {
	pub fn new(time: DateTime<Utc>) -> Self {
		Self {
			time,
		}
	}

	pub fn set_time(&mut self, time: DateTime<Utc>) {
		self.time = time;
	}
}

#[cfg(test)]
impl Clock for FixedClock {
	fn now(&self) -> DateTime<Utc> {
		self.time
	}
}
