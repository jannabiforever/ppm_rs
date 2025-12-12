use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSession {
	pub id: String,
	pub start: DateTime<Utc>,
	pub end: DateTime<Utc>,
}

impl FocusSession {
	pub fn duration(&self) -> Duration {
		self.end - self.start
	}

	pub fn is_active(&self, now: DateTime<Utc>) -> bool {
		now >= self.start && now <= self.end
	}

	/// Generate a unique session ID
	pub fn generate_id() -> String {
		use std::time::{SystemTime, UNIX_EPOCH};

		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();

		format!("session_{}", timestamp)
	}
}
